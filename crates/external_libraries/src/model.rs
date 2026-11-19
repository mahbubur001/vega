//! What the External Libraries node lists: packages from `composer.lock` and
//! `package.json`, and the PHP interpreter's extensions.

use std::path::{Path, PathBuf};

use serde_json::Value;

/// A package the project depends on, installed (or not) under `path`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Package {
    pub name: String,
    pub version: Option<String>,
    pub path: PathBuf,
    pub dev: bool,
}

/// Composer packages from `composer.lock` (`packages` and `packages-dev`),
/// installed under `<root>/vendor/<name>`, sorted by name.
pub(crate) fn composer_packages(lock: &str, root: &Path) -> Vec<Package> {
    let Ok(lock) = serde_json::from_str::<Value>(lock) else {
        return Vec::new();
    };
    let mut packages: Vec<Package> = [("packages", false), ("packages-dev", true)]
        .into_iter()
        .flat_map(|(key, dev)| {
            lock.get(key)
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(move |package| {
                    let name = package.get("name")?.as_str()?.to_string();
                    let version = package
                        .get("version")
                        .and_then(Value::as_str)
                        .map(|version| version.trim_start_matches('v').to_string());
                    Some(Package {
                        path: root.join("vendor").join(&name),
                        name,
                        version,
                        dev,
                    })
                })
        })
        .collect();
    packages.sort_by(|a, b| a.name.cmp(&b.name));
    packages.dedup_by(|a, b| a.name == b.name);
    packages
}

/// Dependency names from `package.json` (`dependencies`, then
/// `devDependencies`), with whether each is a dev dependency.
pub(crate) fn node_dependencies(package_json: &str) -> Vec<(String, bool)> {
    let Ok(package) = serde_json::from_str::<Value>(package_json) else {
        return Vec::new();
    };
    let mut names: Vec<(String, bool)> = Vec::new();
    for (key, dev) in [("dependencies", false), ("devDependencies", true)] {
        if let Some(dependencies) = package.get(key).and_then(Value::as_object) {
            for name in dependencies.keys() {
                if !names.iter().any(|(existing, _)| existing == name) {
                    names.push((name.clone(), dev));
                }
            }
        }
    }
    names.sort();
    names
}

/// The `version` field of an installed package's `package.json`.
pub(crate) fn installed_version(package_json: &str) -> Option<String> {
    serde_json::from_str::<Value>(package_json)
        .ok()?
        .get("version")?
        .as_str()
        .map(str::to_string)
}

/// PHP's version and loaded extensions, from the output of
/// `php -r 'echo PHP_VERSION, "\n", implode("\n", get_loaded_extensions());'`.
pub(crate) fn parse_php_info(output: &str) -> Option<(String, Vec<String>)> {
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
    Some((version, extensions))
}

/// `8.4.12` → `8.4`.
pub(crate) fn short_version(version: &str) -> String {
    version.split('.').take(2).collect::<Vec<_>>().join(".")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_composer_lock_packages_with_versions() {
        let lock = r#"{
            "packages": [
                {"name": "monolog/monolog", "version": "3.7.0"},
                {"name": "guzzlehttp/guzzle", "version": "v7.9.2"}
            ],
            "packages-dev": [
                {"name": "phpunit/phpunit", "version": "11.4.0"},
                {"name": "broken"}
            ]
        }"#;
        let packages = composer_packages(lock, Path::new("/site"));
        let names: Vec<&str> = packages.iter().map(|p| p.name.as_str()).collect();
        assert_eq!(
            names,
            [
                "broken",
                "guzzlehttp/guzzle",
                "monolog/monolog",
                "phpunit/phpunit"
            ]
        );
        assert_eq!(packages[1].version.as_deref(), Some("7.9.2"));
        assert_eq!(
            packages[1].path,
            Path::new("/site/vendor/guzzlehttp/guzzle")
        );
        assert!(!packages[1].dev);
        assert!(packages[3].dev);
        assert!(composer_packages("not json", Path::new("/site")).is_empty());
    }

    #[test]
    fn reads_package_json_dependencies() {
        let json = r#"{
            "dependencies": {"lodash": "^4", "react": "^19"},
            "devDependencies": {"@types/node": "^22", "lodash": "^4"}
        }"#;
        assert_eq!(
            node_dependencies(json),
            [
                ("@types/node".to_string(), true),
                ("lodash".to_string(), false),
                ("react".to_string(), false),
            ]
        );
        assert_eq!(
            installed_version(r#"{"name": "lodash", "version": "4.17.21"}"#).as_deref(),
            Some("4.17.21")
        );
        assert!(node_dependencies("{}").is_empty());
    }

    #[test]
    fn reads_php_version_and_extensions() {
        let (version, extensions) =
            parse_php_info("8.4.12\nCore\ndate\nstandard\nPDO\nmbstring\n").unwrap();
        assert_eq!(version, "8.4.12");
        assert_eq!(extensions, ["Core", "date", "mbstring", "PDO", "standard"]);
        assert_eq!(short_version(&version), "8.4");
        assert!(parse_php_info("PHP Warning: something\n").is_none());
        assert!(parse_php_info("").is_none());
    }
}
