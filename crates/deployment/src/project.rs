//! Locating a project's deployment config: always `.vega/deployment.json` in the project root.

use std::path::{Path, PathBuf};

/// Config location relative to the project root.
pub const CONFIG_PATH: &str = ".vega/deployment.json";

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn missing_config() {
        let dir = tempfile::tempdir().unwrap();
        let loc = locate_config(dir.path());
        assert_eq!(loc.path, dir.path().join(".vega/deployment.json"));
        assert!(!loc.exists);
    }

    #[test]
    fn existing_config() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join(".vega")).unwrap();
        fs::write(dir.path().join(".vega/deployment.json"), "{}").unwrap();
        assert!(locate_config(dir.path()).exists);
    }

    #[test]
    fn vscode_config_is_ignored() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir_all(dir.path().join(".vscode")).unwrap();
        fs::write(dir.path().join(".vscode/iremote.json"), "{}").unwrap();
        assert!(!locate_config(dir.path()).exists);
    }
}
