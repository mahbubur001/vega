//! Lenient parsing of config files: one bad value is reported and replaced by its default, never fatal.

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

use super::model::*;

/// Server ids appear in cache paths and IPC messages, so they are limited to `[A-Za-z0-9._-]`.
pub fn is_valid_server_id(id: &str) -> bool {
    !id.is_empty()
        && id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'))
}

pub fn is_valid_permission(s: &str) -> bool {
    s.is_empty() || parse_mode(s).is_some()
}

/// Overlays the keys of `obj` onto `base` one at a time; keys that fail to deserialize are reported and skipped.
fn overlay<T: Serialize + DeserializeOwned + Clone>(
    base: T,
    obj: &Map<String, Value>,
    context: &str,
    skip: &[&str],
    errors: &mut Vec<String>,
) -> T {
    let Ok(Value::Object(mut current)) = serde_json::to_value(&base) else {
        return base;
    };
    let mut result = base;
    for (key, value) in obj {
        if skip.contains(&key.as_str()) {
            continue;
        }
        let mut candidate = current.clone();
        candidate.insert(key.clone(), value.clone());
        match serde_json::from_value::<T>(Value::Object(candidate.clone())) {
            Ok(parsed) => {
                result = parsed;
                current = candidate;
            }
            Err(e) => errors.push(format!(
                "{context}: invalid value for \"{key}\" ({e}); using the default."
            )),
        }
    }
    result
}

pub fn normalize_options(value: Option<&Value>, errors: &mut Vec<String>) -> Options {
    let Some(Value::Object(obj)) = value else {
        return Options::default();
    };
    let mut options = overlay(Options::default(), obj, "options", &[], errors);
    for (field, label) in [
        (&mut options.file_permissions, "filePermissions"),
        (&mut options.folder_permissions, "folderPermissions"),
    ] {
        *field = field.trim().to_string();
        if !is_valid_permission(field) {
            errors.push(format!(
                "options.{label} \"{field}\" is not an octal mode; ignored."
            ));
            field.clear();
        }
    }
    options.concurrency = options.concurrency.clamp(1, 16);
    options
}

pub fn normalize_server(
    obj: &Map<String, Value>,
    index: usize,
    errors: &mut Vec<String>,
) -> Option<Server> {
    let name = obj
        .get("name")
        .and_then(Value::as_str)
        .filter(|s| !s.trim().is_empty());
    let name = name
        .map(str::to_string)
        .unwrap_or_else(|| format!("Server {}", index + 1));
    let Some(id) = obj
        .get("id")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty())
    else {
        errors.push(format!("Server \"{name}\" has no id; skipped."));
        return None;
    };
    if !is_valid_server_id(id) {
        errors.push(format!(
            "Server \"{name}\": id \"{id}\" may only contain letters, digits, \".\", \"_\" and \"-\"; skipped."
        ));
        return None;
    }
    let kind = match obj.get("type") {
        None => ServerType::Sftp,
        Some(v) => serde_json::from_value(v.clone()).unwrap_or_else(|_| {
            errors.push(format!("Server \"{name}\": unknown type {v}, using SFTP."));
            ServerType::Sftp
        }),
    };
    if obj.contains_key("password") || obj.contains_key("passphrase") {
        errors.push(format!(
            "Server \"{name}\": passwords in the config file are ignored. They are stored in the OS keychain instead."
        ));
    }
    let mut server = Server::new(id, &name, kind);
    server.mappings.clear();
    let context = format!("Server \"{name}\"");
    let mut server = overlay(
        server,
        obj,
        &context,
        &["id", "name", "type", "password", "passphrase"],
        errors,
    );
    server.host = server.host.trim().to_string();
    if server.root_path.is_empty() {
        server.root_path = "/".into();
    }
    if server.port == Some(0) {
        server.port = kind.default_port();
    }
    if kind == ServerType::Sftp && server.auth.is_none() {
        server.auth = Some(SftpAuth::Password);
    }
    if kind.needs_host() && server.host.is_empty() {
        errors.push(format!("Server \"{name}\": host is empty."));
    }
    Some(server)
}

fn normalize_servers(value: Option<&Value>, errors: &mut Vec<String>) -> Vec<Server> {
    let mut seen = std::collections::HashSet::new();
    let mut servers = Vec::new();
    for (i, item) in value
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .enumerate()
    {
        let Value::Object(obj) = item else { continue };
        let Some(server) = normalize_server(obj, i, errors) else {
            continue;
        };
        if !seen.insert(server.id.clone()) {
            errors.push(format!(
                "Duplicate server id \"{}\"; second definition skipped.",
                server.id
            ));
            continue;
        }
        servers.push(server);
    }
    servers
}

