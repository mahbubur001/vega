//! Locating a project's deployment config: always `.zed/deployment.json` in the project root.

use std::path::{Path, PathBuf};

/// Config location relative to the project root.
pub const CONFIG_PATH: &str = ".zed/deployment.json";

/// iRemote's config file, imported once into [`CONFIG_PATH`].
pub const IREMOTE_CONFIG_PATH: &str = ".zed/iremote.json";

/// Where a project's config lives (or will be created).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigLocation {
    pub path: PathBuf,
    pub exists: bool,
}

pub fn locate_config(root: &Path) -> ConfigLocation {
    let path = root.join(CONFIG_PATH);
    let exists = path.is_file();
    ConfigLocation { path, exists }
}

/// Copies an iRemote config into [`CONFIG_PATH`] if the project has one and no
/// VEGA config yet. Returns whether it imported. The iRemote file is left as is.
pub fn import_iremote_config(root: &Path) -> std::io::Result<bool> {
    let target = root.join(CONFIG_PATH);
    let source = root.join(IREMOTE_CONFIG_PATH);
    if target.exists() || !source.is_file() {
        return Ok(false);
    }
    std::fs::copy(&source, &target)?;
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn missing_config() {
        let dir = tempfile::tempdir().unwrap();
        let loc = locate_config(dir.path());
        assert_eq!(loc.path, dir.path().join(".zed/deployment.json"));
        assert!(!loc.exists);
    }

    #[test]
    fn existing_config() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join(".zed")).unwrap();
        fs::write(dir.path().join(".zed/deployment.json"), "{}").unwrap();
        assert!(locate_config(dir.path()).exists);
    }

    #[test]
    fn imports_iremote_config_once() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join(".zed")).unwrap();
        fs::write(dir.path().join(".zed/iremote.json"), "{\"version\":1}").unwrap();

        assert!(import_iremote_config(dir.path()).unwrap());
        assert_eq!(
            fs::read_to_string(dir.path().join(".zed/deployment.json")).unwrap(),
            "{\"version\":1}"
        );
        assert!(dir.path().join(".zed/iremote.json").exists());

        fs::write(dir.path().join(".zed/iremote.json"), "{\"version\":2}").unwrap();
        assert!(!import_iremote_config(dir.path()).unwrap());
    }

    #[test]
    fn vscode_config_is_ignored() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join(".vscode")).unwrap();
        fs::write(dir.path().join(".vscode/iremote.json"), "{}").unwrap();
        assert!(!locate_config(dir.path()).exists);
    }
}
