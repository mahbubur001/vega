//! A bar above an editor showing a server copy (Remote Host "Edit Remote
//! File"): which server file it is, whether it changed on the server since it
//! was opened, and Upload / Compare / Reload from Server.

use std::path::PathBuf;
use std::sync::Arc;

use deployment::engine::{Engine, RemoteEdit};
use gpui::{Context, Entity, EventEmitter, Render, Task, Window};
use gpui_tokio::Tokio;
use ui::{Tooltip, prelude::*};
use workspace::{ItemHandle, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView};

use crate::store::DeploymentStore;

struct ActiveEdit {
    local: PathBuf,
    engine: Arc<Engine>,
    edit: RemoteEdit,
    server_name: SharedString,
}

pub(crate) struct RemoteEditBar {
    store: Entity<DeploymentStore>,
    active: Option<ActiveEdit>,
    /// The server's file changed since it was opened or last uploaded.
    changed_on_server: bool,
    message: Option<SharedString>,
    _task: Option<Task<()>>,
}

impl RemoteEditBar {
    pub(crate) fn new(store: Entity<DeploymentStore>) -> Self {
        Self {
            store,
            active: None,
            changed_on_server: false,
            message: None,
            _task: None,
        }
    }

    /// Asks the server whether its copy changed since we opened it.
    fn check_server(&mut self, cx: &mut Context<Self>) {
        let Some(active) = &self.active else {
            return;
        };
        let (engine, local) = (active.engine.clone(), active.local.clone());
        let check = Tokio::spawn(cx, async move {
            !engine.changed_remote_edits(Some(&local)).await.is_empty()
        });
        self._task = Some(cx.spawn(async move |this, cx| {
            let changed = check.await.unwrap_or(false);
            this.update(cx, |this, cx| {
                this.changed_on_server = changed;
                cx.notify();
            })
            .ok();
        }));
    }

    fn run(
        &mut self,
        done: &'static str,
        operation: impl FnOnce(
            Arc<Engine>,
            PathBuf,
        ) -> futures::future::BoxFuture<'static, deployment::Result<()>>
        + Send
        + 'static,
        cx: &mut Context<Self>,
    ) {
        let Some(active) = &self.active else {
            return;
        };
        let task = Tokio::spawn(cx, operation(active.engine.clone(), active.local.clone()));
        self._task = Some(cx.spawn(async move |this, cx| {
            let result = task.await;
            this.update(cx, |this, cx| {
                this.message = Some(match result {
                    Ok(Ok(())) => done.into(),
                    Ok(Err(error)) => error.to_string().into(),
                    Err(error) => error.to_string().into(),
                });
                this.check_server(cx);
            })
            .ok();
        }));
    }
}

impl EventEmitter<ToolbarItemEvent> for RemoteEditBar {}

impl ToolbarItemView for RemoteEditBar {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        self.message = None;
        self.changed_on_server = false;
        let local = active_pane_item
            .and_then(|item| item.project_path(cx))
            .and_then(|project_path| {
                let store = self.store.read(cx);
                store.absolute_path(&project_path, cx)
            });
        self.active = local.and_then(|local| {
            let engine = self.store.read(cx).engine_for_remote_edit(&local)?;
            let edit = engine.remote_edit(&local)?;
            let server_name = engine
                .servers()
                .into_iter()
                .find(|resolved| resolved.server.id == edit.server_id)
                .map(|resolved| resolved.server.name)
                .unwrap_or_else(|| edit.server_id.clone());
            Some(ActiveEdit {
                local,
                engine,
                edit,
                server_name: server_name.into(),
            })
        });
        if self.active.is_some() {
            self.check_server(cx);
            ToolbarItemLocation::Secondary
        } else {
            ToolbarItemLocation::Hidden
        }
    }
}

impl Render for RemoteEditBar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(active) = &self.active else {
            return div().into_any_element();
        };
        let status = cx.theme().status();
        let (background, border) = if self.changed_on_server {
            (status.warning_background, status.warning_border)
        } else {
            (status.info_background, status.info_border)
        };
        h_flex()
            .w_full()
            .px_3()
            .py_1()
            .gap_2()
            .bg(background)
            .border_b_1()
            .border_color(border)
            .child(
                Icon::new(if self.changed_on_server {
                    IconName::Warning
                } else {
                    IconName::Server
                })
                .size(IconSize::Small)
                .color(if self.changed_on_server {
                    Color::Warning
                } else {
                    Color::Accent
                }),
            )
            .child(
                h_flex()
                    .flex_1()
                    .min_w_0()
                    .gap_1()
                    .child(Label::new("Server copy of").size(LabelSize::Small).color(Color::Muted))
                    .child(
                        Label::new(active.edit.remote.clone())
                            .size(LabelSize::Small)
                            .buffer_font(cx)
                            .truncate(),
                    )
                    .child(
                        Label::new(format!("on {}", active.server_name))
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                    )
                    .child(
                        Label::new(match (&self.message, self.changed_on_server) {
                            (Some(message), _) => format!("· {message}"),
                            (None, true) => "· changed on the server since you opened it".into(),
                            (None, false) => "· saving uploads it".into(),
                        })
                        .size(LabelSize::Small)
                        .color(if self.changed_on_server && self.message.is_none() {
                            Color::Warning
                        } else {
                            Color::Muted
                        })
                        .truncate(),
                    ),
            )
            .child(
                Button::new("remote-edit-upload", "Upload")
                    .style(ButtonStyle::Outlined)
                    .label_size(LabelSize::Small)
                    .tooltip(Tooltip::text("Upload this file to the server now"))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.run(
                            "Uploaded",
                            |engine, local| {
                                Box::pin(async move {
                                    engine.on_saved(&local, true).await;
                                    Ok(())
                                })
                            },
                            cx,
                        )
                    })),
            )
            .child(
                Button::new("remote-edit-compare", "Compare")
                    .style(ButtonStyle::Outlined)
                    .label_size(LabelSize::Small)
                    .tooltip(Tooltip::text("Compare with the server's current version"))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.run(
                            "Compared",
                            |engine, local| {
                                Box::pin(async move { engine.compare_remote_edit(&local).await })
                            },
                            cx,
                        )
                    })),
            )
            .child(
                Button::new("remote-edit-reload", "Reload from Server")
                    .style(ButtonStyle::Outlined)
                    .label_size(LabelSize::Small)
                    .tooltip(Tooltip::text(
                        "Replace this copy with the server's version (unsaved edits are kept until you reload the editor)",
                    ))
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.run(
                            "Reloaded from the server",
                            |engine, local| {
                                Box::pin(async move { engine.reload_remote_edit(&local).await })
                            },
                            cx,
                        )
                    })),
            )
            .into_any_element()
    }
}