pub fn normalize_project(raw: &Value, errors: &mut Vec<String>) -> ProjectConfig {
    let empty = Map::new();
    let obj = raw.as_object().unwrap_or(&empty);
    let mut global_server_data = std::collections::BTreeMap::new();
    if let Some(Value::Object(data)) = obj.get("globalServerData") {
        for (id, v) in data {
            let parsed = match v {
                Value::Object(o) => overlay(
                    ProjectServerData::default(),
                    o,
                    &format!("globalServerData.{id}"),
                    &[],
                    errors,
                ),
                _ => ProjectServerData::default(),
            };
            global_server_data.insert(id.clone(), parsed);
        }
    }
    ProjectConfig {
        version: 1,
        default_server: obj
            .get("defaultServer")
            .and_then(Value::as_str)
            .filter(|s| !s.is_empty())
            .map(Into::into),
        options: normalize_options(obj.get("options"), errors),
        servers: normalize_servers(obj.get("servers"), errors),
        global_server_data,
    }
}

pub fn normalize_global(raw: &Value, errors: &mut Vec<String>) -> GlobalConfig {
    let mut servers = normalize_servers(raw.get("servers"), errors);
    for s in &mut servers {
        s.mappings.clear();
        s.excluded_local.clear();
        s.excluded_remote.clear();
    }
    GlobalConfig {
        version: 1,
        servers,
    }
}

/// Pretty JSON with a trailing newline, so the file stays diff-friendly.
pub fn to_json<T: Serialize>(value: &T) -> String {
    let mut s = serde_json::to_string_pretty(value).unwrap_or_else(|_| "{}".into());
    s.push('\n');
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn bad_values_fall_back_to_defaults() {
        let mut errors = Vec::new();
        let cfg = normalize_project(
            &json!({
                "defaultServer": "prod",
                "options": { "autoUpload": "sometimes", "concurrency": 99, "filePermissions": "9z", "preserveTimestamps": false },
                "servers": [
                    { "id": "prod", "name": "Production", "type": "sftp", "host": " example.com ", "password": "x",
                      "mappings": [{ "local": "", "remote": "/html", "web": "/" }] },
                    { "id": "bad id", "name": "Bad" },
                    { "name": "No id" },
                    { "id": "prod", "name": "Dup", "type": "ftp", "host": "h" },
                    { "id": "web", "type": "gopher", "host": "h" }
                ]
            }),
            &mut errors,
        );
        assert_eq!(cfg.default_server.as_deref(), Some("prod"));
        assert_eq!(cfg.options.auto_upload, AutoUpload::Never);
        assert_eq!(cfg.options.concurrency, 16);
        assert_eq!(cfg.options.file_permissions, "");
        assert!(!cfg.options.preserve_timestamps);
        assert_eq!(cfg.servers.len(), 2);
        let prod = &cfg.servers[0];
        assert_eq!(prod.host, "example.com");
        assert_eq!(prod.port, Some(22));
        assert_eq!(prod.auth, Some(SftpAuth::Password));
        assert_eq!(
            prod.mappings,
            vec![Mapping {
                local: "".into(),
                remote: "/html".into(),
                web: "/".into()
            }]
        );
        assert_eq!(cfg.servers[1].kind, ServerType::Sftp);
        assert_eq!(errors.len(), 7, "{errors:#?}");
    }

    #[test]
    fn round_trip_is_stable() {
        let mut cfg = ProjectConfig::default();
        let mut s = Server::new("prod", "Production", ServerType::Ftps);
        s.host = "example.com".into();
        s.anonymous = true;
        cfg.servers.push(s);
        let text = to_json(&cfg);
        let mut errors = Vec::new();
        let back = normalize_project(&serde_json::from_str(&text).unwrap(), &mut errors);
        assert!(errors.is_empty(), "{errors:?}");
        assert_eq!(back, cfg);
        assert!(
            !text.contains("\"passive\""),
            "default passive=true is not written"
        );
    }

    #[test]
    fn group_and_in_place_need_no_host() {
        let mut errors = Vec::new();
        let cfg = normalize_project(
            &json!({ "servers": [
                { "id": "all", "name": "All", "type": "group", "members": ["a", "b"] },
                { "id": "here", "name": "Here", "type": "in-place", "webUrl": "http://localhost" }
            ]}),
            &mut errors,
        );
        assert!(errors.is_empty(), "{errors:?}");
        assert_eq!(cfg.servers[0].members, vec!["a", "b"]);
    }
}
