//! Deployment engine: servers, path mappings, transfers, compare and sync for
//! SFTP, FTP/FTPS, WebDAV and local targets.
//!
//! Vendored from iRemote (MIT, see `NOTICE-iremote-MIT.txt`). It doesn't depend
//! on GPUI, so it can be tested on its own; `deployment_ui` wires it into VEGA.

pub mod config;
pub mod credentials;
pub mod deploy;
pub mod dirs;
pub mod engine;
pub mod error;
pub mod known_hosts;
pub mod manager;
pub mod mapper;
pub mod pool;
pub mod project;
pub mod prompt;
pub mod protocol;
pub mod queue;
pub mod ssh_config;
pub mod state;
pub mod sync;
pub mod tls;

pub use error::{ErrorCode, RemoteError, Result};

/// Version of the deployment engine.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
