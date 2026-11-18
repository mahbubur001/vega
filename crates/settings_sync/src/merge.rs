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

/// The settings with every never-sync value removed.
pub(crate) fn public_view(value: &Value) -> Value {
    fn strip(value: &Value, top_level: bool) -> Value {
        match value {
            Value::Object(map) => Value::Object(
                map.iter()
                    .filter(|(key, _)| !is_private_key(key, top_level))
                    .map(|(key, value)| (key.clone(), strip(value, false)))
                    .collect(),
            ),
            Value::Array(items) => Value::Array(items.iter().map(|v| strip(v, false)).collect()),
            other => other.clone(),
        }
    }
    strip(value, true)
}

/// `public` (a value from the merge) with the never-sync entries of this
/// machine's `local` value put back, at any depth.
fn restore_private(public: Option<&Value>, local: Option<&Value>) -> Option<Value> {
    match (public, local) {
        (Some(Value::Object(public)), Some(Value::Object(local))) => {
            let mut merged = public.clone();
            for (key, local_value) in local {
                if is_private_key(key, false) {
                    merged.insert(key.clone(), local_value.clone());
                } else if let Some(value) = restore_private(public.get(key), Some(local_value))
                    && public.contains_key(key)
                {
                    merged.insert(key.clone(), value);
                }
            }
            Some(Value::Object(merged))
        }
        (Some(public), _) => Some(public.clone()),
        // The key is gone from the merge: keep only this machine's private entries.
        (None, Some(Value::Object(local))) => {
            let private: Map<String, Value> = local
                .iter()
                .filter(|(key, _)| is_private_key(key, false))
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect();
            (!private.is_empty()).then_some(Value::Object(private))
        }
        (None, _) => None,
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
    let Value::Object(local_public) = public_view(&Value::Object(local_map.clone())) else {
        unreachable!("the public view of an object is an object");
    };
    let remote = remote
        .map(|remote| parse_object(remote, "the settings on GitHub"))
        .transpose()?;
    let base = base
        .map(|base| parse_object(base, "the last synced settings"))
        .transpose()?;

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
        .then(|| apply_to_local(local, &local_map, &local_public, &merged))
        .transpose()?;
    let upload = (remote.as_ref() != Some(&merged) && !(remote.is_none() && merged.is_empty()))
        .then(|| to_text(&merged));

    Ok(SettingsMerge::Merged {
        local,
        upload,
        base: to_text(&merged),
    })
}

/// Edits the local JSONC text key by key so its comments and formatting survive.
fn apply_to_local(
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
        let new_value = restore_private(merged.get(key), local.get(key));
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
