//! Deployment Servers (D2): a PhpStorm-style dialog to add, edit, test and
//! remove deployment servers without editing JSON. The server list is on the
//! left (★ marks the default); the selected server's Connection, Mappings and
//! Excluded Paths are on the right.
//!
//! Edits stay in a draft until Save, which writes them through the
//! deployment `ConfigStore`. The `DeploymentStore` notices the config file
//! change and (re)loads the project's engine.

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use deployment::config::{ConfigStore, Mapping, ResolvedServer, Server, ServerType, SftpAuth};
use deployment::credentials::{Credentials, password_account};
use deployment::engine::Engine;
use deployment::manager::{StepState, TestStep};
use deployment::prompt::Prompter;
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Render, Task, Window,
};
use gpui_tokio::Tokio;
use ui::{ContextMenu, DropdownMenu, ListItem, ListItemSpacing, prelude::*};
use ui_input::InputField;
use util::ResultExt as _;
use workspace::ModalView;

/// The server types offered when adding a server. Groups and "in place"
/// servers are still available by editing the config file.
const ADDABLE_TYPES: [ServerType; 6] = [
    ServerType::Sftp,
    ServerType::Ftp,
    ServerType::Ftps,
    ServerType::FtpsImplicit,
    ServerType::Webdav,
    ServerType::Local,
];

const SFTP_AUTH: [(SftpAuth, &str); 4] = [
    (SftpAuth::Password, "Password"),
    (SftpAuth::Key, "Key pair"),
    (SftpAuth::Agent, "SSH agent"),
    (SftpAuth::Openssh, "OpenSSH config"),
];

/// The servers being edited, before Save.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Draft {
    pub servers: Vec<ResolvedServer>,
    pub default_id: Option<String>,
}

impl Draft {
    pub(crate) fn load(store: &ConfigStore) -> Self {
        Self {
            servers: store.servers(),
            default_id: store.default_server_id().map(str::to_string),
        }
    }

    /// Writes the draft: removes deleted servers, saves every other one and the default.
    pub(crate) fn apply(&self, store: &mut ConfigStore) -> Result<()> {
        let kept: Vec<&str> = self.servers.iter().map(|s| s.server.id.as_str()).collect();
        let removed: Vec<String> = store
            .servers()
            .into_iter()
            .map(|s| s.server.id)
            .filter(|id| !kept.contains(&id.as_str()))
            .collect();
        for id in removed {
            store.delete_server(&id)?;
        }
        for resolved in &self.servers {
            store.upsert_server(resolved.server.clone(), resolved.shared)?;
        }
        let default_id = self
            .default_id
            .as_deref()
            .filter(|id| kept.contains(id))
            .or_else(|| kept.first().copied());
        store.set_default_server(default_id)?;
        Ok(())
    }
}

/// Text fields for the selected server.
struct Form {
    name: Entity<InputField>,
    host: Entity<InputField>,
    port: Entity<InputField>,
    username: Entity<InputField>,
    password: Entity<InputField>,
    key_path: Entity<InputField>,
    root_path: Entity<InputField>,
    web_url: Entity<InputField>,
    excluded_local: Entity<InputField>,
    excluded_remote: Entity<InputField>,
    mappings: Vec<[Entity<InputField>; 3]>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tab {
    Connection,
    Mappings,
    Excluded,
}

pub(crate) struct ServersModal {
    root: PathBuf,
    draft: Draft,
    selected: Option<usize>,
    form: Option<Form>,
    tab: Tab,
    engine: Option<Arc<Engine>>,
    prompter: Arc<dyn Prompter>,
    credentials: Arc<Credentials>,
    test_results: Option<Vec<TestStep>>,
    testing: Option<Task<()>>,
    error: Option<SharedString>,
    focus_handle: FocusHandle,
}

impl ServersModal {
    pub(crate) fn new(
        root: PathBuf,
        engine: Option<Arc<Engine>>,
        prompter: Arc<dyn Prompter>,
        credentials: Arc<Credentials>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let draft = Draft::load(&ConfigStore::load(&root));
        let mut this = Self {
            root,
            draft,
            selected: None,
            form: None,
            tab: Tab::Connection,
            engine,
            prompter,
            credentials,
            test_results: None,
            testing: None,
            error: None,
            focus_handle: cx.focus_handle(),
        };
        if !this.draft.servers.is_empty() {
            this.select(0, window, cx);
        }
        this
    }

