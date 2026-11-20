//! JetBrains' phpstorm-stubs (Apache-2.0): PHP's built-in functions and
//! classes as PHP files, one folder per extension. Downloaded on request.

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context as _, anyhow};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use fs::{Fs, RemoveOptions, RenameOptions};
use futures::{StreamExt as _, io::BufReader};
use http_client::{AsyncBody, HttpClient};

const STUBS_URL: &str =
    "https://github.com/JetBrains/phpstorm-stubs/archive/refs/heads/master.tar.gz";

/// Where downloaded PHP stubs live (one folder per PHP extension).
pub fn stubs_dir() -> PathBuf {
    paths::data_dir().join("php-stubs")
}

/// `stubs_dir()` when the stubs have been downloaded.
pub async fn downloaded_stubs_dir(fs: &dyn Fs) -> Option<PathBuf> {
    let dir = stubs_dir();
    fs.is_dir(&dir).await.then_some(dir)
}

/// The extension folders of the downloaded stubs, by name.
pub async fn stub_folders(fs: &dyn Fs) -> Vec<(String, PathBuf)> {
    let mut folders = Vec::new();
    let Ok(mut entries) = fs.read_dir(&stubs_dir()).await else {
        return folders;
    };
    while let Some(Ok(path)) = entries.next().await {
        let Some(name) = path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
        else {
            continue;
        };
        // Repository folders that aren't extensions.
        if name.starts_with('.') || matches!(name.as_str(), "tests" | "meta" | "vendor") {
            continue;
        }
        if fs.is_dir(&path).await {
            folders.push((name, path));
        }
    }
    folders.sort_by_key(|(name, _)| name.to_lowercase());
    folders
}

/// Downloads JetBrains' phpstorm-stubs (Apache-2.0) into `stubs_dir()`.
pub async fn download_stubs(fs: Arc<dyn Fs>, http: Arc<dyn HttpClient>) -> anyhow::Result<()> {
    let target = stubs_dir();
    let staging = target.with_extension("download");
    fs.remove_dir(
        &staging,
        RemoveOptions {
            recursive: true,
            ignore_if_not_exists: true,
        },
    )
    .await?;
    fs.create_dir(&staging).await?;

    let mut response = http
        .get(STUBS_URL, AsyncBody::default(), true)
        .await
        .context("couldn't reach GitHub")?;
    if !response.status().is_success() {
        return Err(anyhow!("GitHub answered {}", response.status()));
    }
    let archive = Archive::new(GzipDecoder::new(BufReader::new(response.body_mut())));
    archive
        .unpack(&staging)
        .await
        .context("couldn't unpack the stubs")?;

    // The archive holds one folder (phpstorm-stubs-master); it becomes `stubs_dir()`.
    let mut entries = fs.read_dir(&staging).await?;
    let unpacked = entries
        .next()
        .await
        .transpose()?
        .ok_or_else(|| anyhow!("the stubs archive was empty"))?;
    fs.remove_dir(
        &target,
        RemoveOptions {
            recursive: true,
            ignore_if_not_exists: true,
        },
    )
    .await?;
    fs.rename(&unpacked, &target, RenameOptions::default())
        .await?;
    // phpactor's map of the old stubs.
    fs.remove_dir(
        &crate::config::phpactor_stub_cache_dir(&target),
        RemoveOptions {
            recursive: true,
            ignore_if_not_exists: true,
        },
    )
    .await?;
    fs.remove_dir(
        &staging,
        RemoveOptions {
            recursive: true,
            ignore_if_not_exists: true,
        },
    )
    .await?;
    Ok(())
}
