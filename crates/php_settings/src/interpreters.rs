//! Finding PHP interpreters and asking them for their version and extensions.

use std::path::{Path, PathBuf};

use fs::Fs;
use futures::StreamExt as _;

const PHP_INFO: &str = r#"echo PHP_VERSION, "\n", implode("\n", get_loaded_extensions());"#;

/// An interpreter's version and loaded extensions.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PhpInfo {
    pub version: String,
    pub extensions: Vec<String>,
}

/// Runs `php` and asks for its version and extensions.
pub async fn php_info(php: &str) -> Option<PhpInfo> {
    let output = util::command::new_command(php)
        .args(["-r", PHP_INFO])
        .output()
        .await
        .ok()?;
    if !output.status.success() {
        return None;
    }
    parse_php_info(&String::from_utf8_lossy(&output.stdout))
}

/// Parses the output of `PHP_INFO`.
pub fn parse_php_info(output: &str) -> Option<PhpInfo> {
    let mut lines = output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty());
    let version = lines.next()?.to_string();
    if !version.starts_with(|c: char| c.is_ascii_digit()) {
        return None;
    }
    let mut extensions: Vec<String> = lines.map(str::to_string).collect();
    extensions.sort_by_key(|extension| extension.to_lowercase());
    extensions.dedup();
    Some(PhpInfo {
        version,
        extensions,
    })
}

/// `8.4.12` → `8.4`.
pub fn short_version(version: &str) -> String {
    version.split('.').take(2).collect::<Vec<_>>().join(".")
}

/// PHP interpreters installed in the usual places (Homebrew, Herd, MAMP,
/// the system), each with its version.
pub async fn find_interpreters(fs: &dyn Fs) -> Vec<(PathBuf, String)> {
    let home = util::paths::home_dir();
    let mut candidates: Vec<PathBuf> = vec![
        "/opt/homebrew/bin/php".into(),
        "/usr/local/bin/php".into(),
        "/usr/bin/php".into(),
    ];
    for versioned in [Path::new("/opt/homebrew/opt"), Path::new("/usr/local/opt")] {
        for name in dir_names(fs, versioned).await {
            if name.starts_with("php@") {
                candidates.push(versioned.join(name).join("bin/php"));
            }
        }
    }
    let herd = home.join("Library/Application Support/Herd/bin");
    for name in dir_names(fs, &herd).await {
        if name.starts_with("php") && !name.contains('.') {
            candidates.push(herd.join(name));
        }
    }
    for name in dir_names(fs, Path::new("/Applications/MAMP/bin/php")).await {
        candidates.push(
            Path::new("/Applications/MAMP/bin/php")
                .join(name)
                .join("bin/php"),
        );
    }

    let mut found: Vec<(PathBuf, String)> = Vec::new();
    let mut seen: Vec<PathBuf> = Vec::new();
    for candidate in candidates {
        let Ok(Some(_)) = fs.metadata(&candidate).await else {
            continue;
        };
        // Homebrew's bin/php is a link to a versioned install; list it once.
        let real = fs
            .canonicalize(&candidate)
            .await
            .unwrap_or_else(|_| candidate.clone());
        if seen.contains(&real) {
            continue;
        }
        seen.push(real);
        if let Some(info) = php_info(&candidate.to_string_lossy()).await {
            found.push((candidate, info.version));
        }
    }
    found
}

async fn dir_names(fs: &dyn Fs, dir: &Path) -> Vec<String> {
    let mut names = Vec::new();
    if let Ok(mut entries) = fs.read_dir(dir).await {
        while let Some(Ok(path)) = entries.next().await {
            if let Some(name) = path.file_name() {
                names.push(name.to_string_lossy().into_owned());
            }
        }
    }
    names.sort();
    names
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_php_version_and_extensions() {
        let info = parse_php_info("8.4.12\nCore\ndate\nstandard\nPDO\nmbstring\n").unwrap();
        assert_eq!(info.version, "8.4.12");
        assert_eq!(
            info.extensions,
            ["Core", "date", "mbstring", "PDO", "standard"]
        );
        assert_eq!(short_version(&info.version), "8.4");
        assert!(parse_php_info("PHP Warning: something\n").is_none());
        assert!(parse_php_info("").is_none());
    }
}
