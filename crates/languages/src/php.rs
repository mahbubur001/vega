//! PHP, built in (VEGA). Ported from the PHP extension for Zed
//! (https://github.com/zed-extensions/php, Apache-2.0): its tasks and its
//! phpactor and Intelephense language servers. The grammar and queries are in
//! `crates/grammars/src/php`.

use anyhow::{Context as _, Result};
use async_trait::async_trait;
use futures::StreamExt as _;
use github_releases::{GitHubLspBinaryVersion, download_server_raw_binary, latest_github_release};
use gpui::AsyncApp;
use language::{LanguageServerName, LspAdapter, LspAdapterDelegate, LspInstaller, Toolchain};
use lsp::{LanguageServerBinary, Uri};
use node_runtime::{NodeRuntime, VersionStrategy};
use project::{ContextProviderWithTasks, lsp_store::language_server_settings};
use semver::Version;
use serde_json::json;
use std::{
    ffi::OsString,
    future::Future,
    path::{Path, PathBuf},
    sync::Arc,
};
use task::{TaskTemplate, TaskTemplates, VariableName};
use util::{ResultExt as _, fs::remove_matching, maybe};

/// phpunit and pest tasks, and running the selection with `php -r`.
pub(super) fn php_task_context() -> ContextProviderWithTasks {
    let symbol = VariableName::Symbol.template_value();
    let file = VariableName::File.template_value();
    let test_runner = |runner: &str, tag: &str| {
        let command = format!("./vendor/bin/{runner}");
        let tags = vec![tag.to_owned()];
        [
            TaskTemplate {
                label: format!("{runner} test {symbol}"),
                command: command.clone(),
                args: vec!["--filter".into(), format!("\"{symbol}\""), file.clone()],
                tags: tags.clone(),
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: format!("{runner} test {}", VariableName::Filename.template_value()),
                command: command.clone(),
                args: vec![file.clone()],
                tags: tags.clone(),
                ..TaskTemplate::default()
            },
            TaskTemplate {
                label: format!("{runner} run all tests"),
                command,
                tags,
                ..TaskTemplate::default()
            },
        ]
    };
    let mut templates = Vec::new();
    templates.extend(test_runner("phpunit", "phpunit-test"));
    templates.extend(test_runner("pest", "pest-test"));
    templates.push(TaskTemplate {
        label: format!(
            "execute selection {}",
            VariableName::SelectedText.template_value()
        ),
        command: "php".into(),
        args: vec!["-r".into(), VariableName::SelectedText.template_value()],
        ..TaskTemplate::default()
    });
    ContextProviderWithTasks::new(TaskTemplates(templates))
}

// ---- phpactor ------------------------------------------------------------------------------

/// phpactor (https://github.com/phpactor/phpactor): `phpactor` from `PATH`,
/// otherwise `phpactor.phar` from its latest GitHub release, run with `php`.
pub struct PhpactorLspAdapter;

impl PhpactorLspAdapter {
    const SERVER_NAME: LanguageServerName = LanguageServerName::new_static("phpactor");
    const PHAR: &str = "phpactor.phar";
    const VERSION_DIR_PREFIX: &str = "phpactor_";
}

async fn php_binary(delegate: &dyn LspAdapterDelegate) -> Result<PathBuf> {
    delegate
        .which("php".as_ref())
        .await
        .context("phpactor needs PHP: install it and make sure `php` is on your PATH")
}

fn phpactor_binary(php: PathBuf, phar: &Path) -> LanguageServerBinary {
    LanguageServerBinary {
        path: php,
        env: None,
        arguments: vec![phar.into(), "language-server".into()],
    }
}

impl LspInstaller for PhpactorLspAdapter {
    type BinaryVersion = GitHubLspBinaryVersion;

    async fn check_if_user_installed(
        &self,
        delegate: &Arc<dyn LspAdapterDelegate>,
        _: Option<Toolchain>,
        _: &AsyncApp,
    ) -> Option<LanguageServerBinary> {
        let path = delegate.which(Self::SERVER_NAME.as_ref()).await?;
        Some(LanguageServerBinary {
            path,
            env: None,
            arguments: vec!["language-server".into()],
        })
    }