    fn select(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.commit_form(cx);
        self.selected = Some(index);
        self.test_results = None;
        self.form = self
            .draft
            .servers
            .get(index)
            .map(|resolved| new_form(&resolved.server, window, cx));
        cx.notify();
    }

    fn selected_server(&self) -> Option<&Server> {
        self.draft.servers.get(self.selected?).map(|s| &s.server)
    }

    fn selected_server_mut(&mut self) -> Option<&mut Server> {
        let index = self.selected?;
        self.draft.servers.get_mut(index).map(|s| &mut s.server)
    }

    /// Copies the form's text into the draft.
    fn commit_form(&mut self, cx: &App) {
        let Some(form) = &self.form else {
            return;
        };
        let text = |field: &Entity<InputField>| field.read(cx).text(cx).trim().to_string();
        let lines = |field: &Entity<InputField>| {
            text(field)
                .lines()
                .map(str::trim)
                .filter(|line| !line.is_empty())
                .map(str::to_string)
                .collect::<Vec<_>>()
        };
        let name = text(&form.name);
        let host = text(&form.host);
        let port = text(&form.port).parse::<u16>().ok();
        let username = text(&form.username);
        let key_path = text(&form.key_path);
        let root_path = text(&form.root_path);
        let web_url = text(&form.web_url);
        let excluded_local = lines(&form.excluded_local);
        let excluded_remote = lines(&form.excluded_remote);
        let mappings: Vec<Mapping> = form
            .mappings
            .iter()
            .map(|[local, remote, web]| Mapping {
                local: text(local),
                remote: text(remote),
                web: text(web),
            })
            .filter(|mapping| !(mapping.local.is_empty() && mapping.remote.is_empty()))
            .collect();

        let Some(server) = self.selected_server_mut() else {
            return;
        };
        if !name.is_empty() {
            server.name = name;
        }
        server.host = host;
        server.port = port;
        server.username = username;
        server.private_key_path = (!key_path.is_empty()).then_some(key_path);
        server.root_path = root_path;
        server.web_url = web_url;
        server.excluded_local = excluded_local;
        server.excluded_remote = excluded_remote;
        server.mappings = mappings;
    }

    fn add_server(&mut self, kind: ServerType, window: &mut Window, cx: &mut Context<Self>) {
        self.commit_form(cx);
        let store = ConfigStore::load(&self.root);
        let name = format!("New {} server", kind.label());
        let mut id = store.new_server_id(&name);
        while self.draft.servers.iter().any(|s| s.server.id == id) {
            id.push('-');
            id.push('1');
        }
        let mut server = Server::new(id.clone(), name, kind);
        server.mappings = vec![Mapping {
            local: String::new(),
            remote: String::new(),
            web: String::new(),
        }];
        self.draft.servers.push(ResolvedServer {
            server,
            shared: false,
        });
        if self.draft.default_id.is_none() {
            self.draft.default_id = Some(id);
        }
        let index = self.draft.servers.len() - 1;
        self.form = None;
        self.select(index, window, cx);
    }

    fn remove_selected(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(index) = self.selected else {
            return;
        };
        self.form = None;
        let removed = self.draft.servers.remove(index);
        if self.draft.default_id.as_deref() == Some(removed.server.id.as_str()) {
            self.draft.default_id = self.draft.servers.first().map(|s| s.server.id.clone());
        }
        self.selected = None;
        if !self.draft.servers.is_empty() {
            self.select(index.min(self.draft.servers.len() - 1), window, cx);
        }
        cx.notify();
    }

    fn make_default(&mut self, cx: &mut Context<Self>) {
        if let Some(server) = self.selected_server() {
            self.draft.default_id = Some(server.id.clone());
            cx.notify();
        }
    }

    fn set_kind(&mut self, kind: ServerType, cx: &mut Context<Self>) {
        self.commit_form(cx);
        if let Some(server) = self.selected_server_mut() {
            if server.port == server.kind.default_port() {
                server.port = kind.default_port();
            }
            server.kind = kind;
            if kind == ServerType::Sftp && server.auth.is_none() {
                server.auth = Some(SftpAuth::Password);
            }
        }
        cx.notify();
    }

    fn set_auth(&mut self, auth: SftpAuth, cx: &mut Context<Self>) {
        self.commit_form(cx);
        if let Some(server) = self.selected_server_mut() {
            server.auth = Some(auth);
        }
        cx.notify();
    }

