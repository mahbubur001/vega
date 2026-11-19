//! File Transfers (PhpStorm's tool window of the same name): every upload,
//! download and server operation with its progress and result, Retry for
//! failed transfers, Cancel for running ones, and the deployment log.

use std::ops::Range;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use db::kvp::KeyValueStore;
use deployment::engine::Engine;
use deployment::prompt::Level;
use deployment::queue::{TaskInfo, TaskKind, TaskStatus};
use gpui::{
    Action, App, Context, Entity, EventEmitter, FocusHandle, Focusable, Pixels, Render,
    Subscription, UniformListScrollHandle, WeakEntity, Window, px, uniform_list,
};
use gpui_tokio::Tokio;
use time::{OffsetDateTime, UtcOffset};
use ui::{CommonAnimationExt as _, Tooltip, prelude::*};
use util::ResultExt as _;
use workspace::Workspace;
use workspace::dock::{DockPosition, Panel, PanelEvent};
use zed_actions::deployment::ToggleFileTransfers;

use crate::store::DeploymentStore;

const DOCK_POSITION_KEY: &str = "vega_file_transfers_dock_position";

#[derive(Clone, Copy, PartialEq, Eq)]
enum View {
    Transfers,
    Log,
}

pub struct TransfersPanel {
    store: Entity<DeploymentStore>,
    _workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    position: DockPosition,
    view: View,
    local_offset: UtcOffset,
    scroll_handle: UniformListScrollHandle,
    _subscription: Subscription,
}

