//! Smart merge for `settings.json` and the rules that keep secrets off GitHub.
//!
//! Each machine compares its settings with the gist and with the *base* (what
//! both had after the last sync) one top-level key at a time, so edits to
//! different settings on two machines merge without a conflict. Only keys
//! changed differently on both sides conflict.
//!
//! What's compared and uploaded is the *public* view of the settings: values
//! that must never leave the machine (API keys, tokens, `env` blocks, SSH
//! connections, …) are removed first, and kept on each machine when its file
//! is updated. The local file is changed with JSONC text edits, one key at a
//! time, so comments and formatting survive.

use anyhow::{Context as _, Result, anyhow};
use serde_json::{Map, Value};

/// Which side wins keys both machines changed, once the user has chosen.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Prefer {
    Local,
    Remote,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum SettingsMerge {
    Merged {
        /// The new contents of the local file, if it changes.
        local: Option<String>,
        /// The new contents of the gist file, if it changes.
        upload: Option<String>,
        /// What both sides have after this sync.
        base: String,
    },
    /// Top-level keys both sides changed differently.
    Conflict(Vec<String>),
}

/// Words in a key name (any depth, split on `_`, `-` and `.`) that mark its value as never synced.
const PRIVATE_KEY_WORDS: [&str; 7] = [
    "token",
    "secret",
    "password",
    "passphrase",
    "auth",
    "credential",
    "credentials",
];
/// Top-level settings that are specific to a machine or hold credentials.
const PRIVATE_TOP_LEVEL_KEYS: [&str; 3] = ["ssh_connections", "proxy", "server_url"];
/// Prefixes of well-known API keys and tokens.
const SECRET_PREFIXES: [&str; 10] = [
    "sk-",
    "ghp_",
    "gho_",
    "ghu_",
    "ghs_",
    "github_pat_",
    "xoxb-",
    "xoxp-",
    "AKIA",
    "AIza",
];

pub(crate) fn is_private_key(key: &str, top_level: bool) -> bool {
    let key = key.to_ascii_lowercase();
    key == "env"
        || key.contains("api_key")
        || key.contains("apikey")
        || key
            .split(['_', '-', '.'])
            .any(|word| PRIVATE_KEY_WORDS.contains(&word))
        || (top_level && PRIVATE_TOP_LEVEL_KEYS.contains(&key.as_str()))
}

/// This machine's never-sync rules: the built-in ones plus its own
/// `settings_sync.ignored_settings` (top-level keys or dotted paths).
#[derive(Default)]
pub(crate) struct Rules {
    ignored: Vec<Vec<String>>,
}

impl Rules {
    pub(crate) fn for_settings(settings: &Map<String, Value>) -> Self {
        let ignored = settings
            .get("settings_sync")
            .and_then(|sync| sync.get("ignored_settings"))
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .map(|path| path.split('.').map(str::to_string).collect())
            .collect();
        Self { ignored }
    }

    fn is_private(&self, path: &[String]) -> bool {
        let Some(key) = path.last() else {
            return false;
        };
        let top_level = path.len() == 1;
        is_private_key(key, top_level)
            || (top_level && key == "settings_sync")
            || self
                .ignored
                .iter()
                .any(|ignored| ignored.as_slice() == path)
    }

    /// `value` with everything these rules keep private removed.
    pub(crate) fn public_view(&self, value: &Map<String, Value>) -> Map<String, Value> {
        fn strip(rules: &Rules, value: &Value, path: &mut Vec<String>) -> Value {
            match value {
                Value::Object(map) => Value::Object(strip_map(rules, map, path)),
                Value::Array(items) => {
                    Value::Array(items.iter().map(|item| strip(rules, item, path)).collect())
                }
                other => other.clone(),
            }
        }
        fn strip_map(
            rules: &Rules,
            map: &Map<String, Value>,
            path: &mut Vec<String>,
        ) -> Map<String, Value> {
            let mut public = Map::new();
            for (key, value) in map {
                path.push(key.clone());
                if !rules.is_private(path) {
                    public.insert(key.clone(), strip(rules, value, path));
                }
                path.pop();
            }
            public
        }
        strip_map(self, value, &mut Vec::new())
    }

