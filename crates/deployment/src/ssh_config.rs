//! `~/.ssh/config` support. Effective settings come from `ssh -G <alias>`, so `Include`, `Match` and defaults behave
//! exactly like OpenSSH. The host list for SSH sessions is read directly.

use std::path::PathBuf;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ResolvedHost {
    pub hostname: String,
    pub user: Option<String>,
    pub port: u16,
    pub identity_files: Vec<PathBuf>,
    /// `ProxyJump` value (first hop only is used by the built-in client).
    pub proxy_jump: Option<String>,
}

/// Parses `ssh -G` output (lowercase `key value` lines).
pub fn parse_ssh_g(output: &str) -> ResolvedHost {
    let mut r = ResolvedHost {
        port: 22,
        ..Default::default()
    };
    for line in output.lines() {
        let Some((key, value)) = line.split_once(' ') else {
            continue;
        };
        let value = value.trim();
        match key {
            "hostname" => r.hostname = value.to_string(),
            "user" => r.user = Some(value.to_string()),
            "port" => r.port = value.parse().unwrap_or(22),
            "identityfile" => r.identity_files.push(crate::dirs::expand_home(value)),
            "proxyjump" if value != "none" => r.proxy_jump = Some(value.to_string()),
            _ => {}
        }
    }
    r
}

/// Resolves an alias through `ssh -G`. Falls back to the alias itself when OpenSSH isn't installed.
pub async fn resolve(alias: &str) -> ResolvedHost {
    let output = tokio::process::Command::new("ssh")
        .arg("-G")
        .arg(alias)
        .stdin(std::process::Stdio::null())
        .output()
        .await;
    match output {
        Ok(o) if o.status.success() => {
            let mut r = parse_ssh_g(&String::from_utf8_lossy(&o.stdout));
            if r.hostname.is_empty() {
                r.hostname = alias.to_string();
            }
            r
        }
        _ => ResolvedHost {
            hostname: alias.to_string(),
            port: 22,
            ..Default::default()
        },
    }
}

/// Concrete `Host` aliases (no wildcards) from `~/.ssh/config` and the files it `Include`s.
pub fn list_hosts() -> Vec<String> {
    let Some(home) = ::dirs::home_dir() else {
        return Vec::new();
    };
    let mut out = Vec::new();
    let mut seen_files = std::collections::HashSet::new();
    collect_hosts(
        &home.join(".ssh/config"),
        &home.join(".ssh"),
        &mut out,
        &mut seen_files,
    );
    out
}

fn collect_hosts(
    file: &std::path::Path,
    ssh_dir: &std::path::Path,
    out: &mut Vec<String>,
    seen: &mut std::collections::HashSet<PathBuf>,
) {
    if !seen.insert(file.to_path_buf()) {
        return;
    }
    let Ok(text) = std::fs::read_to_string(file) else {
        return;
    };
    for line in text.lines() {
        let line = line.trim();
        let (key, rest) = line
            .split_once(|c: char| c.is_whitespace() || c == '=')
            .unwrap_or((line, ""));
        let rest = rest
            .trim_start_matches(|c: char| c.is_whitespace() || c == '=')
            .trim();
        match key.to_ascii_lowercase().as_str() {
            "host" => {
                for alias in rest.split_whitespace() {
                    if !alias.contains(['*', '?', '!']) && !out.iter().any(|a| a == alias) {
                        out.push(alias.to_string());
                    }
                }
            }
            "include" => {
                for pattern in rest.split_whitespace() {
                    let path = crate::dirs::expand_home(pattern);
                    let path = if path.is_absolute() {
                        path
                    } else {
                        ssh_dir.join(path)
                    };
                    for f in expand_glob(&path) {
                        collect_hosts(&f, ssh_dir, out, seen);
                    }
                }
            }
            _ => {}
        }
    }
}

fn expand_glob(path: &std::path::Path) -> Vec<PathBuf> {
    let s = path.to_string_lossy();
    if !s.contains(['*', '?']) {
        return vec![path.to_path_buf()];
    }
    let (Some(dir), Some(name)) = (path.parent(), path.file_name()) else {
        return Vec::new();
    };
    let Ok(glob) = globset::Glob::new(&name.to_string_lossy()) else {
        return Vec::new();
    };
    let matcher = glob.compile_matcher();
    let mut files: Vec<PathBuf> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.file_name().is_some_and(|n| matcher.is_match(n)))
        .collect();
    files.sort();
    files
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_ssh_g() {
        let r = parse_ssh_g(
            "user deploy\nhostname 10.0.0.5\nport 2222\nidentityfile ~/.ssh/id_ed25519\nproxyjump bastion\n",
        );
        assert_eq!(r.hostname, "10.0.0.5");
        assert_eq!(r.user.as_deref(), Some("deploy"));
        assert_eq!(r.port, 2222);
        assert_eq!(r.identity_files.len(), 1);
        assert_eq!(r.proxy_jump.as_deref(), Some("bastion"));
        assert_eq!(parse_ssh_g("proxyjump none\n").proxy_jump, None);
    }
}