impl TransfersPanel {
    pub(crate) fn new(
        store: Entity<DeploymentStore>,
        workspace: WeakEntity<Workspace>,
        cx: &mut Context<Self>,
    ) -> Self {
        let subscription = cx.observe(&store, |_, _, cx| cx.notify());
        let position = KeyValueStore::global(cx)
            .read_kvp(DOCK_POSITION_KEY)
            .log_err()
            .flatten()
            .and_then(|value| match value.as_str() {
                "left" => Some(DockPosition::Left),
                "right" => Some(DockPosition::Right),
                "bottom" => Some(DockPosition::Bottom),
                _ => None,
            })
            .unwrap_or(DockPosition::Bottom);
        Self {
            store,
            _workspace: workspace,
            focus_handle: cx.focus_handle(),
            position,
            view: View::Transfers,
            local_offset: UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC),
            scroll_handle: UniformListScrollHandle::new(),
            _subscription: subscription,
        }
    }

    fn tasks(&self, cx: &App) -> Vec<(Arc<Engine>, TaskInfo)> {
        self.store.read(cx).tasks()
    }

    /// Runs a failed upload or download again.
    fn retry(&mut self, engine: Arc<Engine>, task: TaskInfo, cx: &mut Context<Self>) {
        Tokio::spawn(cx, async move { retry_task(engine, task).await }).detach();
    }

    fn retry_all_failed(&mut self, cx: &mut Context<Self>) {
        for (engine, task) in self.tasks(cx) {
            if can_retry(&task) {
                self.retry(engine, task, cx);
            }
        }
    }

    fn render_toolbar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let tasks = self.tasks(cx);
        let failed = tasks.iter().filter(|(_, task)| can_retry(task)).count();
        let active = tasks
            .iter()
            .filter(|(_, task)| task.status.is_active())
            .count();
        let accent = cx.theme().colors().text_accent;
        let tab = |id: &'static str, label: String, view: View, cx: &mut Context<Self>| {
            let selected = self.view == view;
            v_flex()
                .id(id)
                .cursor_pointer()
                .pt_1()
                .gap_0p5()
                .child(Label::new(label).size(LabelSize::Small).color(if selected {
                    Color::Default
                } else {
                    Color::Muted
                }))
                .child(
                    div()
                        .h(px(2.))
                        .rounded_full()
                        .when(selected, |bar| bar.bg(accent)),
                )
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.view = view;
                    cx.notify();
                }))
        };
        h_flex()
            .px_2()
            .gap_3()
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .child(tab(
                "transfers-tab",
                if active > 0 {
                    format!("Transfers ({active} running)")
                } else {
                    "Transfers".into()
                },
                View::Transfers,
                cx,
            ))
            .child(tab("log-tab", "Log".into(), View::Log, cx))
            .child(div().flex_1())
            .when(failed > 0, |row| {
                row.child(
                    Button::new("retry-failed", format!("Retry Failed ({failed})"))
                        .label_size(LabelSize::Small)
                        .style(ButtonStyle::Outlined)
                        .on_click(cx.listener(|this, _, _, cx| this.retry_all_failed(cx))),
                )
            })
            .child(
                IconButton::new("cancel-all", IconName::Stop)
                    .icon_size(IconSize::Small)
                    .tooltip(Tooltip::text("Cancel All Transfers"))
                    .disabled(active == 0)
                    .on_click(cx.listener(|this, _, _, cx| {
                        for engine in this.store.read(cx).engines() {
                            engine.cancel(None);
                        }
                    })),
            )
            .child(
                IconButton::new("clear-transfers", IconName::Trash)
                    .icon_size(IconSize::Small)
                    .tooltip(Tooltip::text("Clear Finished and Log"))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.store.update(cx, |store, cx| store.clear_log(cx));
                    })),
            )
    }

    fn render_tasks(&mut self, range: Range<usize>, cx: &mut Context<Self>) -> Vec<AnyElement> {
        let tasks = self.tasks(cx);
        let now = OffsetDateTime::now_utc();
        range
            .filter_map(|index| {
                let (engine, task) = tasks.get(index)?.clone();
                let (icon, color) = status_icon(task.status);
                let name = Path::new(&task.source)
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_else(|| task.source.clone());
                let direction = match task.kind {
                    TaskKind::Upload => format!("→ {}", task.server_name),
                    TaskKind::Download => format!("← {}", task.server_name),
                    TaskKind::Delete => format!("deleted on {}", task.server_name),
                    TaskKind::Mkdir => format!("folder on {}", task.server_name),
                    TaskKind::Rename => format!("renamed on {}", task.server_name),
                    TaskKind::Chmod => format!("permissions on {}", task.server_name),
                };
                let when = crate::remote_host_panel::format_mtime(
                    task.finished_at
                        .or(task.started_at)
                        .unwrap_or(task.queued_at),
                    now,
                    self.local_offset,
                );
                let progress = (task.status == TaskStatus::Running && task.total > 0)
                    .then(|| task.bytes as f32 / task.total as f32);
                let retry_task = task.clone();
                let retry_engine = engine.clone();
                let task_id = task.id;
                let cancel_engine = engine;
                Some(
                    h_flex()
                        .id(("transfer", task.id as usize))
                        .w_full()
                        .px_2()
                        .py_0p5()
                        .gap_2()
                        .hover(|style| style.bg(cx.theme().colors().element_hover))
                        .child(
                            Icon::new(icon)
                                .size(IconSize::Small)
                                .color(color)
                                .map(|icon| {
                                    if task.status == TaskStatus::Running {
                                        icon.with_rotate_animation(2)
                                    } else {
                                        icon
                                    }
                                }),
                        )
                        .child(Label::new(name).truncate())
                        .child(
                            Label::new(direction)
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                                .single_line(),
                        )
                        .when_some(task.error.clone(), |row, error| {
                            row.child(
                                div().min_w_0().flex_1().child(
                                    Label::new(error)
                                        .size(LabelSize::Small)
                                        .color(Color::Error)
                                        .truncate(),
                                ),
                            )
                        })
                        .when(task.error.is_none(), |row| row.child(div().flex_1()))
                        .when_some(progress, |row, progress| {
                            row.child(
                                div()
                                    .w(px(120.))
                                    .h_1()
                                    .rounded_full()
                                    .bg(cx.theme().colors().border_variant)
                                    .child(
                                        div()
                                            .h_full()
                                            .rounded_full()
                                            .w(relative(progress.clamp(0., 1.)))
                                            .bg(cx.theme().colors().text_accent),
                                    ),
                            )
                        })
                        .when(task.total > 0, |row| {
                            row.child(
                                Label::new(crate::remote_host_panel::format_size(task.total))
                                    .size(LabelSize::XSmall)
                                    .color(Color::Muted),
                            )
                        })
                        .child(
                            Label::new(when)
                                .size(LabelSize::XSmall)
                                .color(Color::Muted)
                                .single_line(),
                        )
                        .when(can_retry(&task), |row| {
                            row.child(
                                Button::new(("retry", task.id as usize), "Retry")
                                    .label_size(LabelSize::Small)
                                    .style(ButtonStyle::Outlined)
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.retry(retry_engine.clone(), retry_task.clone(), cx)
                                    })),
                            )
                        })
                        .when(task.status.is_active(), |row| {
                            row.child(
                                IconButton::new(("cancel", task.id as usize), IconName::Close)
                                    .icon_size(IconSize::Small)
                                    .tooltip(Tooltip::text("Cancel"))
                                    .on_click(move |_, _, _| {
                                        cancel_engine.queue.cancel_task(task_id)
                                    }),
                            )
                        })
                        .into_any_element(),
                )
            })
            .collect()
    }

    fn render_log(&mut self, range: Range<usize>, cx: &mut Context<Self>) -> Vec<AnyElement> {
        let store = self.store.read(cx);
        let log = store.log();
        let now = OffsetDateTime::now_utc();
        range
            .filter_map(|index| {
                let line = log.get(index)?;
                let color = match line.level {
                    Level::Error => Color::Error,
                    Level::Warning => Color::Warning,
                    Level::Info => Color::Default,
                };
                Some(
                    h_flex()
                        .px_2()
                        .gap_2()
                        .child(
                            Label::new(crate::remote_host_panel::format_mtime(
                                line.at,
                                now,
                                self.local_offset,
                            ))
                            .size(LabelSize::XSmall)
                            .color(Color::Muted)
                            .buffer_font(cx),
                        )
                        .child(
                            Label::new(line.message.clone())
                                .size(LabelSize::Small)
                                .color(color)
                                .buffer_font(cx)
                                .truncate(),
                        )
                        .into_any_element(),
                )
            })
            .collect()
    }
}

