//! Protocol-neutral errors. Callers branch on [`ErrorCode`], never on message text.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ErrorCode {
    NotFound,
    Exists,
    Permission,
    Auth,
    Connection,
    HostKey,
    Certificate,
    Unsupported,
    Cancelled,
    Config,
    /// An encrypted private key needs a (different) passphrase.
    Passphrase,
    Unknown,
}

#[derive(Debug, Clone, thiserror::Error)]
pub struct RemoteError {
    pub code: ErrorCode,
    pub message: String,
    pub path: Option<String>,
}

impl fmt::Display for RemoteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.path {
            Some(path) if !self.message.contains(path.as_str()) => {
                write!(f, "{}: {}", path, self.message)
            }
            _ => f.write_str(&self.message),
        }
    }
}

impl RemoteError {
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            path: None,
        }
    }

    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn not_found(path: impl Into<String>) -> Self {
        let path = path.into();
        Self {
            code: ErrorCode::NotFound,
            message: "No such file or directory".into(),
            path: Some(path),
        }
    }

    pub fn cancelled() -> Self {
        Self::new(ErrorCode::Cancelled, "Cancelled")
    }

    pub fn unsupported(what: impl Into<String>) -> Self {
        Self::new(ErrorCode::Unsupported, what)
    }

    pub fn connection(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::Connection, message)
    }

    /// Connection-level failures invalidate the client; operation-level ones don't.
    pub fn is_fatal(&self) -> bool {
        matches!(
            self.code,
            ErrorCode::Connection
                | ErrorCode::Auth
                | ErrorCode::HostKey
                | ErrorCode::Certificate
                | ErrorCode::Passphrase
        )
    }

    pub fn is(&self, code: ErrorCode) -> bool {
        self.code == code
    }
}

impl From<std::io::Error> for RemoteError {
    fn from(e: std::io::Error) -> Self {
        use std::io::ErrorKind as K;
        let code = match e.kind() {
            K::NotFound => ErrorCode::NotFound,
            K::AlreadyExists => ErrorCode::Exists,
            K::PermissionDenied => ErrorCode::Permission,
            K::ConnectionRefused
            | K::ConnectionReset
            | K::ConnectionAborted
            | K::NotConnected
            | K::BrokenPipe
            | K::TimedOut
            | K::UnexpectedEof
            | K::HostUnreachable
            | K::NetworkUnreachable => ErrorCode::Connection,
            _ => ErrorCode::Unknown,
        };
        Self::new(code, e.to_string())
    }
}

pub type Result<T, E = RemoteError> = std::result::Result<T, E>;