    async fn fetch_latest_server_version(
        &self,
        delegate: &Arc<dyn LspAdapterDelegate>,
        pre_release: bool,
        _: &mut AsyncApp,
    ) -> Result<GitHubLspBinaryVersion> {
        let release = latest_github_release(
            "phpactor/phpactor",
            true,
            pre_release,
            delegate.http_client(),
        )
        .await?;
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == Self::PHAR)
            .with_context(|| format!("no {} in phpactor {}", Self::PHAR, release.tag_name))?;
        Ok(GitHubLspBinaryVersion {
            name: release.tag_name,
            url: asset.browser_download_url.clone(),
            digest: asset.digest.clone(),
        })
    }

    fn check_if_version_installed(
        &self,
        version: &GitHubLspBinaryVersion,
        container_dir: &PathBuf,
        delegate: &Arc<dyn LspAdapterDelegate>,
    ) -> impl Send + Future<Output = Option<LanguageServerBinary>> + use<> {
        let phar = container_dir
            .join(format!("{}{}", Self::VERSION_DIR_PREFIX, version.name))
            .join(Self::PHAR);
        let delegate = delegate.clone();
        async move {
            if !phar.is_file() {
                return None;
            }
            let php = php_binary(delegate.as_ref()).await.log_err()?;
            Some(phpactor_binary(php, &phar))
        }
    }

    fn fetch_server_binary(
        &self,
        version: GitHubLspBinaryVersion,
        container_dir: PathBuf,
        delegate: &Arc<dyn LspAdapterDelegate>,
    ) -> impl Send + Future<Output = Result<LanguageServerBinary>> + use<> {
        let delegate = delegate.clone();
        async move {
            let php = php_binary(delegate.as_ref()).await?;
            let version_dir =
                container_dir.join(format!("{}{}", Self::VERSION_DIR_PREFIX, version.name));
            let phar = version_dir.join(Self::PHAR);
            if !phar.is_file() {
                download_server_raw_binary(
                    delegate.background_executor(),
                    delegate.http_client(),
                    &version.url,
                    version.digest.as_deref(),
                    &version_dir,
                    Self::PHAR,
                )
                .await?;
                remove_matching(&container_dir, |entry| entry != version_dir).await;
            }
            Ok(phpactor_binary(php, &phar))
        }
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        maybe!(async {
            let mut newest: Option<PathBuf> = None;
            let mut entries = smol::fs::read_dir(&container_dir).await?;
            while let Some(entry) = entries.next().await {
                let path = entry?.path();
                let is_version_dir = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .is_some_and(|name| name.starts_with(Self::VERSION_DIR_PREFIX));
                if is_version_dir && path.join(Self::PHAR).is_file() {
                    newest = newest.max(Some(path));
                }
            }
            let phar = newest.context("no downloaded phpactor")?.join(Self::PHAR);
            anyhow::Ok(phpactor_binary(php_binary(delegate).await?, &phar))
        })
        .await
        .log_err()
    }
}

#[async_trait(?Send)]
impl LspAdapter for PhpactorLspAdapter {
    fn name(&self) -> LanguageServerName {
        Self::SERVER_NAME
    }
}

// ---- Intelephense --------------------------------------------------------------------------

const INTELEPHENSE_SERVER_PATH: &str = "node_modules/intelephense/lib/intelephense.js";

fn intelephense_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), "--stdio".into()]
}

/// Intelephense (https://intelephense.com), installed from npm.
pub struct IntelephenseLspAdapter {
    node: NodeRuntime,
}

impl IntelephenseLspAdapter {
    const SERVER_NAME: LanguageServerName = LanguageServerName::new_static("intelephense");
    const PACKAGE_NAME: &str = "intelephense";

    pub fn new(node: NodeRuntime) -> Self {
        Self { node }
    }
}

impl LspInstaller for IntelephenseLspAdapter {
    type BinaryVersion = Version;

    async fn check_if_user_installed(
        &self,
        delegate: &Arc<dyn LspAdapterDelegate>,
        _: Option<Toolchain>,
        _: &AsyncApp,
    ) -> Option<LanguageServerBinary> {
        let path = delegate.which(Self::SERVER_NAME.as_ref()).await?;
        Some(LanguageServerBinary {
            path,
            env: Some(delegate.shell_env().await),
            arguments: vec!["--stdio".into()],
        })
    }

