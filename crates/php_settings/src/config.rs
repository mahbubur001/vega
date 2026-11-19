//! A project's PHP settings (`.vega/php.json`) and how they reach the PHP
//! language servers (`lsp` keys in `.vega/settings.json`).

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use settings_json::replace_value_in_json_text;

/// PHP versions the language level can be set to.
pub const LANGUAGE_LEVELS: &[&str] = &["7.4", "8.0", "8.1", "8.2", "8.3", "8.4", "8.5"];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "snake_case")]
pub struct PhpConfig {
    /// `8.3`; `None` lets the language server decide (composer.json, then the interpreter).
    pub language_level: Option<String>,
    /// The CLI interpreter's path; `None` is `php` from `PATH`.
    pub interpreter: Option<String>,
    /// Folders outside the project that PHP code may use (absolute paths).
    pub include_paths: Vec<String>,
    /// Extensions whose stubs the language server loads; `None` is its default set.
    pub stubs: Option<Vec<String>>,
    /// List Composer packages under External Libraries.
    pub composer_libraries: bool,
}

impl Default for PhpConfig {
    fn default() -> Self {
        Self {
            language_level: None,
            interpreter: None,
            include_paths: Vec::new(),
            stubs: None,
            composer_libraries: true,
        }
    }
}

impl PhpConfig {
    /// The interpreter command to run.
    pub fn php(&self) -> String {
        self.interpreter
            .clone()
            .filter(|path| !path.trim().is_empty())
            .unwrap_or_else(|| "php".into())
    }

    pub fn include_paths(&self) -> Vec<PathBuf> {
        self.include_paths.iter().map(PathBuf::from).collect()
    }
}

pub fn config_path(root: &Path) -> PathBuf {
    root.join(paths::local_settings_folder_name())
        .join("php.json")
}

pub fn settings_path(root: &Path) -> PathBuf {
    root.join(paths::local_settings_folder_name())
        .join("settings.json")
}

pub fn parse_config(text: &str) -> PhpConfig {
    serde_json::from_str(text).unwrap_or_default()
}

pub fn config_text(config: &PhpConfig) -> String {
    let mut text = serde_json::to_string_pretty(config).unwrap_or_else(|_| "{}".into());
    text.push('\n');
    text
}

const INTELEPHENSE: &[&str] = &["lsp", "intelephense", "settings", "intelephense"];
const PHPACTOR: &[&str] = &["lsp", "phpactor", "initialization_options"];

/// `settings.json` text with the language-server keys for `config` set (or
/// removed when they are back to the default). Other settings are kept.
pub fn settings_with_lsp_keys(settings: &str, config: &PhpConfig) -> String {
    let mut text = if settings.trim().is_empty() {
        "{\n}\n".to_string()
    } else {
        settings.to_string()
    };
    let include_paths = (!config.include_paths.is_empty()).then(|| json!(config.include_paths));
    let level = config.language_level.as_deref();
    let edits: [(Vec<&str>, Option<Value>); 5] = [
        (
            [INTELEPHENSE, &["environment", "includePaths"]].concat(),
            include_paths.clone(),
        ),
        (
            [INTELEPHENSE, &["environment", "phpVersion"]].concat(),
            level.map(|level| json!(format!("{level}.0"))),
        ),
        (
            [INTELEPHENSE, &["stubs"]].concat(),
            config.stubs.as_ref().map(|stubs| json!(stubs)),
        ),
        (
            [PHPACTOR, &["php.version"]].concat(),
            level.map(|level| json!(level)),
        ),
        ([PHPACTOR, &["indexer.stub_paths"]].concat(), include_paths),
    ];
    for (path, value) in edits {
        if value.is_none() && !has_key(&text, &path) {
            continue;
        }
        let (range, replacement) =
            replace_value_in_json_text(&text, &path, 2, value.as_ref(), None);
        text.replace_range(range, &replacement);
    }
    text
}

