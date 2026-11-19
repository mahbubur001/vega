//! Local ↔ remote ↔ web path translation (longest mapping wins), excluded paths and exclude-by-name.
//!
//! Local paths are compared as forward-slash strings; `windows` selects case-insensitive matching and `\` handling
//! (tests pass it explicitly). Remote paths are POSIX.

use std::path::{Path, PathBuf};

use globset::{GlobBuilder, GlobMatcher};

use crate::config::Server;

#[derive(Debug, Clone)]
struct Resolved {
    local: String,
    remote: String,
    web: String,
}

#[derive(Debug, Clone)]
pub struct PathMapper {
    project_root: String,
    web_url: String,
    mappings: Vec<Resolved>,
    excluded_local: Vec<String>,
    excluded_remote: Vec<String>,
    exclude_names: Vec<GlobMatcher>,
    windows: bool,
}

/// Forward slashes, no trailing separator (except roots like "/" and "C:/").
pub fn to_slash(p: &str) -> String {
    let s = p.replace('\\', "/");
    let is_drive_root = s.len() == 3 && s.as_bytes()[1] == b':' && s.ends_with('/');
    if s.len() > 1 && s.ends_with('/') && !is_drive_root {
        s[..s.len() - 1].to_string()
    } else {
        s
    }
}

/// Normalizes a POSIX path: resolves `.`/`..`, collapses slashes, always absolute, no trailing slash.
pub fn norm_remote(p: &str) -> String {
    let slashed = p.replace('\\', "/");
    let mut parts: Vec<&str> = Vec::new();
    for seg in slashed.split('/') {
        match seg {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            s => parts.push(s),
        }
    }
    format!("/{}", parts.join("/"))
}

/// Joins remote paths POSIX-style.
pub fn join_remote(base: &str, rel: &str) -> String {
    if rel.starts_with('/') {
        norm_remote(rel)
    } else {
        norm_remote(&format!("{base}/{rel}"))
    }
}

pub fn remote_parent(p: &str) -> String {
    let n = norm_remote(p);
    match n.rfind('/') {
        Some(0) | None => "/".into(),
        Some(i) => n[..i].to_string(),
    }
}

pub fn remote_name(p: &str) -> &str {
    p.trim_end_matches('/').rsplit('/').next().unwrap_or("")
}

/// Where `path` ends up when `from` moves to `to`, or None when `path` isn't `from` or inside it.
pub fn rebase_remote(from: &str, to: &str, path: &str) -> Option<String> {
    let rest = relative_inside(&norm_remote(from), &norm_remote(path), false)?;
    Some(if rest.is_empty() {
        norm_remote(to)
    } else {
        join_remote(to, &rest)
    })
}

/// Path of `child` relative to `parent`, or None when it's not inside it.
fn relative_inside(parent: &str, child: &str, case_insensitive: bool) -> Option<String> {
    let (a, b): (String, String) = if case_insensitive {
        (parent.to_lowercase(), child.to_lowercase())
    } else {
        (parent.into(), child.into())
    };
    if a == b {
        return Some(String::new());
    }
    let prefix = if a == "/" || a.ends_with('/') {
        a
    } else {
        format!("{a}/")
    };
    b.starts_with(&prefix)
        .then(|| child[prefix.len()..].to_string())
}

fn is_absolute_local(p: &str, windows: bool) -> bool {
    if windows {
        let b = p.as_bytes();
        (b.len() >= 2 && b[1] == b':') || p.starts_with("\\\\") || p.starts_with("//")
    } else {
        p.starts_with('/')
    }
}