    async fn fetch_latest_server_version(
        &self,
        _: &Arc<dyn LspAdapterDelegate>,
        _: bool,
        _: &mut AsyncApp,
    ) -> Result<Version> {
        self.node
            .npm_package_latest_version(Self::PACKAGE_NAME)
            .await
    }

    fn check_if_version_installed(
        &self,
        version: &Version,
        container_dir: &PathBuf,
        _: &Arc<dyn LspAdapterDelegate>,
    ) -> impl Send + Future<Output = Option<LanguageServerBinary>> + use<> {
        let node = self.node.clone();
        let version = version.clone();
        let container_dir = container_dir.clone();
        async move {
            let server_path = container_dir.join(INTELEPHENSE_SERVER_PATH);
            let should_install = node
                .should_install_npm_package(
                    Self::PACKAGE_NAME,
                    &server_path,
                    &container_dir,
                    VersionStrategy::Latest(&version),
                )
                .await;
            if should_install {
                return None;
            }
            Some(LanguageServerBinary {
                path: node.binary_path().await.ok()?,
                env: None,
                arguments: intelephense_arguments(&server_path),
            })
        }
    }

    fn fetch_server_binary(
        &self,
        _: Version,
        container_dir: PathBuf,
        _: &Arc<dyn LspAdapterDelegate>,
    ) -> impl Send + Future<Output = Result<LanguageServerBinary>> + use<> {
        let node = self.node.clone();
        async move {
            node.npm_install_latest_packages(&container_dir, &[Self::PACKAGE_NAME])
                .await?;
            Ok(LanguageServerBinary {
                path: node.binary_path().await?,
                env: None,
                arguments: intelephense_arguments(&container_dir.join(INTELEPHENSE_SERVER_PATH)),
            })
        }
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        maybe!(async {
            let server_path = container_dir.join(INTELEPHENSE_SERVER_PATH);
            anyhow::ensure!(server_path.exists(), "missing {server_path:?}");
            Ok(LanguageServerBinary {
                path: self.node.binary_path().await?,
                env: None,
                arguments: intelephense_arguments(&server_path),
            })
        })
        .await
        .log_err()
    }
}

#[async_trait(?Send)]
impl LspAdapter for IntelephenseLspAdapter {
    fn name(&self) -> LanguageServerName {
        Self::SERVER_NAME
    }

    /// The `lsp.intelephense.settings` from the settings files under an
    /// `intelephense` key, the way the PHP extension sent them (VEGA's PHP
    /// settings write them in that shape).
    async fn workspace_configuration(
        self: Arc<Self>,
        delegate: &Arc<dyn LspAdapterDelegate>,
        _: Option<Toolchain>,
        _: Option<Uri>,
        cx: &mut AsyncApp,
    ) -> Result<serde_json::Value> {
        let settings = cx.update(|cx| {
            language_server_settings(delegate.as_ref(), &self.name(), cx)
                .and_then(|settings| settings.settings.clone())
        });
        Ok(json!({ "intelephense": settings.unwrap_or_default() }))
    }
}

#[cfg(test)]
mod tests {
    use gpui::{AppContext as _, TestAppContext};
    use language::Buffer;

    #[gpui::test]
    async fn the_php_queries_load_and_highlight(cx: &mut TestAppContext) {
        let language = crate::language("php", tree_sitter_php::LANGUAGE_PHP.into());
        let text = "<?php\nfunction greet(string $name): string {\n    return \"Hi $name\";\n}\n";
        let buffer = cx.new(|cx| Buffer::local(text, cx).with_language(language.clone(), cx));
        buffer.condition(cx, |buffer, _| !buffer.is_parsing()).await;
        let outline = buffer.read_with(cx, |buffer, _| {
            buffer
                .snapshot()
                .outline(None)
                .items
                .into_iter()
                .map(|item| item.text)
                .collect::<Vec<_>>()
        });
        assert_eq!(outline, vec!["function greet"]);
    }
}