    /// `public` (a value from the merge, at `path`) with the private entries of
    /// `source` (the same key in a full copy of the settings) put back.
    fn restore(
        &self,
        public: Option<&Value>,
        source: Option<&Value>,
        path: &mut Vec<String>,
    ) -> Option<Value> {
        match (public, source) {
            (Some(Value::Object(public)), Some(Value::Object(source))) => {
                let mut merged = public.clone();
                for (key, source_value) in source {
                    path.push(key.clone());
                    if self.is_private(path) {
                        merged.insert(key.clone(), source_value.clone());
                    } else if public.contains_key(key)
                        && let Some(value) = self.restore(public.get(key), Some(source_value), path)
                    {
                        merged.insert(key.clone(), value);
                    }
                    path.pop();
                }
                Some(Value::Object(merged))
            }
            (Some(public), _) => Some(public.clone()),
            // The key is gone from the merge: keep only the private entries.
            (None, Some(Value::Object(source))) => {
                let mut private = Map::new();
                for (key, value) in source {
                    path.push(key.clone());
                    if self.is_private(path) {
                        private.insert(key.clone(), value.clone());
                    } else if let Some(value) = self.restore(None, Some(value), path) {
                        private.insert(key.clone(), value);
                    }
                    path.pop();
                }
                (!private.is_empty()).then_some(Value::Object(private))
            }
            (None, _) => None,
        }
    }

    /// `public` with every top-level key's private entries from `source` put back.
    fn restore_map(
        &self,
        public: &Map<String, Value>,
        source: &Map<String, Value>,
    ) -> Map<String, Value> {
        let keys: std::collections::BTreeSet<&String> =
            public.keys().chain(source.keys()).collect();
        let mut result = Map::new();
        for key in keys {
            let mut path = vec![key.clone()];
            let value = if self.is_private(&path) {
                source.get(key).cloned()
            } else {
                self.restore(public.get(key), source.get(key), &mut path)
            };
            if let Some(value) = value {
                result.insert(key.clone(), value);
            }
        }
        result
    }
}

/// The JSON path of the first string that looks like an API key or token.
pub(crate) fn find_secret(value: &Value) -> Option<String> {
    fn search(value: &Value, path: &mut Vec<String>) -> Option<String> {
        match value {
            Value::String(text) => {
                let text = text.trim();
                let looks_secret = SECRET_PREFIXES
                    .iter()
                    .any(|prefix| text.starts_with(prefix) && text.len() >= prefix.len() + 16)
                    || text.contains("-----BEGIN") && text.contains("PRIVATE KEY");
                looks_secret.then(|| path.join("."))
            }
            Value::Object(map) => map.iter().find_map(|(key, value)| {
                path.push(key.clone());
                let found = search(value, path);
                path.pop();
                found
            }),
            Value::Array(items) => items.iter().enumerate().find_map(|(index, value)| {
                path.push(index.to_string());
                let found = search(value, path);
                path.pop();
                found
            }),
            _ => None,
        }
    }
    search(value, &mut Vec::new())
}

fn parse_object(text: &str, what: &str) -> Result<Map<String, Value>> {
    if text.trim().is_empty() {
        return Ok(Map::new());
    }
    match settings_json::parse_json_with_comments::<Value>(text)
        .with_context(|| format!("parsing {what}"))?
    {
        Value::Object(map) => Ok(map),
        Value::Null => Ok(Map::new()),
        _ => Err(anyhow!("{what} isn't a JSON object")),
    }
}

fn to_text(map: &Map<String, Value>) -> String {
    let mut text = serde_json::to_string_pretty(map).unwrap_or_else(|_| "{}".into());
    text.push('\n');
    text
}

