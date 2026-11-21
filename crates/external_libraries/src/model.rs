//! Packages the External Libraries node lists (`node_modules/@types`).

use std::path::PathBuf;

use serde_json::Value;

/// A package the project depends on, installed (or not) under `path`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Package {
    pub name: String,
    pub version: Option<String>,
    pub path: PathBuf,
    pub dev: bool,
}

/// The `version` field of an installed package's `package.json`.
pub(crate) fn installed_version(package_json: &str) -> Option<String> {
    serde_json::from_str::<Value>(package_json)
        .ok()?
        .get("version")?
        .as_str()
        .map(str::to_string)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_installed_version() {
        assert_eq!(
            installed_version(r#"{"name": "lodash", "version": "4.17.21"}"#).as_deref(),
            Some("4.17.21")
        );
        assert_eq!(installed_version("{}"), None);
    }
}