    fn add_mapping(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(form) = &mut self.form {
            form.mappings
                .push(mapping_fields(&Mapping::default(), window, cx));
        }
        cx.notify();
    }

    fn remove_mapping(&mut self, index: usize, cx: &mut Context<Self>) {
        if let Some(form) = &mut self.form
            && index < form.mappings.len()
        {
            form.mappings.remove(index);
        }
        cx.notify();
    }

    /// The password typed into the form, stored for this session (and in the
    /// keychain on Save) so a connection test can use it.
    fn take_password(&self, cx: &App) -> Option<(String, String)> {
        let form = self.form.as_ref()?;
        let password = form.password.read(cx).text(cx);
        let server = self.selected_server()?;
        (!password.is_empty()).then(|| (password_account(server), password))
    }

    fn test_connection(&mut self, cx: &mut Context<Self>) {
        self.commit_form(cx);
        let Some(server) = self.selected_server().cloned() else {
            return;
        };
        let password = self.take_password(cx);
        let credentials = self.credentials.clone();
        let engine = self
            .engine
            .clone()
            .unwrap_or_else(|| Engine::new(&self.root, self.prompter.clone(), credentials.clone()));
        self.test_results = None;
        let task = Tokio::spawn(cx, async move {
            if let Some((account, password)) = password {
                credentials.set(&account, &password, false).await;
            }
            engine.test_connection(&server).await
        });
        self.testing = Some(cx.spawn(async move |this, cx| {
            let steps = task.await.log_err();
            this.update(cx, |this, cx| {
                this.testing = None;
                this.test_results = steps;
                cx.notify();
            })
            .ok();
        }));
        cx.notify();
    }

    fn save(&mut self, cx: &mut Context<Self>) {
        self.commit_form(cx);
        let mut store = ConfigStore::load(&self.root);
        if let Err(error) = self.draft.apply(&mut store) {
            self.error = Some(format!("Couldn't save: {error}").into());
            cx.notify();
            return;
        }
        if let Some((account, password)) = self.take_password(cx) {
            let credentials = self.credentials.clone();
            Tokio::spawn(cx, async move {
                credentials.set(&account, &password, true).await;
            })
            .detach();
        }
        if let Some(engine) = &self.engine {
            engine.reload_config();
        }
        cx.emit(DismissEvent);
    }

    fn render_server_list(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let this = cx.weak_entity();
        let add_menu = ContextMenu::build(window, cx, move |mut menu, _, _| {
            for kind in ADDABLE_TYPES {
                let this = this.clone();
                menu = menu.entry(kind.label(), None, move |window, cx| {
                    this.update(cx, |this, cx| this.add_server(kind, window, cx))
                        .ok();
                });
            }
            menu
        });

        v_flex()
            .w(rems(14.))
            .h_full()
            .gap_1()
            .pr_2()
            .border_r_1()
            .border_color(cx.theme().colors().border_variant)
            .children(
                self.draft
                    .servers
                    .iter()
                    .enumerate()
                    .map(|(index, resolved)| {
                        let server = &resolved.server;
                        let is_default =
                            self.draft.default_id.as_deref() == Some(server.id.as_str());
                        ListItemButton::new(
                            index,
                            server.name.clone(),
                            server.kind.label(),
                            is_default,
                        )
                        .selected(self.selected == Some(index))
                        .on_click(
                            cx.listener(move |this, _, window, cx| this.select(index, window, cx)),
                        )
                    }),
            )
            .child(
                h_flex()
                    .mt_2()
                    .gap_1()
                    .child(DropdownMenu::new("add-server", "Add", add_menu))
                    .child(
                        Button::new("remove-server", "Remove")
                            .style(ButtonStyle::Subtle)
                            .disabled(self.selected.is_none())
                            .on_click(
                                cx.listener(|this, _, window, cx| this.remove_selected(window, cx)),
                            ),
                    ),
            )
    }