/// Merges this machine's `settings.json` (`local`, raw JSONC) with the gist's
/// copy (`remote`, `None` if the gist has none) against `base`.
pub(crate) fn merge_settings(
    local: &str,
    remote: Option<&str>,
    base: Option<&str>,
    prefer: Option<Prefer>,
) -> Result<SettingsMerge> {
    let local_map = parse_object(local, "this machine's settings")?;
    let rules = Rules::for_settings(&local_map);
    let local_public = rules.public_view(&local_map);
    let remote_full = remote
        .map(|remote| parse_object(remote, "the settings on GitHub"))
        .transpose()?;
    // Compare only what this machine syncs, so its ignored settings are
    // neither overwritten here nor removed from GitHub.
    let remote = remote_full.as_ref().map(|remote| rules.public_view(remote));
    let base = base
        .map(|base| parse_object(base, "the last synced settings"))
        .transpose()?
        .map(|base| rules.public_view(&base));

    let merged = match &remote {
        None => local_public.clone(),
        Some(remote) => {
            let mut keys: Vec<&String> = local_public.keys().collect();
            for key in remote
                .keys()
                .chain(base.iter().flat_map(|base| base.keys()))
            {
                if !keys.contains(&key) {
                    keys.push(key);
                }
            }

            let mut merged = Map::new();
            let mut conflicts = Vec::new();
            for key in keys {
                let ours = local_public.get(key);
                let theirs = remote.get(key);
                let before = base.as_ref().and_then(|base| base.get(key));
                let chosen = if ours == theirs || theirs == before {
                    ours
                } else if ours == before {
                    theirs
                } else {
                    match prefer {
                        Some(Prefer::Local) => ours,
                        Some(Prefer::Remote) => theirs,
                        None => {
                            conflicts.push(key.clone());
                            continue;
                        }
                    }
                };
                if let Some(value) = chosen {
                    merged.insert(key.clone(), value.clone());
                }
            }
            if !conflicts.is_empty() {
                return Ok(SettingsMerge::Conflict(conflicts));
            }
            merged
        }
    };

    if let Some(path) = find_secret(&Value::Object(merged.clone())) {
        return Err(anyhow!(
            "settings.json has a value at `{path}` that looks like an API key or token, so it \
             wasn't synced. Move it out of settings.json (for example into a provider's key \
             field) and sync again."
        ));
    }

    let local = (merged != local_public)
        .then(|| apply_to_local(&rules, local, &local_map, &local_public, &merged))
        .transpose()?;
    // GitHub keeps what this machine ignores (other machines' values).
    let gist = match &remote_full {
        Some(remote_full) => rules.restore_map(&merged, remote_full),
        None => merged,
    };
    let upload = (remote_full.as_ref() != Some(&gist)
        && !(remote_full.is_none() && gist.is_empty()))
    .then(|| to_text(&gist));

    Ok(SettingsMerge::Merged {
        local,
        upload,
        base: to_text(&gist),
    })
}

