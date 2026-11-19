//! Loads and saves `.vega/deployment.json` plus the shared servers file, and resolves servers for a project.

use std::path::{Path, PathBuf};

use serde_json::Value;

use super::model::*;
use super::normalize::{normalize_global, normalize_project, to_json};
use crate::error::{ErrorCode, RemoteError, Result};
use crate::project::locate_config;

#[derive(Debug, Clone)]
pub struct ConfigStore {
    root: PathBuf,
    project_path: PathBuf,
    global_path: PathBuf,
    pub project: ProjectConfig,
    pub global: GlobalConfig,
    project_exists: bool,
    /// Problems found while reading the files (shown to the user, never fatal).
    pub errors: Vec<String>,
}

fn read_json(path: &Path, errors: &mut Vec<String>) -> Option<Value> {
    let text = std::fs::read_to_string(path).ok()?;
    match serde_json::from_str(&text) {
        Ok(v) => Some(v),
        Err(e) => {
            errors.push(format!("{}: {e}", path.display()));
            None
        }
    }
}

impl ConfigStore {
    pub fn load(root: &Path) -> Self {
        Self::load_with(root, crate::dirs::global_servers_file())
    }

    pub fn load_with(root: &Path, global_path: PathBuf) -> Self {
        let mut store = Self {
            root: root.to_path_buf(),
            project_path: locate_config(root).path,
            global_path,
            project: ProjectConfig::default(),
            global: GlobalConfig::default(),
            project_exists: false,
            errors: Vec::new(),
        };
        store.reload();
        store
    }