/// Failed uploads and downloads can run again.
fn can_retry(task: &TaskInfo) -> bool {
    task.status == TaskStatus::Failed && matches!(task.kind, TaskKind::Upload | TaskKind::Download)
}

async fn retry_task(engine: Arc<Engine>, task: TaskInfo) {
    let result = match task.kind {
        TaskKind::Upload => engine
            .upload(Some(&task.server_id), &[PathBuf::from(&task.source)], true)
            .await
            .map(drop),
        TaskKind::Download => engine
            .remote_download(&task.server_id, std::slice::from_ref(&task.source))
            .await
            .map(drop),
        _ => Ok(()),
    };
    if let Err(error) = result {
        engine.log_line(Level::Error, format!("Retry of {}: {error}", task.source));
    }
}

fn status_icon(status: TaskStatus) -> (IconName, Color) {
    match status {
        TaskStatus::Queued => (IconName::Circle, Color::Muted),
        TaskStatus::Running => (IconName::ArrowCircle, Color::Accent),
        TaskStatus::Done => (IconName::Check, Color::Success),
        TaskStatus::Skipped => (IconName::Dash, Color::Muted),
        TaskStatus::Failed => (IconName::XCircle, Color::Error),
        TaskStatus::Cancelled => (IconName::Close, Color::Muted),
    }
}

impl Focusable for TransfersPanel {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<PanelEvent> for TransfersPanel {}

impl Render for TransfersPanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let count = match self.view {
            View::Transfers => self.tasks(cx).len(),
            View::Log => self.store.read(cx).log().len(),
        };
        let empty = match self.view {
            View::Transfers => {
                "No transfers yet. Uploads, downloads and server changes appear here."
            }
            View::Log => "The deployment log is empty.",
        };
        v_flex()
            .key_context("FileTransfers")
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(cx.theme().colors().panel_background)
            .child(self.render_toolbar(cx))
            .child(if count == 0 {
                h_flex()
                    .p_3()
                    .child(Label::new(empty).color(Color::Muted))
                    .into_any_element()
            } else {
                match self.view {
                    View::Transfers => uniform_list(
                        "file-transfers",
                        count,
                        cx.processor(|this, range: Range<usize>, _, cx| {
                            this.render_tasks(range, cx)
                        }),
                    ),
                    View::Log => uniform_list(
                        "deployment-log",
                        count,
                        cx.processor(|this, range: Range<usize>, _, cx| this.render_log(range, cx)),
                    ),
                }
                .flex_1()
                .track_scroll(&self.scroll_handle)
                .into_any_element()
            })
    }
}

impl Panel for TransfersPanel {
    fn persistent_name() -> &'static str {
        "FileTransfersPanel"
    }

    fn panel_key() -> &'static str {
        "FileTransfersPanel"
    }

    fn position(&self, _: &Window, _: &App) -> DockPosition {
        self.position
    }

    fn position_is_valid(&self, _: DockPosition) -> bool {
        true
    }

    fn set_position(&mut self, position: DockPosition, _: &mut Window, cx: &mut Context<Self>) {
        self.position = position;
        let value = match position {
            DockPosition::Left => "left",
            DockPosition::Right => "right",
            DockPosition::Bottom => "bottom",
        };
        let kvp = KeyValueStore::global(cx);
        db::write_and_log(cx, move || async move {
            kvp.write_kvp(DOCK_POSITION_KEY.into(), value.into()).await
        });
        cx.notify();
    }

    fn default_size(&self, _: &Window, _: &App) -> Pixels {
        px(220.)
    }

    /// Shown once the project has a deployment server.
    fn icon(&self, _: &Window, cx: &App) -> Option<IconName> {
        (!self.store.read(cx).engines().is_empty()).then_some(IconName::ArrowRightLeft)
    }

    fn icon_tooltip(&self, _: &Window, _: &App) -> Option<&'static str> {
        Some("File Transfers")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleFileTransfers)
    }

    fn activation_priority(&self) -> u32 {
        21
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn task(kind: TaskKind, status: TaskStatus) -> TaskInfo {
        TaskInfo {
            id: 1,
            kind,
            server_id: "s".into(),
            server_name: "S".into(),
            source: "/p/a.php".into(),
            target: "/www/a.php".into(),
            status,
            bytes: 0,
            total: 0,
            error: None,
            queued_at: 0,
            started_at: None,
            finished_at: None,
        }
    }

    #[test]
    fn only_failed_transfers_can_be_retried() {
        assert!(can_retry(&task(TaskKind::Upload, TaskStatus::Failed)));
        assert!(can_retry(&task(TaskKind::Download, TaskStatus::Failed)));
        assert!(!can_retry(&task(TaskKind::Upload, TaskStatus::Done)));
        assert!(!can_retry(&task(TaskKind::Delete, TaskStatus::Failed)));
    }
}