/// Edits the local JSONC text key by key so its comments and formatting survive.
fn apply_to_local(
    rules: &Rules,
    text: &str,
    local: &Map<String, Value>,
    local_public: &Map<String, Value>,
    merged: &Map<String, Value>,
) -> Result<String> {
    let mut text = if text.trim().is_empty() {
        "{}\n".to_string()
    } else {
        text.to_string()
    };
    let tab_size = settings_json::infer_json_indent_size(&text);
    let keys = merged
        .keys()
        .chain(local_public.keys())
        .collect::<std::collections::BTreeSet<_>>();
    for key in keys {
        if merged.get(key) == local_public.get(key) {
            continue;
        }
        let new_value = rules.restore(merged.get(key), local.get(key), &mut vec![key.clone()]);
        let (range, replacement) = settings_json::replace_value_in_json_text(
            &text,
            &[key.as_str()],
            tab_size,
            new_value.as_ref(),
            None,
        );
        text.replace_range(range, &replacement);
    }
    Ok(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn merged(result: SettingsMerge) -> (Option<String>, Option<String>, String) {
        match result {
            SettingsMerge::Merged {
                local,
                upload,
                base,
            } => (local, upload, base),
            SettingsMerge::Conflict(keys) => panic!("unexpected conflict on {keys:?}"),
        }
    }

    fn value(text: &str) -> Value {
        settings_json::parse_json_with_comments(text).unwrap()
    }

    #[test]
    fn different_keys_changed_on_each_side_merge() {
        let base = r#"{ "vim_mode": false, "tab_size": 4 }"#;
        let local = "// mine\n{\n  \"vim_mode\": true,\n  \"tab_size\": 4\n}\n";
        let remote = r#"{ "vim_mode": false, "tab_size": 2 }"#;
        let (local_out, upload, base_out) =
            merged(merge_settings(local, Some(remote), Some(base), None).unwrap());

        let local_out = local_out.expect("tab_size comes from GitHub");
        assert!(
            local_out.starts_with("// mine\n"),
            "comments survive: {local_out}"
        );
        assert_eq!(
            value(&local_out),
            json!({ "vim_mode": true, "tab_size": 2 })
        );
        assert_eq!(
            value(&upload.expect("vim_mode goes to GitHub")),
            json!({ "vim_mode": true, "tab_size": 2 })
        );
        assert_eq!(value(&base_out), json!({ "vim_mode": true, "tab_size": 2 }));
    }

    #[test]
    fn same_key_changed_on_both_sides_conflicts() {
        let result = merge_settings(
            r#"{ "tab_size": 8 }"#,
            Some(r#"{ "tab_size": 2 }"#),
            Some(r#"{ "tab_size": 4 }"#),
            None,
        )
        .unwrap();
        assert_eq!(result, SettingsMerge::Conflict(vec!["tab_size".into()]));

        let (local, upload, _) = merged(
            merge_settings(
                r#"{ "tab_size": 8 }"#,
                Some(r#"{ "tab_size": 2 }"#),
                Some(r#"{ "tab_size": 4 }"#),
                Some(Prefer::Remote),
            )
            .unwrap(),
        );
        assert_eq!(value(&local.unwrap()), json!({ "tab_size": 2 }));
        assert_eq!(upload, None);
    }

    #[test]
    fn removals_merge_too() {
        let (local, upload, _) = merged(
            merge_settings(
                r#"{ "a": 1, "b": 2 }"#,
                Some(r#"{ "a": 1 }"#),
                Some(r#"{ "a": 1, "b": 2 }"#),
                None,
            )
            .unwrap(),
        );
        assert_eq!(value(&local.unwrap()), json!({ "a": 1 }));
        assert_eq!(upload, None);
    }

    #[test]
    fn private_values_never_leave_the_machine_and_survive_pulls() {
        let local = r#"{
  "terminal": { "font_size": 12, "env": { "TOKEN": "abc" } },
  "proxy": "http://localhost:8080",
  "language_models": { "openai": { "api_url": "https://api.openai.com/v1", "api_key": "nope" } }
}"#;
        let remote = r#"{ "terminal": { "font_size": 14 }, "language_models": { "openai": { "api_url": "https://api.openai.com/v1" } } }"#;
        let base = r#"{ "terminal": { "font_size": 12 }, "language_models": { "openai": { "api_url": "https://api.openai.com/v1" } } }"#;
        let (local_out, upload, base_out) =
            merged(merge_settings(local, Some(remote), Some(base), None).unwrap());

        assert_eq!(upload, None, "nothing public changed here");
        assert!(!base_out.contains("TOKEN") && !base_out.contains("proxy"));
        assert_eq!(
            value(&local_out.unwrap()),
            json!({
                "terminal": { "font_size": 14, "env": { "TOKEN": "abc" } },
                "proxy": "http://localhost:8080",
                "language_models": { "openai": { "api_url": "https://api.openai.com/v1", "api_key": "nope" } }
            })
        );
    }

    #[test]
    fn first_upload_is_scrubbed() {
        let (local, upload, _) = merged(
            merge_settings(
                r#"{ "theme": "VEGA Dark", "server_url": "https://example.com", "github_token": "x" }"#,
                None,
                None,
                None,
            )
            .unwrap(),
        );
        assert_eq!(local, None);
        assert_eq!(value(&upload.unwrap()), json!({ "theme": "VEGA Dark" }));
    }

    #[test]
    fn values_that_look_like_keys_block_the_upload() {
        let error = merge_settings(
            r#"{ "assistant": { "custom_header": "sk-ant-0123456789abcdef0123" } }"#,
            None,
            None,
            None,
        )
        .unwrap_err();
        assert!(
            error.to_string().contains("assistant.custom_header"),
            "{error}"
        );
        assert_eq!(find_secret(&json!({ "theme": "sk-dark" })), None);
    }

    #[test]
    fn fresh_machine_takes_everything_from_github() {
        let (local, upload, _) = merged(
            merge_settings(
                "// Settings\n{\n}\n",
                Some(r#"{ "vim_mode": true }"#),
                None,
                None,
            )
            .unwrap(),
        );
        assert_eq!(value(&local.unwrap()), json!({ "vim_mode": true }));
        assert_eq!(upload, None);
    }

    #[test]
    fn ignored_settings_stay_as_they_are_on_both_sides() {
        let local = r#"{
  "settings_sync": { "ignored_settings": ["buffer_font_size", "terminal.font_size"] },
  "buffer_font_size": 12,
  "terminal": { "font_size": 12, "shell": "zsh" }
}"#;
        let remote =
            r#"{ "buffer_font_size": 16, "terminal": { "font_size": 16, "shell": "fish" } }"#;
        let base = r#"{ "buffer_font_size": 16, "terminal": { "font_size": 16, "shell": "zsh" } }"#;
        let (local_out, upload, base_out) =
            merged(merge_settings(local, Some(remote), Some(base), None).unwrap());

        // The shell change arrives; this machine's ignored sizes stay.
        assert_eq!(
            value(&local_out.unwrap()),
            json!({
                "settings_sync": { "ignored_settings": ["buffer_font_size", "terminal.font_size"] },
                "buffer_font_size": 12,
                "terminal": { "font_size": 12, "shell": "fish" }
            })
        );
        // GitHub keeps the other machines' sizes, so nothing is uploaded.
        assert_eq!(upload, None);
        assert_eq!(value(&base_out), value(remote));
    }

    #[test]
    fn ignored_settings_are_not_uploaded_and_not_removed_from_github() {
        let local = r#"{
  "settings_sync": { "ignored_settings": ["buffer_font_size"] },
  "buffer_font_size": 12,
  "vim_mode": true
}"#;
        let remote = r#"{ "buffer_font_size": 16, "vim_mode": false }"#;
        let (local_out, upload, _) =
            merged(merge_settings(local, Some(remote), Some(remote), None).unwrap());
        assert_eq!(local_out, None);
        assert_eq!(
            value(&upload.unwrap()),
            json!({ "buffer_font_size": 16, "vim_mode": true }),
            "vim_mode goes up; GitHub's buffer_font_size is kept and the sync block isn't uploaded"
        );
    }

    #[test]
    fn private_key_rules() {
        assert!(is_private_key("api_key", false));
        assert!(is_private_key("GITHUB_TOKEN", false));
        assert!(is_private_key("env", false));
        assert!(is_private_key("ssh_connections", true));
        assert!(!is_private_key("ssh_connections", false));
        assert!(!is_private_key("theme", true));
        assert!(is_private_key("auth_token", false));
        assert!(
            !is_private_key("max_tokens", false),
            "token counts aren't secrets"
        );
        assert!(!is_private_key("max_output_tokens", false));
        assert!(!is_private_key("author", false));
    }
}