    pub fn reload(&mut self) {
        self.errors.clear();
        let project = read_json(&self.project_path, &mut self.errors);
        self.project_exists = project.is_some() || self.project_path.is_file();
        self.project = project
            .map(|v| normalize_project(&v, &mut self.errors))
            .unwrap_or_default();
        self.global = read_json(&self.global_path, &mut self.errors)
            .map(|v| normalize_global(&v, &mut self.errors))
            .unwrap_or_default();
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn project_path(&self) -> &Path {
        &self.project_path
    }

    pub fn global_path(&self) -> &Path {
        &self.global_path
    }

    pub fn project_exists(&self) -> bool {
        self.project_exists
    }

    pub fn options(&self) -> &Options {
        &self.project.options
    }

    /// Project servers first, then shared ones (with this project's mappings). Project definitions win on id clashes.
    pub fn servers(&self) -> Vec<ResolvedServer> {
        let mut out: Vec<ResolvedServer> = self
            .project
            .servers
            .iter()
            .map(|s| ResolvedServer {
                server: s.clone(),
                shared: false,
            })
            .collect();
        for g in &self.global.servers {
            if out.iter().any(|r| r.server.id == g.id) {
                continue;
            }
            let mut server = g.clone();
            if let Some(data) = self.project.global_server_data.get(&g.id) {
                server.mappings = data.mappings.clone();
                server.excluded_local = data.excluded_local.clone();
                server.excluded_remote = data.excluded_remote.clone();
            }
            out.push(ResolvedServer {
                server,
                shared: true,
            });
        }
        out
    }

    pub fn server(&self, id: &str) -> Option<ResolvedServer> {
        self.servers().into_iter().find(|s| s.server.id == id)
    }

    pub fn default_server_id(&self) -> Option<&str> {
        self.project.default_server.as_deref()
    }

    pub fn default_server(&self) -> Option<ResolvedServer> {
        self.default_server_id().and_then(|id| self.server(id))
    }

    /// Transfer servers behind an id: the server itself, or a group's members.
    pub fn expand_targets(&self, id: &str) -> Vec<ResolvedServer> {
        match self.server(id) {
            Some(r) if r.server.kind == ServerType::Group => r
                .server
                .members
                .iter()
                .filter_map(|m| self.server(m))
                .filter(|m| m.server.kind.transfers())
                .collect(),
            Some(r) if r.server.kind.transfers() => vec![r],
            _ => Vec::new(),
        }
    }

    pub fn save_project(&mut self) -> Result<()> {
        crate::dirs::write_atomic(&self.project_path, to_json(&self.project).as_bytes()).map_err(
            |e| {
                RemoteError::new(
                    ErrorCode::Config,
                    format!("Cannot write {}: {e}", self.project_path.display()),
                )
            },
        )?;
        self.project_exists = true;
        Ok(())
    }

    pub fn save_global(&self) -> Result<()> {
        crate::dirs::write_atomic(&self.global_path, to_json(&self.global).as_bytes()).map_err(
            |e| {
                RemoteError::new(
                    ErrorCode::Config,
                    format!("Cannot write {}: {e}", self.global_path.display()),
                )
            },
        )
    }

    /// Adds or replaces a server. `shared` decides where the definition lives; mappings always stay in the project.
    pub fn upsert_server(&mut self, server: Server, shared: bool) -> Result<()> {
        let id = server.id.clone();
        let was_global = self.global.servers.iter().any(|s| s.id == id);
        self.project.servers.retain(|s| s.id != id);
        self.global.servers.retain(|s| s.id != id);
        if shared {
            let mut def = server.clone();
            def.mappings.clear();
            def.excluded_local.clear();
            def.excluded_remote.clear();
            self.global.servers.push(def);
            self.project.global_server_data.insert(
                id,
                ProjectServerData {
                    mappings: server.mappings,
                    excluded_local: server.excluded_local,
                    excluded_remote: server.excluded_remote,
                },
            );
        } else {
            self.project.global_server_data.remove(&id);
            self.project.servers.push(server);
        }
        if shared || was_global {
            self.save_global()?;
        }
        self.save_project()
    }

    pub fn delete_server(&mut self, id: &str) -> Result<()> {
        let was_global = self.global.servers.iter().any(|s| s.id == id);
        self.project.servers.retain(|s| s.id != id);
        self.global.servers.retain(|s| s.id != id);
        self.project.global_server_data.remove(id);
        if self.project.default_server.as_deref() == Some(id) {
            self.project.default_server = None;
        }
        for s in &mut self.project.servers {
            s.members.retain(|m| m != id);
        }
        if was_global {
            self.save_global()?;
        }
        self.save_project()
    }

    pub fn set_default_server(&mut self, id: Option<&str>) -> Result<()> {
        self.project.default_server = id.map(Into::into);
        self.save_project()
    }

    pub fn set_options(&mut self, options: Options) -> Result<()> {
        self.project.options = options;
        self.save_project()
    }

    /// Generates an unused server id from a display name.
    pub fn new_server_id(&self, name: &str) -> String {
        let base: String = name
            .to_lowercase()
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .trim_matches('-')
            .to_string();
        let base = if base.is_empty() {
            "server".to_string()
        } else {
            base
        };
        let taken: Vec<String> = self.servers().into_iter().map(|s| s.server.id).collect();
        (1..)
            .map(|n| {
                if n == 1 {
                    base.clone()
                } else {
                    format!("{base}-{n}")
                }
            })
            .find(|id| !taken.contains(id))
            .unwrap_or(base)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store() -> (tempfile::TempDir, ConfigStore) {
        let dir = tempfile::tempdir().unwrap();
        let s = ConfigStore::load_with(
            &dir.path().join("proj"),
            dir.path().join("global/servers.json"),
        );
        (dir, s)
    }

    #[test]
    fn shared_server_keeps_mappings_in_project() {
        let (_dir, mut s) = store();
        let mut server = Server::new("prod", "Production", ServerType::Sftp);
        server.host = "example.com".into();
        server.mappings = vec![Mapping {
            local: "".into(),
            remote: "/html".into(),
            web: "/".into(),
        }];
        s.upsert_server(server.clone(), true).unwrap();
        assert!(s.project.servers.is_empty());
        assert!(s.global.servers[0].mappings.is_empty());
        s.reload();
        let resolved = s.server("prod").unwrap();
        assert!(resolved.shared);
        assert_eq!(resolved.server, server);

        // Making it project-only again removes the shared definition.
        s.upsert_server(server, false).unwrap();
        s.reload();
        assert!(s.global.servers.is_empty());
        assert!(!s.server("prod").unwrap().shared);
    }

    #[test]
    fn groups_expand_to_transfer_members() {
        let (_dir, mut s) = store();
        for id in ["a", "b"] {
            let mut srv = Server::new(id, id, ServerType::Local);
            srv.root_path = "/tmp".into();
            s.upsert_server(srv, false).unwrap();
        }
        let mut g = Server::new("all", "All", ServerType::Group);
        g.members = vec!["a".into(), "b".into(), "missing".into()];
        s.upsert_server(g, false).unwrap();
        let ids: Vec<_> = s
            .expand_targets("all")
            .into_iter()
            .map(|r| r.server.id)
            .collect();
        assert_eq!(ids, ["a", "b"]);
        s.delete_server("a").unwrap();
        assert_eq!(s.server("all").unwrap().server.members, ["b", "missing"]);
    }

    #[test]
    fn new_ids_are_unique() {
        let (_dir, mut s) = store();
        s.upsert_server(
            Server::new("production", "Production", ServerType::Local),
            false,
        )
        .unwrap();
        assert_eq!(s.new_server_id("Production"), "production-2");
        assert_eq!(s.new_server_id("Staging (EU)"), "staging--eu");
    }
}
