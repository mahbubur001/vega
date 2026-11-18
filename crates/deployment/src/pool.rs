//! One connection pool per server. The first connection opens alone (a bad password is asked once, not per
//! parallel request); later ones open in parallel up to the server's limit. Idle connections close after a while.
//!
//! A [`Lease`] returns its client on drop only when it was marked clean, so a transfer cancelled mid-operation
//! (future dropped) never hands a half-used connection to the next caller.

use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

use crate::error::Result;
use crate::protocol::RemoteClient;

#[async_trait]
pub trait ClientFactory: Send + Sync {
    async fn create(&self) -> Result<Box<dyn RemoteClient>>;
}

struct Inner {
    factory: Arc<dyn ClientFactory>,
    idle: std::sync::Mutex<Vec<(Box<dyn RemoteClient>, Instant)>>,
    permits: Arc<Semaphore>,
    first: tokio::sync::Mutex<()>,
    connected_once: AtomicBool,
    idle_close: Duration,
}

#[derive(Clone)]
pub struct ConnectionPool {
    inner: Arc<Inner>,
}

pub struct Lease {
    client: Option<Box<dyn RemoteClient>>,
    reused: bool,
    clean: bool,
    inner: Arc<Inner>,
    _permit: OwnedSemaphorePermit,
}

impl ConnectionPool {
    pub fn new(
        factory: Arc<dyn ClientFactory>,
        max_connections: usize,
        idle_close: Duration,
    ) -> Self {
        Self {
            inner: Arc::new(Inner {
                factory,
                idle: Default::default(),
                permits: Arc::new(Semaphore::new(max_connections.max(1))),
                first: tokio::sync::Mutex::new(()),
                connected_once: AtomicBool::new(false),
                idle_close,
            }),
        }
    }

    pub async fn acquire(&self) -> Result<Lease> {
        let permit = self
            .inner
            .permits
            .clone()
            .acquire_owned()
            .await
            .expect("pool semaphore never closes");
        let now = Instant::now();
        let reused = {
            let mut idle = self.inner.idle.lock().unwrap_or_else(|e| e.into_inner());
            idle.retain(|(c, at)| {
                c.is_connected() && now.duration_since(*at) < self.inner.idle_close
            });
            idle.pop().map(|(c, _)| c)
        };
        if let Some(client) = reused {
            return Ok(Lease {
                client: Some(client),
                reused: true,
                clean: false,
                inner: self.inner.clone(),
                _permit: permit,
            });
        }
        let client = if self.inner.connected_once.load(Ordering::SeqCst) {
            self.inner.factory.create().await?
        } else {
            let _first = self.inner.first.lock().await;
            let c = self.inner.factory.create().await?;
            self.inner.connected_once.store(true, Ordering::SeqCst);
            c
        };
        Ok(Lease {
            client: Some(client),
            reused: false,
            clean: false,
            inner: self.inner.clone(),
            _permit: permit,
        })
    }

    /// Closes idle connections older than the idle timeout.
    pub async fn reap(&self) {
        let now = Instant::now();
        let stale: Vec<Box<dyn RemoteClient>> = {
            let mut idle = self.inner.idle.lock().unwrap_or_else(|e| e.into_inner());
            let (keep, stale): (Vec<_>, Vec<_>) = idle.drain(..).partition(|(c, at)| {
                c.is_connected() && now.duration_since(*at) < self.inner.idle_close
            });
            *idle = keep;
            stale.into_iter().map(|(c, _)| c).collect()
        };
        for mut c in stale {
            c.close().await;
        }
    }

    pub async fn close_all(&self) {
        let all: Vec<_> = self
            .inner
            .idle
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .drain(..)
            .collect();
        for (mut c, _) in all {
            c.close().await;
        }
        self.inner.connected_once.store(false, Ordering::SeqCst);
    }

    pub fn same(&self, other: &ConnectionPool) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }

    pub fn idle_count(&self) -> usize {
        self.inner.idle.lock().map(|i| i.len()).unwrap_or(0)
    }
}

impl Lease {
    pub fn reused(&self) -> bool {
        self.reused
    }

    /// The operation finished; the connection may be reused.
    pub fn mark_clean(&mut self) {
        self.clean = true;
    }

    /// Never return this connection to the pool.
    pub fn discard(&mut self) {
        self.clean = false;
    }

    pub fn client(&mut self) -> &mut (dyn RemoteClient + 'static) {
        self.client
            .as_deref_mut()
            .expect("lease holds a client until dropped")
    }
}

impl Deref for Lease {
    type Target = dyn RemoteClient;
    fn deref(&self) -> &Self::Target {
        self.client
            .as_deref()
            .expect("lease holds a client until dropped")
    }
}

impl DerefMut for Lease {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.client()
    }
}

impl Drop for Lease {
    fn drop(&mut self) {
        let Some(client) = self.client.take() else {
            return;
        };
        if self.clean && client.is_connected() {
            self.inner
                .idle
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .push((client, Instant::now()));
        } else if let Ok(handle) = tokio::runtime::Handle::try_current() {
            let mut client = client;
            handle.spawn(async move { client.close().await });
        }
    }
}

/// Runs `$body` (an async block using `$c: &mut dyn RemoteClient`) on a pooled connection. Fatal errors discard the
/// connection; a dead reused connection is retried once on a fresh one.
#[macro_export]
macro_rules! with_client {
    ($pool:expr, |$c:ident| $body:expr) => {{
        let pool: &$crate::pool::ConnectionPool = &$pool;
        let mut attempt = 0u8;
        loop {
            let mut lease = match pool.acquire().await {
                Ok(l) => l,
                Err(e) => break Err(e),
            };
            let reused = lease.reused();
            let result = {
                let $c: &mut dyn $crate::protocol::RemoteClient = lease.client();
                $body.await
            };
            match result {
                Err(e) if e.is_fatal() || !lease.is_connected() => {
                    lease.discard();
                    if e.code == $crate::ErrorCode::Connection && reused && attempt == 0 {
                        attempt += 1;
                        continue;
                    }
                    break Err(e);
                }
                r => {
                    lease.mark_clean();
                    break r;
                }
            }
        }
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::local::LocalClient;
    use std::sync::atomic::AtomicUsize;

    struct Counting(AtomicUsize);

    #[async_trait]
    impl ClientFactory for Counting {
        async fn create(&self) -> Result<Box<dyn RemoteClient>> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Ok(Box::new(LocalClient::new()))
        }
    }

    #[tokio::test]
    async fn reuses_clean_connections_only() {
        let f = Arc::new(Counting(AtomicUsize::new(0)));
        let pool = ConnectionPool::new(f.clone(), 2, Duration::from_secs(60));
        {
            let mut l = pool.acquire().await.unwrap();
            l.mark_clean();
        }
        assert_eq!(pool.idle_count(), 1);
        {
            let l = pool.acquire().await.unwrap();
            assert!(l.reused());
            // dropped without mark_clean → discarded
        }
        assert_eq!(pool.idle_count(), 0);
        let r: Result<Vec<_>> = with_client!(pool, |c| async { c.list("/").await });
        assert!(r.is_ok());
        assert_eq!(pool.idle_count(), 1);
        assert_eq!(f.0.load(Ordering::SeqCst), 2);
    }
}