    fn render_connection(
        &self,
        server: &Server,
        form: &Form,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let this = cx.weak_entity();
        let kind_menu = ContextMenu::build(window, cx, {
            let this = this.clone();
            move |mut menu, _, _| {
                for kind in ADDABLE_TYPES {
                    let this = this.clone();
                    menu = menu.entry(kind.label(), None, move |_, cx| {
                        this.update(cx, |this, cx| this.set_kind(kind, cx)).ok();
                    });
                }
                menu
            }
        });
        let is_remote = !matches!(server.kind, ServerType::Local);
        let is_sftp = server.kind == ServerType::Sftp;
        let auth = server.auth.unwrap_or_default();
        let auth_label = SFTP_AUTH
            .iter()
            .find(|(value, _)| *value == auth)
            .map_or("Password", |(_, label)| label);
        let auth_menu = ContextMenu::build(window, cx, move |mut menu, _, _| {
            for (value, label) in SFTP_AUTH {
                let this = this.clone();
                menu = menu.entry(label, None, move |_, cx| {
                    this.update(cx, |this, cx| this.set_auth(value, cx)).ok();
                });
            }
            menu
        });
        let needs_password = !is_sftp || auth == SftpAuth::Password;

        v_flex()
            .gap_2()
            .child(form.name.clone())
            .child(
                h_flex()
                    .gap_2()
                    .child(Label::new("Type").color(Color::Muted))
                    .child(DropdownMenu::new(
                        "server-type",
                        server.kind.label(),
                        kind_menu,
                    )),
            )
            .when(is_remote, |column| {
                column
                    .child(
                        h_flex()
                            .gap_2()
                            .child(div().flex_1().child(form.host.clone()))
                            .child(div().w(rems(6.)).child(form.port.clone())),
                    )
                    .child(form.username.clone())
                    .when(is_sftp, |column| {
                        column.child(
                            h_flex()
                                .gap_2()
                                .child(Label::new("Authentication").color(Color::Muted))
                                .child(DropdownMenu::new("sftp-auth", auth_label, auth_menu)),
                        )
                    })
                    .when(is_sftp && auth == SftpAuth::Key, |column| {
                        column.child(form.key_path.clone())
                    })
                    .when(needs_password, |column| column.child(form.password.clone()))
            })
            .child(form.root_path.clone())
            .child(form.web_url.clone())
            .into_any_element()
    }

    fn render_mappings(&self, form: &Form, cx: &mut Context<Self>) -> AnyElement {
        v_flex()
            .gap_2()
            .child(
                Label::new(
                    "Local paths are relative to the project; remote paths to the server's root path.",
                )
                .size(LabelSize::Small)
                .color(Color::Muted),
            )
            .children(form.mappings.iter().enumerate().map(|(index, [local, remote, web])| {
                h_flex()
                    .gap_2()
                    .items_end()
                    .child(div().flex_1().child(local.clone()))
                    .child(div().flex_1().child(remote.clone()))
                    .child(div().flex_1().child(web.clone()))
                    .child(
                        IconButton::new(("remove-mapping", index), IconName::Trash)
                            .icon_size(IconSize::Small)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.remove_mapping(index, cx)
                            })),
                    )
            }))
            .child(
                Button::new("add-mapping", "Add Mapping")
                    .style(ButtonStyle::Subtle)
                    .start_icon(Icon::new(IconName::Plus).size(IconSize::Small))
                    .on_click(cx.listener(|this, _, window, cx| this.add_mapping(window, cx))),
            )
            .into_any_element()
    }

    fn render_excluded(&self, form: &Form) -> AnyElement {
        v_flex()
            .gap_2()
            .child(
                Label::new(
                    "One path or glob per line. Excluded paths are never uploaded or downloaded.",
                )
                .size(LabelSize::Small)
                .color(Color::Muted),
            )
            .child(form.excluded_local.clone())
            .child(form.excluded_remote.clone())
            .into_any_element()
    }

    fn render_test_results(&self) -> Option<impl IntoElement> {
        if self.testing.is_some() {
            return Some(
                Label::new("Testing the connection…")
                    .color(Color::Muted)
                    .into_any_element(),
            );
        }
        let steps = self.test_results.as_ref()?;
        Some(
            v_flex()
                .gap_1()
                .children(steps.iter().map(|step| {
                    let (icon, color) = match step.state {
                        StepState::Ok => (IconName::Check, Color::Success),
                        StepState::Warning => (IconName::Warning, Color::Warning),
                        StepState::Failed => (IconName::XCircle, Color::Error),
                        StepState::Skipped => (IconName::Dash, Color::Muted),
                    };
                    h_flex()
                        .gap_2()
                        .child(Icon::new(icon).size(IconSize::Small).color(color))
                        .child(Label::new(step.label.clone()))
                        .when(!step.detail.is_empty(), |row| {
                            row.child(
                                Label::new(step.detail.clone())
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                        })
                }))
                .into_any_element(),
        )
    }
}