fn has_key(text: &str, path: &[&str]) -> bool {
    let Ok(value) = settings_json::parse_json_with_comments::<Value>(text) else {
        return false;
    };
    path.iter()
        .try_fold(&value, |value, key| value.get(*key))
        .is_some()
}

/// The lowest PHP version `composer.json` requires (`"php": "^8.2"` → `8.2`).
pub fn composer_php_level(composer_json: &str) -> Option<String> {
    let value: Value = serde_json::from_str(composer_json).ok()?;
    let constraint = value.get("require")?.get("php")?.as_str()?;
    let digits: String = constraint
        .trim_start_matches(|c: char| !c.is_ascii_digit())
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    let mut parts = digits.split('.').filter(|part| !part.is_empty());
    let major = parts.next()?;
    let minor = parts.next().unwrap_or("0");
    Some(format!("{major}.{minor}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_round_trips_with_defaults() {
        let config = parse_config(r#"{"interpreter": "/opt/homebrew/bin/php"}"#);
        assert_eq!(config.php(), "/opt/homebrew/bin/php");
        assert!(config.composer_libraries);
        assert_eq!(config.language_level, None);
        assert_eq!(parse_config(&config_text(&config)), config);
        assert_eq!(parse_config("not json"), PhpConfig::default());
        assert_eq!(PhpConfig::default().php(), "php");
    }

    #[test]
    fn writes_language_server_keys_and_keeps_other_settings() {
        let config = PhpConfig {
            language_level: Some("8.2".into()),
            include_paths: vec!["/libs/wordpress".into()],
            stubs: Some(vec!["Core".into(), "standard".into()]),
            ..Default::default()
        };
        let text = settings_with_lsp_keys("{\n  \"tab_size\": 4\n}\n", &config);
        let value: Value = settings_json::parse_json_with_comments(&text).unwrap();
        assert_eq!(value["tab_size"], 4);
        let intelephense = &value["lsp"]["intelephense"]["settings"]["intelephense"];
        assert_eq!(
            intelephense["environment"]["includePaths"],
            json!(["/libs/wordpress"])
        );
        assert_eq!(intelephense["environment"]["phpVersion"], "8.2.0");
        assert_eq!(intelephense["stubs"], json!(["Core", "standard"]));
        let phpactor = &value["lsp"]["phpactor"]["initialization_options"];
        assert_eq!(phpactor["php.version"], "8.2");
        assert_eq!(phpactor["indexer.stub_paths"], json!(["/libs/wordpress"]));

        // Back to the defaults: the keys go away, the rest stays.
        let text = settings_with_lsp_keys(&text, &PhpConfig::default());
        let value: Value = settings_json::parse_json_with_comments(&text).unwrap();
        assert_eq!(value["tab_size"], 4);
        let intelephense = &value["lsp"]["intelephense"]["settings"]["intelephense"];
        assert!(intelephense["environment"].get("phpVersion").is_none());
        assert!(intelephense.get("stubs").is_none());
        assert!(
            value["lsp"]["phpactor"]["initialization_options"]
                .get("php.version")
                .is_none()
        );

        // Nothing to write into an empty file stays an empty object.
        let value: Value = settings_json::parse_json_with_comments(&settings_with_lsp_keys(
            "",
            &PhpConfig::default(),
        ))
        .unwrap();
        assert_eq!(value, json!({}));
    }

    #[test]
    fn reads_the_php_requirement_from_composer_json() {
        let level = |require: &str| {
            composer_php_level(&format!(r#"{{"require": {{"php": "{require}"}}}}"#))
        };
        assert_eq!(level("^8.2").as_deref(), Some("8.2"));
        assert_eq!(level(">=7.4.0").as_deref(), Some("7.4"));
        assert_eq!(level("~8.1.3 || ^8.3").as_deref(), Some("8.1"));
        assert_eq!(level("8").as_deref(), Some("8.0"));
        assert_eq!(composer_php_level(r#"{"require": {}}"#), None);
        assert_eq!(composer_php_level("nope"), None);
    }
}
