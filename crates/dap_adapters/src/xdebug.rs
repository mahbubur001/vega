//! Xdebug for PHP, built in (VEGA). Zed shipped this adapter until it moved
//! into the PHP extension (https://github.com/zed-extensions/php, Apache-2.0);
//! VEGA builds PHP in, so the adapter is back here under the extension's name,
//! "Xdebug", so existing debug scenarios keep working. `xdebug_schema.json` is
//! the extension's `debug_adapter_schemas/Xdebug.json`.
//!
//! The adapter is xdebug/vscode-php-debug, downloaded from its GitHub releases
//! and run with Node.js.

use std::{path::PathBuf, sync::OnceLock};

use collections::HashMap;

use adapters::latest_github_release;
use anyhow::{Context as _, bail};
use dap::adapters::{DebugTaskDefinition, TcpArguments};
use dap::{StartDebuggingRequestArguments, StartDebuggingRequestArgumentsRequest};
use gpui::{AsyncApp, SharedString};
use language::LanguageName;
use util::ResultExt as _;

use crate::*;

#[derive(Default)]
pub(crate) struct XdebugAdapter {
    checked: OnceLock<()>,
}

impl XdebugAdapter {
    const ADAPTER_NAME: &'static str = "Xdebug";
    const ADAPTER_PATH: &'static str = "extension/out/phpDebug.js";

    async fn fetch_latest_adapter_version(
        &self,
        delegate: &Arc<dyn DapDelegate>,
    ) -> Result<AdapterVersion> {
        let release = latest_github_release(
            "xdebug/vscode-php-debug",
            true,
            false,
            delegate.http_client(),
        )
        .await?;
        let asset_name = format!(
            "php-debug-{}.vsix",
            release.tag_name.trim_start_matches('v')
        );
        let url = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .with_context(|| format!("no asset found matching {asset_name:?}"))?
            .browser_download_url
            .clone();
        Ok(AdapterVersion {
            tag_name: release.tag_name,
            url,
        })
    }

    async fn installed_directory(&self) -> Result<PathBuf> {
        let adapter_dir = paths::debug_adapters_dir().join(self.name().as_ref());
        let prefix = format!("{}_", self.name());
        util::fs::find_file_name_in_dir(adapter_dir.as_path(), |file_name| {
            file_name.starts_with(&prefix)
        })
        .await
        .context("Couldn't find the Xdebug adapter; check your internet connection")
    }
}

#[async_trait(?Send)]
impl DebugAdapter for XdebugAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    fn adapter_language_name(&self) -> Option<LanguageName> {
        Some(SharedString::new_static("PHP").into())
    }

    fn dap_schema(&self) -> serde_json::Value {
        serde_json::from_str(include_str!("xdebug_schema.json"))
            .expect("xdebug_schema.json is valid JSON")
    }

    async fn request_kind(
        &self,
        _: &serde_json::Value,
    ) -> Result<StartDebuggingRequestArgumentsRequest> {
        Ok(StartDebuggingRequestArgumentsRequest::Launch)
    }

    async fn config_from_zed_format(&self, zed_scenario: ZedDebugConfig) -> Result<DebugScenario> {
        let config = match &zed_scenario.request {
            dap::DebugRequest::Attach(_) => bail!("Xdebug doesn't support attaching"),
            dap::DebugRequest::Launch(launch) => json!({
                "request": "launch",
                "program": launch.program,
                "cwd": launch.cwd,
                "args": launch.args,
                "env": launch.env_json(),
                "stopOnEntry": zed_scenario.stop_on_entry.unwrap_or_default(),
            }),
        };
        Ok(DebugScenario {
            adapter: zed_scenario.adapter,
            label: zed_scenario.label,
            build: None,
            config,
            tcp_connection: None,
        })
    }

    async fn get_binary(
        &self,
        delegate: &Arc<dyn DapDelegate>,
        task_definition: &DebugTaskDefinition,
        user_installed_path: Option<PathBuf>,
        user_args: Option<Vec<String>>,
        user_env: Option<HashMap<String, String>>,
        _: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        if user_installed_path.is_none() && self.checked.set(()).is_ok() {
            delegate.output_to_console(format!("Checking latest version of {}...", self.name()));
            if let Some(version) = self.fetch_latest_adapter_version(delegate).await.log_err() {
                adapters::download_adapter_from_github(
                    self.name(),
                    version,
                    adapters::DownloadedFileType::Vsix,
                    delegate.as_ref(),
                )
                .await?;
            }
        }

        let adapter_dir = match user_installed_path {
            Some(path) => path,
            None => self.installed_directory().await?,
        };
        let script = adapter_dir
            .join(Self::ADAPTER_PATH)
            .to_string_lossy()
            .into_owned();

        let tcp_connection = task_definition.tcp_connection.clone().unwrap_or_default();
        let (host, port, timeout) = crate::configure_tcp_connection(tcp_connection).await?;

        let mut configuration = task_definition.config.clone();
        if let Some(object) = configuration.as_object_mut() {
            object
                .entry("cwd")
                .or_insert_with(|| delegate.worktree_root_path().to_string_lossy().into());
        }

        let arguments = match user_args {
            Some(mut args) => {
                args.insert(0, script);
                args
            }
            None => vec![script, format!("--server={port}")],
        };

        Ok(DebugAdapterBinary {
            command: Some(
                delegate
                    .node_runtime()
                    .binary_path()
                    .await?
                    .to_string_lossy()
                    .into_owned(),
            ),
            arguments,
            connection: Some(TcpArguments {
                port,
                host,
                timeout,
            }),
            cwd: Some(delegate.worktree_root_path().to_path_buf()),
            envs: user_env.unwrap_or_default(),
            request_args: StartDebuggingRequestArguments {
                request: self.request_kind(&configuration).await?,
                configuration,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_schema_requires_a_launch_request() {
        let schema = XdebugAdapter::default().dap_schema();
        assert_eq!(schema["properties"]["request"]["enum"], json!(["launch"]));
    }
}
