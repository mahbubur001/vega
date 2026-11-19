//! Deployment configuration model, the Deployment settings: servers (Connection tab), mappings,
//! excluded paths and project-wide options. Secrets are never part of it; they live in the OS keychain.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum ServerType {
    #[default]
    Sftp,
    Ftp,
    Ftps,
    FtpsImplicit,
    Webdav,
    Local,
    /// "In place": the project is served directly; only the web URL is used.
    InPlace,
    /// "Server group": uploads go to every member.
    Group,
}

impl ServerType {
    pub const ALL: [ServerType; 8] = [
        ServerType::Sftp,
        ServerType::Ftp,
        ServerType::Ftps,
        ServerType::FtpsImplicit,
        ServerType::Webdav,
        ServerType::Local,
        ServerType::InPlace,
        ServerType::Group,
    ];

    pub fn label(self) -> &'static str {
        match self {
            ServerType::Sftp => "SFTP",
            ServerType::Ftp => "FTP",
            ServerType::Ftps => "FTPS (explicit)",
            ServerType::FtpsImplicit => "FTPS (implicit)",
            ServerType::Webdav => "WebDAV",
            ServerType::Local => "Local or mounted folder",
            ServerType::InPlace => "In place",
            ServerType::Group => "Server group",
        }
    }

    pub fn short_label(self) -> &'static str {
        match self {
            ServerType::Sftp => "SFTP",
            ServerType::Ftp => "FTP",
            ServerType::Ftps | ServerType::FtpsImplicit => "FTPS",
            ServerType::Webdav => "WebDAV",
            ServerType::Local => "Local",
            ServerType::InPlace => "In place",
            ServerType::Group => "Group",
        }
    }

    pub fn default_port(self) -> Option<u16> {
        match self {
            ServerType::Sftp => Some(22),
            ServerType::Ftp | ServerType::Ftps => Some(21),
            ServerType::FtpsImplicit => Some(990),
            _ => None,
        }
    }

    pub fn is_ftp(self) -> bool {
        matches!(
            self,
            ServerType::Ftp | ServerType::Ftps | ServerType::FtpsImplicit
        )
    }

    /// Types that transfer files (everything but In place and groups).
    pub fn transfers(self) -> bool {
        !matches!(self, ServerType::InPlace | ServerType::Group)
    }

    pub fn needs_host(self) -> bool {
        !matches!(
            self,
            ServerType::Local | ServerType::InPlace | ServerType::Group
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum SftpAuth {
    #[default]
    Password,
    Key,
    Agent,
    /// Use `~/.ssh/config` (HostName, User, Port, IdentityFile, ProxyJump) plus the agent.
    Openssh,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum WebdavAuth {
    #[default]
    Basic,
    Digest,
    None,
}

/// "Strict host key checking".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum HostKeyPolicy {
    #[default]
    Ask,
    AcceptNew,
    Yes,
    No,
}

/// Which SSH implementation runs SFTP and SSH sessions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum SshBackend {
    /// Pure-Rust client (russh).
    #[default]
    Builtin,
    /// The system `ssh` binary (ProxyJump chains, certificates, FIDO keys, Kerberos).
    Openssh,
}

/// FTPS data channel protection (PROT).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum DataProtection {
    #[default]
    Private,
    Clear,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Mapping {
    /// Local path relative to the project root ("" = the root) or absolute.
    pub local: String,
    /// Deployment path, relative to the server root path.
    pub remote: String,
    /// Web path, relative to the server web URL.
    pub web: String,
}

impl Mapping {
    pub fn root() -> Self {
        Self {
            local: String::new(),
            remote: "/".into(),
            web: "/".into(),
        }
    }
}

fn is_false(b: &bool) -> bool {
    !*b
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub kind: ServerType,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub host: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub username: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<SftpAuth>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private_key_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssh_config_host: Option<String>,
    /// Bastion as `user@host:port`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jump_host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssh_backend: Option<SshBackend>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strict_host_key_checking: Option<HostKeyPolicy>,
    #[serde(default)]
    pub root_path: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub web_url: String,
    #[serde(default = "default_true", skip_serializing_if = "is_true")]
    pub passive: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub anonymous: bool,
    /// FTP: use LIST instead of MLSD.
    #[serde(default, skip_serializing_if = "is_false")]
    pub use_list: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keep_alive_command: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_protection: Option<DataProtection>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub disable_tls13: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub allow_untrusted_certificate: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trusted_certificate_fingerprint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webdav_auth: Option<WebdavAuth>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keep_alive_seconds: Option<u32>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub allow_shell_commands: bool,
    /// SFTP: transfer with the system `rsync` over `ssh`.
    #[serde(default, skip_serializing_if = "is_false")]
    pub use_rsync: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rsync_options: Option<String>,
    /// Group members (server ids).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mappings: Vec<Mapping>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub excluded_local: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub excluded_remote: Vec<String>,
}

fn is_true(b: &bool) -> bool {
    *b
}

fn default_true() -> bool {
    true
}

impl Server {
    pub fn new(id: impl Into<String>, name: impl Into<String>, kind: ServerType) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            kind,
            port: kind.default_port(),
            auth: (kind == ServerType::Sftp).then_some(SftpAuth::Password),
            root_path: "/".into(),
            passive: true,
            mappings: if kind.transfers() {
                vec![Mapping::root()]
            } else {
                Vec::new()
            },
            ..Default::default()
        }
    }

    pub fn port_or_default(&self) -> u16 {
        self.port.or(self.kind.default_port()).unwrap_or(0)
    }

    pub fn max_connections_or_default(&self) -> usize {
        self.max_connections
            .map(|n| n.clamp(1, 16) as usize)
            .unwrap_or(match self.kind {
                ServerType::Sftp => 2,
                _ => 4,
            })
    }

    /// `user@host:port` for display.
    pub fn address(&self) -> String {
        match self.kind {
            ServerType::Local => self.root_path.clone(),
            ServerType::InPlace => self.web_url.clone(),
            ServerType::Group => format!("{} servers", self.members.len()),
            ServerType::Webdav => self.host.clone(),
            _ => {
                let user = if self.username.is_empty() {
                    String::new()
                } else {
                    format!("{}@", self.username)
                };
                format!("{user}{}:{}", self.host, self.port_or_default())
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum AutoUpload {
    Always,
    OnExplicitSave,
    #[default]
    Never,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum OverwriteCheck {
    None,
    #[default]
    TimestampAndSize,
    Content,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub enum LogLevel {
    Errors,
    #[default]
    Brief,
    Details,
}

pub const DEFAULT_EXCLUDE_NAMES: [&str; 9] = [
    ".svn",
    ".cvs",
    ".idea",
    ".DS_Store",
    ".git",
    ".hg",
    ".zed",
    ".vega",
    "node_modules",
];

/// Project-wide options (Deployment › Options). Field order follows the Options dialog.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Options {
    pub exclude_names: Vec<String>,
    pub logging: LogLevel,
    pub overwrite_up_to_date: bool,
    pub use_temp_file: bool,
    pub preserve_timestamps: bool,
    pub delete_missing_target: bool,
    pub create_empty_directories: bool,
    pub prompt_overwrite_local: bool,
    pub confirm_upload: bool,
    pub auto_upload: AutoUpload,
    pub skip_external_changes: bool,
    pub delete_remote_on_local_delete: bool,
    pub preserve_original_permissions: bool,
    /// Octal string ("644"); empty = server default.
    pub file_permissions: String,
    pub folder_permissions: String,
    pub warn_on_newer_remote: OverwriteCheck,
    pub notify_remote_changes: bool,
    pub confirm_remote_delete: bool,
    pub concurrency: u32,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            exclude_names: DEFAULT_EXCLUDE_NAMES
                .iter()
                .map(|s| s.to_string())
                .collect(),
            logging: LogLevel::Brief,
            overwrite_up_to_date: false,
            use_temp_file: false,
            preserve_timestamps: true,
            delete_missing_target: false,
            create_empty_directories: false,
            prompt_overwrite_local: true,
            confirm_upload: false,
            auto_upload: AutoUpload::Never,
            skip_external_changes: true,
            delete_remote_on_local_delete: false,
            preserve_original_permissions: false,
            file_permissions: String::new(),
            folder_permissions: String::new(),
            warn_on_newer_remote: OverwriteCheck::TimestampAndSize,
            notify_remote_changes: true,
            confirm_remote_delete: true,
            concurrency: 4,
        }
    }
}

impl Options {
    pub fn file_mode(&self) -> Option<u32> {
        parse_mode(&self.file_permissions)
    }

    pub fn folder_mode(&self) -> Option<u32> {
        parse_mode(&self.folder_permissions)
    }
}

pub fn parse_mode(s: &str) -> Option<u32> {
    let t = s.trim();
    if (3..=4).contains(&t.len()) && t.chars().all(|c| ('0'..='7').contains(&c)) {
        u32::from_str_radix(t, 8).ok()
    } else {
        None
    }
}

/// Per-project mappings for a shared server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct ProjectServerData {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mappings: Vec<Mapping>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub excluded_local: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub excluded_remote: Vec<String>,
}

/// `.vega/deployment.json`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectConfig {
    pub version: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_server: Option<String>,
    #[serde(default)]
    pub options: Options,
    /// Servers visible only for this project.
    #[serde(default)]
    pub servers: Vec<Server>,
    /// Mappings and excluded paths for shared servers, keyed by server id.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub global_server_data: std::collections::BTreeMap<String, ProjectServerData>,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            version: 1,
            default_server: None,
            options: Options::default(),
            servers: Vec::new(),
            global_server_data: Default::default(),
        }
    }
}

/// `~/.config/iremote/servers.json`: servers shared by all projects (no mappings).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlobalConfig {
    pub version: u32,
    #[serde(default)]
    pub servers: Vec<Server>,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            version: 1,
            servers: Vec::new(),
        }
    }
}

/// A server as used by a project: its definition plus the project's mappings.
#[derive(Debug, Clone, PartialEq)]
pub struct ResolvedServer {
    pub server: Server,
    /// True when defined in the shared servers file.
    pub shared: bool,
}