fn input(
    placeholder: &str,
    label: &str,
    text: &str,
    window: &mut Window,
    cx: &mut App,
) -> Entity<InputField> {
    cx.new(|cx| {
        let field = InputField::new(window, cx, placeholder).label(label.to_string());
        field.set_text(text, window, cx);
        field
    })
}

fn mapping_fields(mapping: &Mapping, window: &mut Window, cx: &mut App) -> [Entity<InputField>; 3] {
    [
        input("src", "Local path", &mapping.local, window, cx),
        input("/var/www/site", "Remote path", &mapping.remote, window, cx),
        input("/", "Web path", &mapping.web, window, cx),
    ]
}

fn new_form(server: &Server, window: &mut Window, cx: &mut App) -> Form {
    let multiline = |label: &str, lines: &[String], window: &mut Window, cx: &mut App| {
        let field = input("", label, &lines.join("\n"), window, cx);
        field.update(cx, |field, cx| {
            field.editor().set_multiline(Some(6), window, cx)
        });
        field
    };
    let port = server.port.map(|port| port.to_string()).unwrap_or_default();
    Form {
        name: input("My server", "Name", &server.name, window, cx),
        host: input("example.com", "Host", &server.host, window, cx),
        port: input(
            &server
                .kind
                .default_port()
                .map(|p| p.to_string())
                .unwrap_or_default(),
            "Port",
            &port,
            window,
            cx,
        ),
        username: input("deploy", "Username", &server.username, window, cx),
        password: cx.new(|cx| {
            InputField::new(window, cx, "Leave empty to keep the saved password")
                .label("Password")
                .masked(true)
        }),
        key_path: input(
            "~/.ssh/id_ed25519",
            "Private key",
            server.private_key_path.as_deref().unwrap_or_default(),
            window,
            cx,
        ),
        root_path: input("/var/www", "Root path", &server.root_path, window, cx),
        web_url: input(
            "https://example.com",
            "Web server URL",
            &server.web_url,
            window,
            cx,
        ),
        excluded_local: multiline("Excluded local paths", &server.excluded_local, window, cx),
        excluded_remote: multiline("Excluded remote paths", &server.excluded_remote, window, cx),
        mappings: server
            .mappings
            .iter()
            .map(|mapping| mapping_fields(mapping, window, cx))
            .collect(),
    }
}