fn normalize_local(p: &str) -> String {
    // Resolve "." and ".." segments lexically, keeping a leading root ("/" or "C:/").
    let s = to_slash(p);
    let (root, rest) = if s.len() >= 2 && s.as_bytes()[1] == b':' {
        let end = if s.len() > 2 && s.as_bytes()[2] == b'/' {
            3
        } else {
            2
        };
        (s[..end].to_string(), &s[end..])
    } else if let Some(r) = s.strip_prefix('/') {
        ("/".to_string(), r)
    } else {
        (String::new(), s.as_str())
    };
    let mut parts: Vec<&str> = Vec::new();
    for seg in rest.split('/') {
        match seg {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            s => parts.push(s),
        }
    }
    let joined = parts.join("/");
    if root.is_empty() {
        joined
    } else if joined.is_empty() {
        to_slash(&root)
    } else {
        format!(
            "{}{joined}",
            if root.ends_with('/') {
                root
            } else {
                format!("{root}/")
            }
        )
    }
}

impl PathMapper {
    pub fn new(project_root: &Path, server: &Server, exclude_names: &[String]) -> Self {
        Self::with_platform(
            &project_root.to_string_lossy(),
            server,
            exclude_names,
            cfg!(windows),
        )
    }

    pub fn with_platform(
        project_root: &str,
        server: &Server,
        exclude_names: &[String],
        windows: bool,
    ) -> Self {
        let root = normalize_local(project_root);
        let local_abs = |p: &str| {
            if p.is_empty() || p == "." {
                root.clone()
            } else if is_absolute_local(p, windows) {
                normalize_local(p)
            } else {
                normalize_local(&format!("{root}/{p}"))
            }
        };
        let base = if server.root_path.is_empty() {
            "/"
        } else {
            server.root_path.as_str()
        };
        let mappings = server
            .mappings
            .iter()
            .map(|m| Resolved {
                local: local_abs(&m.local),
                // An empty deployment path is the root path itself.
                remote: if m.remote.trim_matches('/').is_empty() {
                    norm_remote(base)
                } else {
                    join_remote(base, m.remote.trim_start_matches('/'))
                },
                web: if m.web.is_empty() {
                    "/".into()
                } else {
                    m.web.clone()
                },
            })
            .collect();
        let exclude_names = exclude_names
            .iter()
            .filter(|p| !p.trim().is_empty())
            .filter_map(|p| {
                GlobBuilder::new(p.trim())
                    .case_insensitive(windows)
                    .literal_separator(true)
                    .build()
                    .ok()
            })
            .map(|g| g.compile_matcher())
            .collect();
        Self {
            project_root: root.clone(),
            web_url: server.web_url.trim().to_string(),
            mappings,
            excluded_local: server
                .excluded_local
                .iter()
                .filter(|p| !p.is_empty())
                .map(|p| local_abs(p))
                .collect(),
            excluded_remote: server
                .excluded_remote
                .iter()
                .filter(|p| !p.is_empty())
                .map(|p| norm_remote(p))
                .collect(),
            exclude_names,
            windows,
        }
    }

    pub fn has_mappings(&self) -> bool {
        !self.mappings.is_empty()
    }

    pub fn remote_roots(&self) -> Vec<String> {
        self.mappings.iter().map(|m| m.remote.clone()).collect()
    }

    pub fn local_roots(&self) -> Vec<PathBuf> {
        self.mappings
            .iter()
            .map(|m| PathBuf::from(&m.local))
            .collect()
    }

    fn match_local(&self, local: &str) -> Option<(&Resolved, String)> {
        let p = normalize_local(local);
        self.mappings
            .iter()
            .filter_map(|m| relative_inside(&m.local, &p, self.windows).map(|rel| (m, rel)))
            .max_by_key(|(m, _)| m.local.len())
    }

    fn match_remote(&self, remote: &str) -> Option<(&Resolved, String)> {
        let p = norm_remote(remote);
        self.mappings
            .iter()
            .filter_map(|m| relative_inside(&m.remote, &p, false).map(|rel| (m, rel)))
            .max_by_key(|(m, _)| m.remote.len())
    }

    /// Absolute remote path for a local file, or None when it isn't mapped.
    pub fn to_remote(&self, local: &Path) -> Option<String> {
        self.to_remote_str(&local.to_string_lossy())
    }

    pub fn to_remote_str(&self, local: &str) -> Option<String> {
        self.match_local(local)
            .map(|(m, rel)| join_remote(&m.remote, &rel))
    }

