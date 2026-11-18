//! Every transfer goes through the queue: it feeds the Transfers view and progress reporting. Tasks with the same
//! key (server + remote path) run in order; a queued upload/download of the same file is replaced by the newer one
//! and both callers wait for it. Concurrency is limited per server; cancel drops in-flight operations (their pooled
//! connection is discarded, see [`crate::pool::Lease`]).

use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};
use tokio::sync::{Notify, broadcast, oneshot};

use crate::error::{ErrorCode, RemoteError, Result};
use crate::state::now_ms;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TaskKind {
    Upload,
    Download,
    Delete,
    Mkdir,
    Rename,
    Chmod,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TaskStatus {
    Queued,
    Running,
    Done,
    Skipped,
    Failed,
    Cancelled,
}

impl TaskStatus {
    pub fn is_active(self) -> bool {
        matches!(self, TaskStatus::Queued | TaskStatus::Running)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskInfo {
    pub id: u64,
    pub kind: TaskKind,
    pub server_id: String,
    pub server_name: String,
    /// Local path for uploads, remote for downloads.
    pub source: String,
    pub target: String,
    pub status: TaskStatus,
    pub bytes: u64,
    pub total: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub queued_at: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskOutcome {
    Done,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum QueueEvent {
    Changed { task: TaskInfo },
    Idle,
}

pub type ProgressSink = Arc<dyn Fn(u64, u64) + Send + Sync>;
pub type Job = Box<dyn FnOnce(ProgressSink) -> BoxFuture<'static, Result<TaskOutcome>> + Send>;

pub struct TaskSpec {
    pub kind: TaskKind,
    pub server_id: String,
    pub server_name: String,
    pub source: String,
    pub target: String,
    /// Tasks sharing a key run strictly in order; queued uploads/downloads with the same key coalesce.
    pub key: String,
    /// Parallel tasks allowed for this server.
    pub concurrency: usize,
    pub job: Job,
}

struct Queued {
    id: u64,
    kind: TaskKind,
    server_id: String,
    key: String,
    concurrency: usize,
    job: Job,
    waiters: Vec<oneshot::Sender<Result<TaskOutcome>>>,
}

struct Running {
    server_id: String,
    key: String,
    cancel: Arc<Cancel>,
}

#[derive(Default)]
struct Cancel {
    flag: AtomicBool,
    notify: Notify,
}

impl Cancel {
    fn cancel(&self) {
        self.flag.store(true, Ordering::SeqCst);
        self.notify.notify_waiters();
    }

    async fn cancelled(&self) {
        loop {
            let notified = self.notify.notified();
            if self.flag.load(Ordering::SeqCst) {
                return;
            }
            notified.await;
        }
    }
}

const MAX_HISTORY: usize = 500;

#[derive(Default)]
struct State {
    queued: VecDeque<Queued>,
    running: HashMap<u64, Running>,
    /// Newest first.
    history: VecDeque<TaskInfo>,
}

pub struct TransferQueue {
    state: Mutex<State>,
    next_id: AtomicU64,
    events: broadcast::Sender<QueueEvent>,
}

impl Default for TransferQueue {
    fn default() -> Self {
        Self {
            state: Mutex::default(),
            next_id: AtomicU64::new(1),
            events: broadcast::channel(1024).0,
        }
    }
}

impl TransferQueue {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<QueueEvent> {
        self.events.subscribe()
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn emit_task(&self, info: &TaskInfo) {
        let _ = self.events.send(QueueEvent::Changed { task: info.clone() });
    }

    fn update(&self, id: u64, f: impl FnOnce(&mut TaskInfo)) -> Option<TaskInfo> {
        let mut st = self.lock();
        let info = st.history.iter_mut().find(|t| t.id == id)?;
        f(info);
        Some(info.clone())
    }

    pub fn tasks(&self) -> Vec<TaskInfo> {
        self.lock().history.iter().cloned().collect()
    }

    pub fn pending_count(&self) -> usize {
        let st = self.lock();
        st.queued.len() + st.running.len()
    }

    /// Queues a task and waits for its result.
    pub async fn run(self: &Arc<Self>, spec: TaskSpec) -> Result<TaskOutcome> {
        let rx = self.enqueue(spec);
        rx.await.unwrap_or_else(|_| Err(RemoteError::cancelled()))
    }

    pub fn enqueue(self: &Arc<Self>, spec: TaskSpec) -> oneshot::Receiver<Result<TaskOutcome>> {
        let (tx, rx) = oneshot::channel();
        {
            let mut st = self.lock();
            if matches!(spec.kind, TaskKind::Upload | TaskKind::Download)
                && let Some(dup) = st
                    .queued
                    .iter_mut()
                    .find(|q| q.key == spec.key && q.kind == spec.kind)
            {
                // A newer save of the same file supersedes the queued one; both callers wait for it.
                dup.job = spec.job;
                dup.waiters.push(tx);
                return rx;
            }
            let id = self.next_id.fetch_add(1, Ordering::SeqCst);
            let info = TaskInfo {
                id,
                kind: spec.kind,
                server_id: spec.server_id.clone(),
                server_name: spec.server_name,
                source: spec.source,
                target: spec.target,
                status: TaskStatus::Queued,
                bytes: 0,
                total: 0,
                error: None,
                queued_at: now_ms(),
                started_at: None,
                finished_at: None,
            };
            st.history.push_front(info.clone());
            while st.history.len() > MAX_HISTORY {
                // Drop the oldest finished entry.
                match st.history.iter().rposition(|t| !t.status.is_active()) {
                    Some(i) => {
                        st.history.remove(i);
                    }
                    None => break,
                }
            }
            st.queued.push_back(Queued {
                id,
                kind: spec.kind,
                server_id: spec.server_id,
                key: spec.key,
                concurrency: spec.concurrency.max(1),
                job: spec.job,
                waiters: vec![tx],
            });
            drop(st);
            self.emit_task(&info);
        }
        self.pump();
        rx
    }

    fn pump(self: &Arc<Self>) {
        let mut start = Vec::new();
        {
            let mut st = self.lock();
            let mut i = 0;
            while i < st.queued.len() {
                let q = &st.queued[i];
                let running_for = st
                    .running
                    .values()
                    .filter(|r| r.server_id == q.server_id)
                    .count();
                let key_busy = st.running.values().any(|r| r.key == q.key)
                    || st.queued.iter().take(i).any(|earlier| earlier.key == q.key);
                if running_for >= q.concurrency || key_busy {
                    i += 1;
                    continue;
                }
                let q = st.queued.remove(i).expect("index checked");
                let cancel = Arc::new(Cancel::default());
                st.running.insert(
                    q.id,
                    Running {
                        server_id: q.server_id.clone(),
                        key: q.key.clone(),
                        cancel: cancel.clone(),
                    },
                );
                start.push((q, cancel));
            }
        }
        for (q, cancel) in start {
            let this = self.clone();
            tokio::spawn(async move { this.execute(q, cancel).await });
        }
    }

    async fn execute(self: Arc<Self>, q: Queued, cancel: Arc<Cancel>) {
        let id = q.id;
        if let Some(info) = self.update(id, |t| {
            t.status = TaskStatus::Running;
            t.started_at = Some(now_ms());
        }) {
            self.emit_task(&info);
        }
        let last_emit = Mutex::new(Instant::now());
        let weak = Arc::downgrade(&self);
        let progress: ProgressSink = Arc::new(move |bytes, total| {
            let Some(this) = weak.upgrade() else { return };
            let info = this.update(id, |t| {
                t.bytes = bytes;
                t.total = total;
            });
            let mut last = last_emit.lock().unwrap_or_else(|e| e.into_inner());
            if last.elapsed().as_millis() >= 100 || bytes == total {
                *last = Instant::now();
                if let Some(info) = info {
                    this.emit_task(&info);
                }
            }
        });
        let result = tokio::select! {
            r = (q.job)(progress) => r,
            _ = cancel.cancelled() => Err(RemoteError::cancelled()),
        };
        let info = self.update(id, |t| {
            t.finished_at = Some(now_ms());
            match &result {
                Ok(TaskOutcome::Done) => {
                    t.status = TaskStatus::Done;
                    if t.total > 0 {
                        t.bytes = t.total;
                    }
                }
                Ok(TaskOutcome::Skipped) => t.status = TaskStatus::Skipped,
                Err(e) if e.code == ErrorCode::Cancelled => t.status = TaskStatus::Cancelled,
                Err(e) => {
                    t.status = TaskStatus::Failed;
                    t.error = Some(e.to_string());
                }
            }
        });
        let idle = {
            let mut st = self.lock();
            st.running.remove(&id);
            st.running.is_empty() && st.queued.is_empty()
        };
        if let Some(info) = info {
            self.emit_task(&info);
        }
        for w in q.waiters {
            let _ = w.send(result.clone());
        }
        if idle {
            let _ = self.events.send(QueueEvent::Idle);
        }
        self.pump();
    }

    /// Cancels queued and running tasks (of one server, or all).
    pub fn cancel(self: &Arc<Self>, server_id: Option<&str>) {
        let matches = |s: &str| server_id.is_none_or(|id| id == s);
        let (dropped, running): (Vec<Queued>, Vec<Arc<Cancel>>) = {
            let mut st = self.lock();
            let (drop, keep): (Vec<Queued>, Vec<Queued>) =
                st.queued.drain(..).partition(|q| matches(&q.server_id));
            st.queued = keep.into();
            let running = st
                .running
                .values()
                .filter(|r| matches(&r.server_id))
                .map(|r| r.cancel.clone())
                .collect();
            (drop, running)
        };
        for q in dropped {
            if let Some(info) = self.update(q.id, |t| {
                t.status = TaskStatus::Cancelled;
                t.finished_at = Some(now_ms());
            }) {
                self.emit_task(&info);
            }
            for w in q.waiters {
                let _ = w.send(Err(RemoteError::cancelled()));
            }
        }
        for c in running {
            c.cancel();
        }
    }

    /// Removes finished entries from the history (Retry keeps working until then).
    pub fn clear_finished(&self) {
        self.lock().history.retain(|t| t.status.is_active());
        let _ = self.events.send(QueueEvent::Idle);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;
    use std::time::Duration;

    fn spec(
        key: &str,
        server: &str,
        f: impl FnOnce() -> BoxFuture<'static, Result<TaskOutcome>> + Send + 'static,
    ) -> TaskSpec {
        TaskSpec {
            kind: TaskKind::Upload,
            server_id: server.into(),
            server_name: server.into(),
            source: key.into(),
            target: key.into(),
            key: key.into(),
            concurrency: 2,
            job: Box::new(move |_p| f()),
        }
    }

    #[tokio::test]
    async fn coalesces_queued_uploads_and_limits_concurrency() {
        let q = TransferQueue::new();
        let runs = Arc::new(AtomicUsize::new(0));
        let gate = Arc::new(Notify::new());
        let mut rx = Vec::new();
        // First task occupies key "a" until released.
        let g = gate.clone();
        rx.push(q.enqueue(spec("a", "s", move || {
            Box::pin(async move {
                g.notified().await;
                Ok(TaskOutcome::Done)
            })
        })));
        for _ in 0..3 {
            let r = runs.clone();
            rx.push(q.enqueue(spec("a", "s", move || {
                Box::pin(async move {
                    r.fetch_add(1, Ordering::SeqCst);
                    Ok(TaskOutcome::Done)
                })
            })));
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
        assert_eq!(
            q.tasks().len(),
            2,
            "three queued saves of the same file became one task"
        );
        gate.notify_one();
        for r in rx {
            assert_eq!(r.await.unwrap().unwrap(), TaskOutcome::Done);
        }
        assert_eq!(runs.load(Ordering::SeqCst), 1);
        assert_eq!(q.pending_count(), 0);
    }

    #[tokio::test]
    async fn cancel_running_and_queued() {
        let q = TransferQueue::new();
        let never =
            || -> BoxFuture<'static, Result<TaskOutcome>> { Box::pin(futures::future::pending()) };
        let a = q.enqueue(spec("a", "s", never));
        let b = q.enqueue(spec("b", "s", never));
        let c = q.enqueue(spec("c", "s", never));
        tokio::time::sleep(Duration::from_millis(20)).await;
        q.cancel(Some("s"));
        for r in [a, b, c] {
            assert_eq!(r.await.unwrap().unwrap_err().code, ErrorCode::Cancelled);
        }
        assert!(q.tasks().iter().all(|t| t.status == TaskStatus::Cancelled));
    }
}