/// A server row in the list: name, type, and ★ for the default.
#[derive(IntoElement)]
struct ListItemButton {
    index: usize,
    name: String,
    kind: &'static str,
    is_default: bool,
    selected: bool,
    on_click: Option<Box<dyn Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl ListItemButton {
    fn new(index: usize, name: String, kind: &'static str, is_default: bool) -> Self {
        Self {
            index,
            name,
            kind,
            is_default,
            selected: false,
            on_click: None,
        }
    }

    fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    fn on_click(
        mut self,
        handler: impl Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for ListItemButton {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        ListItem::new(("deployment-server", self.index))
            .inset(true)
            .spacing(ListItemSpacing::Sparse)
            .toggle_state(self.selected)
            .when_some(self.on_click, |item, on_click| item.on_click(on_click))
            .child(
                h_flex()
                    .w_full()
                    .gap_1()
                    .child(Label::new(self.name).truncate())
                    .when(self.is_default, |row| {
                        row.child(
                            Icon::new(IconName::Star)
                                .size(IconSize::XSmall)
                                .color(Color::Accent),
                        )
                    })
                    .child(div().flex_1())
                    .child(
                        Label::new(self.kind)
                            .size(LabelSize::XSmall)
                            .color(Color::Muted),
                    ),
            )
    }
}

impl Focusable for ServersModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for ServersModal {}
impl ModalView for ServersModal {}

impl Render for ServersModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let server = self.selected_server().cloned();
        let is_default = server
            .as_ref()
            .is_some_and(|s| self.draft.default_id.as_deref() == Some(s.id.as_str()));
        let tab_button =
            |id: &'static str, label: &'static str, tab: Tab, cx: &mut Context<Self>| {
                Button::new(id, label)
                    .style(ButtonStyle::Subtle)
                    .toggle_state(self.tab == tab)
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.tab = tab;
                        cx.notify();
                    }))
            };
        let tabs = h_flex()
            .gap_1()
            .child(tab_button(
                "tab-connection",
                "Connection",
                Tab::Connection,
                cx,
            ))
            .child(tab_button("tab-mappings", "Mappings", Tab::Mappings, cx))
            .child(tab_button(
                "tab-excluded",
                "Excluded Paths",
                Tab::Excluded,
                cx,
            ));

        let body = match (&server, self.form.take()) {
            (Some(server), Some(form)) => {
                let body = match self.tab {
                    Tab::Connection => self.render_connection(server, &form, window, cx),
                    Tab::Mappings => self.render_mappings(&form, cx),
                    Tab::Excluded => self.render_excluded(&form),
                };
                self.form = Some(form);
                body
            }
            _ => Label::new("Add a server to deploy this project to.")
                .color(Color::Muted)
                .into_any_element(),
        };

        v_flex()
            .key_context("DeploymentServers")
            .track_focus(&self.focus_handle)
            .w(rems(52.))
            .max_h(rems(40.))
            .p_3()
            .gap_3()
            .elevation_3(cx)
            .on_action(cx.listener(|_, _: &menu::Cancel, _, cx| cx.emit(DismissEvent)))
            .child(Headline::new("Deployment Servers").size(HeadlineSize::Small))
            .child(
                h_flex()
                    .items_start()
                    .gap_3()
                    .min_h(rems(24.))
                    .child(self.render_server_list(window, cx))
                    .child(
                        v_flex()
                            .flex_1()
                            .gap_3()
                            .when(server.is_some(), |column| column.child(tabs))
                            .child(div().id("server-details").overflow_y_scroll().child(body))
                            .children(self.render_test_results()),
                    ),
            )
            .when_some(self.error.clone(), |column, error| {
                column.child(Label::new(error).color(Color::Error))
            })
            .child(
                h_flex()
                    .justify_between()
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Button::new("test-connection", "Test Connection")
                                    .disabled(server.is_none() || self.testing.is_some())
                                    .on_click(
                                        cx.listener(|this, _, _, cx| this.test_connection(cx)),
                                    ),
                            )
                            .child(
                                Button::new("make-default", "Use as Default")
                                    .style(ButtonStyle::Subtle)
                                    .disabled(server.is_none() || is_default)
                                    .on_click(cx.listener(|this, _, _, cx| this.make_default(cx))),
                            ),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                Button::new("cancel-servers", "Cancel")
                                    .on_click(cx.listener(|_, _, _, cx| cx.emit(DismissEvent))),
                            )
                            .child(
                                Button::new("save-servers", "Save")
                                    .style(ButtonStyle::Filled)
                                    .on_click(cx.listener(|this, _, _, cx| this.save(cx))),
                            ),
                    ),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store(dir: &std::path::Path) -> ConfigStore {
        ConfigStore::load_with(dir, dir.join("servers.json"))
    }

    fn server(id: &str) -> ResolvedServer {
        let mut server = Server::new(id, id.to_uppercase(), ServerType::Sftp);
        server.host = format!("{id}.example.com");
        ResolvedServer {
            server,
            shared: false,
        }
    }

    #[test]
    fn saving_a_draft_adds_edits_removes_and_sets_the_default() {
        let dir = tempfile::tempdir().unwrap();
        let mut config = store(dir.path());
        config.upsert_server(server("a").server, false).unwrap();
        config.upsert_server(server("b").server, false).unwrap();
        config.set_default_server(Some("a")).unwrap();

        let mut draft = Draft::load(&config);
        draft.servers.retain(|s| s.server.id != "a");
        draft.servers[0].server.host = "changed.example.com".into();
        draft.servers.push(server("c"));
        draft.default_id = Some("c".into());
        draft.apply(&mut config).unwrap();

        let reloaded = store(dir.path());
        let ids: Vec<_> = reloaded
            .servers()
            .into_iter()
            .map(|s| s.server.id)
            .collect();
        assert_eq!(ids, ["b", "c"]);
        assert_eq!(
            reloaded.server("b").unwrap().server.host,
            "changed.example.com"
        );
        assert_eq!(reloaded.default_server_id(), Some("c"));
    }

    #[test]
    fn removing_the_default_picks_another() {
        let dir = tempfile::tempdir().unwrap();
        let mut config = store(dir.path());
        config.upsert_server(server("a").server, false).unwrap();
        config.upsert_server(server("b").server, false).unwrap();
        config.set_default_server(Some("a")).unwrap();

        let mut draft = Draft::load(&config);
        draft.servers.retain(|s| s.server.id != "a");
        draft.apply(&mut config).unwrap();

        assert_eq!(store(dir.path()).default_server_id(), Some("b"));
    }
}