    /// Local path for a remote file, or None when the remote path isn't mapped.
    pub fn to_local(&self, remote: &str) -> Option<PathBuf> {
        self.match_remote(remote).map(|(m, rel)| {
            let joined = if rel.is_empty() {
                m.local.clone()
            } else {
                format!("{}/{rel}", m.local)
            };
            PathBuf::from(if self.windows {
                joined.replace('/', "\\")
            } else {
                joined
            })
        })
    }

    pub fn web_url_for_remote(&self, remote: &str) -> Option<String> {
        if self.web_url.is_empty() {
            return None;
        }
        let (m, rel) = self.match_remote(remote)?;
        let path = join_remote(&format!("/{}", m.web.trim_start_matches('/')), &rel);
        let encoded: Vec<String> = path
            .split('/')
            .map(|s| {
                url::form_urlencoded::byte_serialize(s.as_bytes())
                    .collect::<String>()
                    .replace('+', "%20")
            })
            .collect();
        Some(format!(
            "{}{}",
            self.web_url.trim_end_matches('/'),
            encoded.join("/")
        ))
    }

    pub fn web_url_for_local(&self, local: &Path) -> Option<String> {
        self.to_remote(local)
            .and_then(|r| self.web_url_for_remote(&r))
    }

    fn name_excluded(&self, rel: &str) -> bool {
        !rel.is_empty()
            && rel
                .split('/')
                .filter(|s| !s.is_empty())
                .any(|seg| self.exclude_names.iter().any(|g| g.is_match(seg)))
    }

    pub fn is_local_excluded(&self, local: &Path) -> bool {
        let p = normalize_local(&local.to_string_lossy());
        if self
            .excluded_local
            .iter()
            .any(|ex| relative_inside(ex, &p, self.windows).is_some())
        {
            return true;
        }
        let rel = match self.match_local(&p) {
            Some((_, rel)) => rel,
            None => relative_inside(&self.project_root, &p, self.windows).unwrap_or(p.clone()),
        };
        self.name_excluded(&rel)
    }

    pub fn is_remote_excluded(&self, remote: &str) -> bool {
        let p = norm_remote(remote);
        if self
            .excluded_remote
            .iter()
            .any(|ex| relative_inside(ex, &p, false).is_some())
        {
            return true;
        }
        match self.match_remote(&p) {
            Some((_, rel)) => self.name_excluded(&rel),
            None => self.name_excluded(&p),
        }
    }

    /// Mapped and not excluded: eligible for upload.
    pub fn is_deployable(&self, local: &Path) -> bool {
        self.to_remote(local).is_some() && !self.is_local_excluded(local)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Mapping, ServerType};

    fn server(maps: &[(&str, &str, &str)]) -> Server {
        let mut s = Server::new("s", "S", ServerType::Sftp);
        s.root_path = "/var/www".into();
        s.web_url = "https://example.com/".into();
        s.mappings = maps
            .iter()
            .map(|(l, r, w)| Mapping {
                local: (*l).into(),
                remote: (*r).into(),
                web: (*w).into(),
            })
            .collect();
        s
    }

    fn names() -> Vec<String> {
        vec![".git".into(), "node_modules".into(), "*.log".into()]
    }

    #[test]
    fn longest_local_mapping_wins() {
        let m = PathMapper::with_platform(
            "/p",
            &server(&[("", "/html", "/"), ("public/assets", "/cdn", "/static")]),
            &names(),
            false,
        );
        assert_eq!(
            m.to_remote_str("/p/public/index.php").as_deref(),
            Some("/var/www/html/public/index.php")
        );
        assert_eq!(
            m.to_remote_str("/p/public/assets/app.js").as_deref(),
            Some("/var/www/cdn/app.js")
        );
        assert_eq!(m.to_remote_str("/p").as_deref(), Some("/var/www/html"));
        assert_eq!(m.to_remote_str("/other/x").as_deref(), None);
        assert_eq!(
            m.to_local("/var/www/cdn/app.js"),
            Some(PathBuf::from("/p/public/assets/app.js"))
        );
        assert_eq!(m.to_local("/var/www/html"), Some(PathBuf::from("/p")));
        assert_eq!(m.to_local("/etc/passwd"), None);
    }

    #[test]
    fn web_urls() {
        let m = PathMapper::with_platform(
            "/p",
            &server(&[("", "/html", "/"), ("public/assets", "/cdn", "static")]),
            &[],
            false,
        );
        assert_eq!(
            m.web_url_for_remote("/var/www/html/a b.php").as_deref(),
            Some("https://example.com/a%20b.php")
        );
        assert_eq!(
            m.web_url_for_remote("/var/www/cdn/x.js").as_deref(),
            Some("https://example.com/static/x.js")
        );
    }

    #[test]
    fn excludes() {
        let mut s = server(&[("", "/html", "/")]);
        s.excluded_local = vec!["storage/logs".into()];
        s.excluded_remote = vec!["/var/www/html/uploads".into()];
        let m = PathMapper::with_platform("/p", &s, &names(), false);
        assert!(m.is_local_excluded(Path::new("/p/storage/logs/a.txt")));
        assert!(m.is_local_excluded(Path::new("/p/.git/HEAD")));
        assert!(m.is_local_excluded(Path::new("/p/app/debug.log")));
        assert!(!m.is_local_excluded(Path::new("/p/app/logger.php")));
        assert!(m.is_remote_excluded("/var/www/html/uploads/x.png"));
        assert!(m.is_remote_excluded("/var/www/html/node_modules/a/b.js"));
        assert!(!m.is_deployable(Path::new("/p/node_modules/x")));
        assert!(m.is_deployable(Path::new("/p/index.php")));
    }

    #[test]
    fn windows_paths_are_case_insensitive() {
        let m = PathMapper::with_platform(
            "C:\\Proj",
            &server(&[("", "/html", "/"), ("D:\\shared", "/shared", "/")]),
            &names(),
            true,
        );
        assert_eq!(
            m.to_remote_str("c:\\proj\\Src\\A.php").as_deref(),
            Some("/var/www/html/Src/A.php")
        );
        assert_eq!(
            m.to_remote_str("D:/shared/x").as_deref(),
            Some("/var/www/shared/x")
        );
        assert_eq!(
            m.to_local("/var/www/html/a/b.txt"),
            Some(PathBuf::from("C:\\Proj\\a\\b.txt"))
        );
        assert!(m.is_local_excluded(Path::new("C:\\Proj\\.GIT\\config")));
    }

    #[test]
    fn remote_path_helpers() {
        assert_eq!(norm_remote("/a//b/./c/../d/"), "/a/b/d");
        assert_eq!(join_remote("/a", "b/c"), "/a/b/c");
        assert_eq!(remote_parent("/a/b"), "/a");
        assert_eq!(remote_parent("/a"), "/");
        assert_eq!(remote_name("/a/b.txt"), "b.txt");
    }

    #[test]
    fn empty_deployment_path_is_the_root_path() {
        let s = server(&[("", "", "")]);
        let m = PathMapper::with_platform("/p", &s, &[], false);
        assert_eq!(m.remote_roots(), ["/var/www"]);
        assert_eq!(
            m.to_remote(Path::new("/p/index.php")).as_deref(),
            Some("/var/www/index.php")
        );
    }

    #[test]
    fn rebases_paths_under_a_moved_folder() {
        assert_eq!(rebase_remote("/a", "/b", "/a").as_deref(), Some("/b"));
        assert_eq!(
            rebase_remote("/a", "/b", "/a/x/y").as_deref(),
            Some("/b/x/y")
        );
        assert_eq!(rebase_remote("/", "/b", "/x").as_deref(), Some("/b/x"));
        assert_eq!(rebase_remote("/a", "/b", "/ab"), None);
        assert_eq!(rebase_remote("/a/x", "/b", "/a"), None);
    }
}
