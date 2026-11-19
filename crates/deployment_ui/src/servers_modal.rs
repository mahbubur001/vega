//! Deployment Servers (D2): a PhpStorm-style "Deployment" dialog to add, edit,
//! test and remove deployment servers without editing JSON. The server list is
//! on the left (+ / − / ✓ default, which shows in bold); the selected server's
//! Connection (with a collapsible Advanced section), Mappings and Excluded
//! Paths tabs are on the right.
//!
//! Edits stay in a draft until Save, which writes them through the
//! deployment `ConfigStore`. The `DeploymentStore` notices the config file
//! change and (re)loads the project's engine.

use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use deployment::config::{
    ConfigStore, DataProtection, HostKeyPolicy, Mapping, ResolvedServer, Server, ServerType,
    SftpAuth, SshBackend, WebdavAuth,
};
use deployment::credentials::{Credentials, password_account};
use deployment::engine::Engine;
use deployment::manager::{StepState, TestStep};
use deployment::prompt::Prompter;
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, Render, Task,
    WeakEntity, Window,
};
use gpui_tokio::Tokio;
use ui::{
    Callout, Checkbox, CommonAnimationExt as _, ContextMenu, ContextMenuEntry, Disclosure,
    DropdownMenu, IconPosition, ListItem, ListItemSpacing, PopoverMenu, Severity, ToggleState,
    Tooltip, prelude::*,
};
use ui_input::InputField;
use util::ResultExt as _;
use workspace::ModalView;

use crate::dialog;

/// The server types offered when adding a server, in PhpStorm's order.
const ADDABLE_TYPES: [ServerType; 8] = [
    ServerType::Sftp,
    ServerType::Ftp,
    ServerType::Ftps,
    ServerType::FtpsImplicit,
    ServerType::Webdav,
    ServerType::Local,
    ServerType::InPlace,
    ServerType::Group,
];

/// The types a server's Type dropdown can switch between (groups stay groups).
const SWITCHABLE_TYPES: [ServerType; 7] = [
    ServerType::Sftp,
    ServerType::Ftp,
    ServerType::Ftps,
    ServerType::FtpsImplicit,
    ServerType::Webdav,
    ServerType::Local,
    ServerType::InPlace,
];

const SFTP_AUTH: [(SftpAuth, &str); 4] = [
    (SftpAuth::Password, "Password"),
    (SftpAuth::Key, "Key pair"),
    (SftpAuth::Agent, "SSH agent"),
    (SftpAuth::Openssh, "OpenSSH config"),
];

const HOST_KEY_POLICIES: [(HostKeyPolicy, &str); 4] = [
    (HostKeyPolicy::Ask, "Ask for unknown hosts"),
    (HostKeyPolicy::AcceptNew, "Accept new hosts"),
    (HostKeyPolicy::Yes, "Only known hosts"),
    (HostKeyPolicy::No, "Don't check (unsafe)"),
];

const SSH_BACKENDS: [(SshBackend, &str); 2] = [
    (SshBackend::Builtin, "Built-in"),
    (SshBackend::Openssh, "System OpenSSH"),
];

const DATA_PROTECTION: [(DataProtection, &str); 2] = [
    (DataProtection::Private, "Private (encrypted)"),
    (DataProtection::Clear, "Clear"),
];

const WEBDAV_AUTH: [(WebdavAuth, &str); 3] = [
    (WebdavAuth::Basic, "Basic"),
    (WebdavAuth::Digest, "Digest"),
    (WebdavAuth::None, "None"),
];

/// FTP file name encodings the engine understands (UTF-8 is the default).
const ENCODINGS: [(Option<&str>, &str); 2] = [
    (None, "UTF-8"),
    (Some("ISO-8859-1"), "ISO-8859-1 (Latin-1)"),
];

fn encoding_choice(encoding: Option<&str>) -> Option<&'static str> {
    let latin1 = encoding.is_some_and(|encoding| {
        matches!(
            encoding.trim().to_ascii_lowercase().as_str(),
            "latin1" | "latin-1" | "iso-8859-1" | "iso8859-1"
        )
    });
    latin1.then_some("ISO-8859-1")
}

fn choice_label<T: PartialEq>(options: &[(T, &'static str)], value: &T) -> &'static str {
    options
        .iter()
        .find(|(option, _)| option == value)
        .map_or("", |(_, label)| label)
}

/// An optional text setting: empty means unset.
fn optional_text(text: String) -> Option<String> {
    (!text.is_empty()).then_some(text)
}

/// An optional count: empty or invalid means unset (the default).
fn optional_count(text: &str) -> Option<u32> {
    text.parse::<u32>().ok().filter(|count| *count > 0)
}

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
    /// Excluded paths, each local (project) or on the server.
    excluded: Vec<(ExcludedSide, Entity<InputField>)>,
    mappings: Vec<[Entity<InputField>; 3]>,
    jump_host: Entity<InputField>,
    keep_alive_seconds: Entity<InputField>,
    keep_alive_command: Entity<InputField>,
    trusted_fingerprint: Entity<InputField>,
    max_connections: Entity<InputField>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tab {
    Connection,
    Mappings,
    Excluded,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ExcludedSide {
    Local,
    Deployment,
}

impl ExcludedSide {
    fn label(self) -> &'static str {
        match self {
            ExcludedSide::Local => "Local path",
            ExcludedSide::Deployment => "Deployment path",
        }
    }
}

/// Width of the labels in front of the fields.
const LABEL_WIDTH: f32 = 8.5;

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
    /// Whether the keychain has the selected server's password (None while checking).
    password_saved: Option<bool>,
    advanced_open: bool,
    /// Shown in the Deployment window rather than as a modal.
    hosted: bool,
    /// Engine for Test Connection and Autodetect (see [`Self::test_engine`]).
    tester: Option<Arc<Engine>>,
    error: Option<SharedString>,
    focus_handle: FocusHandle,
    _tasks: Vec<Task<()>>,
}

impl ServersModal {
    pub(crate) fn new(
        root: PathBuf,
        engine: Option<Arc<Engine>>,
        prompter: Arc<dyn Prompter>,
        credentials: Arc<Credentials>,
        hosted: bool,
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
            password_saved: None,
            advanced_open: false,
            hosted,
            tester: None,
            error: None,
            focus_handle: cx.focus_handle(),
            _tasks: Vec::new(),
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
        self.check_saved_password(cx);
        cx.notify();
    }

    fn check_saved_password(&mut self, cx: &mut Context<Self>) {
        self.password_saved = None;
        let Some(account) = self.selected_server().map(password_account) else {
            return;
        };
        let credentials = self.credentials.clone();
        let lookup = Tokio::spawn(cx, async move { credentials.get(&account).await.is_some() });
        self._tasks.push(cx.spawn(async move |this, cx| {
            let saved = lookup.await.unwrap_or(false);
            this.update(cx, |this, cx| {
                this.password_saved = Some(saved);
                cx.notify();
            })
            .ok();
        }));
    }

    /// Forgets the saved password, so the next connection asks again.
    fn reset_password(&mut self, cx: &mut Context<Self>) {
        let Some(account) = self.selected_server().map(password_account) else {
            return;
        };
        let credentials = self.credentials.clone();
        Tokio::spawn(cx, async move { credentials.delete(&account).await }).detach();
        self.password_saved = Some(false);
        cx.notify();
    }

    /// Asks the server for its home folder and puts it in Root path.
    fn autodetect_root(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.commit_form(cx);
        let Some(server) = self.selected_server().cloned() else {
            return;
        };
        let password = self.take_password(cx);
        let credentials = self.credentials.clone();
        let engine = self.test_engine();
        let detect = Tokio::spawn(cx, async move {
            if let Some((account, password)) = password {
                credentials.set(&account, &password, false).await;
            }
            engine.detect_root(&server).await
        });
        self._tasks.push(cx.spawn_in(window, async move |this, cx| {
            let result = detect.await;
            this.update_in(cx, |this, window, cx| {
                match result {
                    Ok(Ok(root)) => {
                        if let Some(form) = &this.form {
                            form.root_path
                                .update(cx, |field, cx| field.set_text(&root, window, cx));
                        }
                        this.error = None;
                    }
                    Ok(Err(error)) => this.error = Some(format!("Autodetect: {error}").into()),
                    Err(error) => this.error = Some(format!("Autodetect: {error}").into()),
                }
                cx.notify();
            })
            .ok();
        }));
    }

    fn open_web_url(&mut self, cx: &mut Context<Self>) {
        let Some(form) = &self.form else {
            return;
        };
        let url = form.web_url.read(cx).text(cx);
        if url.starts_with("http://") || url.starts_with("https://") {
            cx.open_url(url.trim());
        }
    }

    /// Picks the local folder of mapping `index` (stored relative to the project).
    fn browse_local_path(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let paths = cx.prompt_for_paths(gpui::PathPromptOptions {
            files: false,
            directories: true,
            multiple: false,
            prompt: Some("Choose".into()),
        });
        let root = self.root.clone();
        self._tasks.push(cx.spawn_in(window, async move |this, cx| {
            let Ok(Ok(Some(paths))) = paths.await else {
                return;
            };
            let Some(path) = paths.into_iter().next() else {
                return;
            };
            let relative = path
                .strip_prefix(&root)
                .map(|relative| relative.to_string_lossy().into_owned())
                .unwrap_or_else(|_| path.to_string_lossy().into_owned());
            this.update_in(cx, |this, window, cx| {
                if let Some([local, _, _]) =
                    this.form.as_ref().and_then(|form| form.mappings.get(index))
                {
                    local.update(cx, |field, cx| field.set_text(&relative, window, cx));
                }
                cx.notify();
            })
            .ok();
        }));
    }

    fn add_excluded(&mut self, side: ExcludedSide, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(form) = &mut self.form {
            let placeholder = match side {
                ExcludedSide::Local => "storage/logs",
                ExcludedSide::Deployment => "/var/www/site/cache",
            };
            let field = input(placeholder, "", window, cx);
            window.focus(&field.focus_handle(cx), cx);
            form.excluded.push((side, field));
        }
        cx.notify();
    }

    fn remove_excluded(&mut self, index: usize, cx: &mut Context<Self>) {
        if let Some(form) = &mut self.form
            && index < form.excluded.len()
        {
            form.excluded.remove(index);
        }
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
        let name = text(&form.name);
        let host = text(&form.host);
        let port = text(&form.port).parse::<u16>().ok();
        let username = text(&form.username);
        let key_path = text(&form.key_path);
        let root_path = text(&form.root_path);
        let web_url = text(&form.web_url);
        let excluded = |side: ExcludedSide| {
            form.excluded
                .iter()
                .filter(|(field_side, _)| *field_side == side)
                .map(|(_, field)| text(field))
                .filter(|path| !path.is_empty())
                .collect::<Vec<_>>()
        };
        let excluded_local = excluded(ExcludedSide::Local);
        let excluded_remote = excluded(ExcludedSide::Deployment);
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
        let jump_host = optional_text(text(&form.jump_host));
        let keep_alive_seconds = optional_count(&text(&form.keep_alive_seconds));
        let keep_alive_command = optional_text(text(&form.keep_alive_command));
        let trusted_fingerprint = optional_text(text(&form.trusted_fingerprint));
        let max_connections = optional_count(&text(&form.max_connections));

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
        server.jump_host = jump_host;
        server.keep_alive_seconds = keep_alive_seconds;
        server.keep_alive_command = keep_alive_command;
        server.trusted_certificate_fingerprint = trusted_fingerprint;
        server.max_connections = max_connections;
    }

    /// Changes the selected server from a toggle or dropdown.
    fn update_server(&mut self, cx: &mut Context<Self>, update: impl FnOnce(&mut Server)) {
        self.commit_form(cx);
        if let Some(server) = self.selected_server_mut() {
            update(server);
        }
        cx.notify();
    }

    fn add_server(&mut self, kind: ServerType, window: &mut Window, cx: &mut Context<Self>) {
        self.commit_form(cx);
        let name = match kind {
            ServerType::Group => "New group".to_string(),
            _ => format!("New {} server", kind.label()),
        };
        let id = self.unique_id(&name);
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
        let engine = self.test_engine();
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

    /// The engine that tests connections and detects the root path. In the
    /// Deployment window it is a separate engine whose questions (passwords,
    /// host keys) are asked in that window.
    fn test_engine(&mut self) -> Arc<Engine> {
        if !self.hosted
            && let Some(engine) = &self.engine
        {
            return engine.clone();
        }
        self.tester
            .get_or_insert_with(|| {
                Engine::new(&self.root, self.prompter.clone(), self.credentials.clone())
            })
            .clone()
    }

    /// Writes the draft to the config (and a typed password to the keychain).
    /// Returns whether it worked.
    fn apply(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        self.commit_form(cx);
        let mut store = ConfigStore::load(&self.root);
        if let Err(error) = self.draft.apply(&mut store) {
            self.error = Some(format!("Couldn't save: {error}").into());
            cx.notify();
            return false;
        }
        self.error = None;
        if let Some((account, password)) = self.take_password(cx) {
            let credentials = self.credentials.clone();
            Tokio::spawn(cx, async move {
                credentials.set(&account, &password, true).await;
            })
            .detach();
            if let Some(form) = &self.form {
                form.password
                    .update(cx, |field, cx| field.clear(window, cx));
            }
            self.password_saved = Some(true);
        }
        if let Some(engine) = &self.engine {
            engine.reload_config();
        }
        if let Some(tester) = &self.tester {
            tester.reload_config();
        }
        cx.notify();
        true
    }

    fn save(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.apply(window, cx) {
            cx.emit(DismissEvent);
        }
    }

    /// Adds a copy of the selected server ("… copy"), selected.
    fn duplicate_selected(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.commit_form(cx);
        let Some(resolved) = self
            .selected
            .and_then(|index| self.draft.servers.get(index))
            .cloned()
        else {
            return;
        };
        let mut server = resolved.server;
        server.name = format!("{} copy", server.name);
        server.id = self.unique_id(&server.name);
        self.draft.servers.push(ResolvedServer {
            server,
            shared: resolved.shared,
        });
        let index = self.draft.servers.len() - 1;
        self.form = None;
        self.select(index, window, cx);
    }

    /// Focuses the Name field (PhpStorm's Rename).
    fn rename_selected(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.tab = Tab::Connection;
        if let Some(form) = &self.form {
            let focus = form.name.focus_handle(cx);
            window.focus(&focus, cx);
        }
        cx.notify();
    }

    fn unique_id(&self, name: &str) -> String {
        let store = ConfigStore::load(&self.root);
        let mut id = store.new_server_id(name);
        while self.draft.servers.iter().any(|s| s.server.id == id) {
            id.push_str("-1");
        }
        id
    }

    fn toggle_member(&mut self, member: String, cx: &mut Context<Self>) {
        self.update_server(cx, |server| {
            if let Some(index) = server.members.iter().position(|id| *id == member) {
                server.members.remove(index);
            } else {
                server.members.push(member);
            }
        });
    }

    fn render_server_list(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let this = cx.weak_entity();
        let selected_is_default = self
            .selected_server()
            .is_some_and(|server| self.draft.default_id.as_deref() == Some(server.id.as_str()));
        let toolbar = h_flex()
            .gap_0p5()
            .px_1()
            .pb_1()
            .child(
                PopoverMenu::new("add-server-menu")
                    .trigger_with_tooltip(
                        IconButton::new("add-server", IconName::Plus)
                            .icon_size(IconSize::Small)
                            .icon_color(Color::Accent),
                        Tooltip::text("Add Server"),
                    )
                    .anchor(gpui::Anchor::TopLeft)
                    .menu(move |window, cx| {
                        let this = this.clone();
                        Some(ContextMenu::build(window, cx, move |mut menu, _, _| {
                            for kind in ADDABLE_TYPES {
                                let this = this.clone();
                                let (icon, color) = server_type_icon(kind);
                                if kind == ServerType::Group {
                                    menu = menu.separator();
                                }
                                menu = menu.item(
                                    ContextMenuEntry::new(kind.label())
                                        .icon(icon)
                                        .icon_color(color)
                                        .icon_position(IconPosition::Start)
                                        .handler(move |window, cx| {
                                            this.update(cx, |this, cx| {
                                                this.add_server(kind, window, cx)
                                            })
                                            .ok();
                                        }),
                                );
                            }
                            menu
                        }))
                    }),
            )
            .child(
                IconButton::new("remove-server", IconName::Dash)
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Error)
                    .tooltip(Tooltip::text("Remove Server"))
                    .disabled(self.selected.is_none())
                    .on_click(cx.listener(|this, _, window, cx| this.remove_selected(window, cx))),
            )
            .child(
                IconButton::new("duplicate-server", IconName::Copy)
                    .icon_size(IconSize::Small)
                    .tooltip(Tooltip::text("Duplicate"))
                    .disabled(self.selected.is_none())
                    .on_click(
                        cx.listener(|this, _, window, cx| this.duplicate_selected(window, cx)),
                    ),
            )
            .child(
                div()
                    .w_px()
                    .h_4()
                    .mx_1()
                    .bg(cx.theme().colors().border_variant),
            )
            .child(
                IconButton::new("default-server", IconName::Check)
                    .icon_size(IconSize::Small)
                    .icon_color(Color::Success)
                    .tooltip(Tooltip::text("Use as Default"))
                    .disabled(self.selected.is_none() || selected_is_default)
                    .on_click(cx.listener(|this, _, _, cx| this.make_default(cx))),
            )
            .child(
                IconButton::new("rename-server", IconName::Pencil)
                    .icon_size(IconSize::Small)
                    .tooltip(Tooltip::text("Rename"))
                    .disabled(self.selected.is_none())
                    .on_click(cx.listener(|this, _, window, cx| this.rename_selected(window, cx))),
            );

        v_flex()
            .w(rems(15.))
            .h_full()
            .flex_none()
            .border_r_1()
            .border_color(cx.theme().colors().border_variant)
            .child(toolbar)
            .child(
                v_flex()
                    .id("deployment-server-list")
                    .flex_1()
                    .overflow_y_scroll()
                    .pr_1()
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
                                    server.kind,
                                    is_default,
                                )
                                .selected(self.selected == Some(index))
                                .on_click(cx.listener(
                                    move |this, _, window, cx| this.select(index, window, cx),
                                ))
                            }),
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
        if server.kind == ServerType::Group {
            return self.render_group(server, form, cx);
        }
        let this = cx.weak_entity();
        let kind_menu = ContextMenu::build(window, cx, {
            let this = this.clone();
            move |mut menu, _, _| {
                for kind in SWITCHABLE_TYPES {
                    let this = this.clone();
                    let (icon, color) = server_type_icon(kind);
                    menu = menu.item(
                        ContextMenuEntry::new(kind.label())
                            .icon(icon)
                            .icon_color(color)
                            .icon_position(IconPosition::Start)
                            .handler(move |_, cx| {
                                this.update(cx, |this, cx| this.set_kind(kind, cx)).ok();
                            }),
                    );
                }
                menu
            }
        });
        let is_remote = !matches!(server.kind, ServerType::Local | ServerType::InPlace);
        let in_place = server.kind == ServerType::InPlace;
        let is_sftp = server.kind == ServerType::Sftp;
        let is_ftp = server.kind.is_ftp();
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
        let needs_password = !(is_sftp && auth != SftpAuth::Password) && !server.anonymous;
        let shared = self
            .selected
            .and_then(|index| self.draft.servers.get(index))
            .is_some_and(|resolved| resolved.shared);
        let (kind_icon, kind_color) = server_type_icon(server.kind);

        v_flex()
            .gap_2()
            .child(
                Checkbox::new("visible-only-here", ToggleState::from(!shared))
                    .label("Visible only for this project")
                    .on_click(cx.listener(|this, state: &ToggleState, _, cx| {
                        let only_here = state.selected();
                        if let Some(resolved) = this
                            .selected
                            .and_then(|index| this.draft.servers.get_mut(index))
                        {
                            resolved.shared = !only_here;
                        }
                        cx.notify();
                    })),
            )
            .child(field_row("Name:", form.name.clone()))
            .child(field_row(
                "Type:",
                h_flex()
                    .gap_1p5()
                    .child(Icon::new(kind_icon).size(IconSize::Small).color(kind_color))
                    .child(DropdownMenu::new(
                        "server-type",
                        server.kind.label(),
                        kind_menu,
                    )),
            ))
            .when(is_remote, |column| {
                column
                    .child(field_row(
                        "Host:",
                        h_flex()
                            .gap_2()
                            .child(div().flex_1().child(form.host.clone()))
                            .child(Label::new("Port:"))
                            .child(div().w(rems(5.)).child(form.port.clone())),
                    ))
                    .child(field_row(
                        "Username:",
                        h_flex()
                            .gap_2()
                            .child(div().flex_1().child(form.username.clone()))
                            .when(is_ftp, |row| {
                                row.child(
                                    Checkbox::new(
                                        "ftp-anonymous",
                                        ToggleState::from(server.anonymous),
                                    )
                                    .label("Login as anonymous")
                                    .on_click(cx.listener(
                                        |this, state: &ToggleState, _, cx| {
                                            let checked = state.selected();
                                            this.update_server(cx, |server| {
                                                server.anonymous = checked
                                            });
                                        },
                                    )),
                                )
                            }),
                    ))
                    .when(is_sftp, |column| {
                        column.child(field_row(
                            "Authentication:",
                            DropdownMenu::new("sftp-auth", auth_label, auth_menu),
                        ))
                    })
                    .when(is_sftp && auth == SftpAuth::Key, |column| {
                        column.child(field_row("Private key:", form.key_path.clone()))
                    })
                    .when(needs_password, |column| {
                        if self.password_saved == Some(true) {
                            column.child(field_row(
                                "",
                                h_flex()
                                    .gap_2()
                                    .child(Label::new("Password saved permanently"))
                                    .child(
                                        Button::new("reset-password", "Reset")
                                            .style(ButtonStyle::Transparent)
                                            .color(Color::Accent)
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.reset_password(cx)
                                            })),
                                    ),
                            ))
                        } else {
                            column.child(field_row("Password:", form.password.clone()))
                        }
                    })
            })
            .when(!in_place, |column| {
                column
                    .child(field_row(
                        "",
                        v_flex()
                            .gap_1()
                            .items_start()
                            .child(
                                Button::new("test-connection", "Test Connection")
                                    .style(ButtonStyle::Outlined)
                                    .start_icon(
                                        Icon::new(IconName::ArrowRightLeft).size(IconSize::Small),
                                    )
                                    .disabled(self.testing.is_some())
                                    .on_click(
                                        cx.listener(|this, _, _, cx| this.test_connection(cx)),
                                    ),
                            )
                            .children(self.render_test_results(&server.host)),
                    ))
                    .child(div().h_2())
                    .child(field_row(
                        if is_remote { "Root path:" } else { "Folder:" },
                        h_flex()
                            .gap_2()
                            .child(div().flex_1().child(form.root_path.clone()))
                            .when(is_remote, |row| {
                                row.child(
                                    Button::new("autodetect-root", "Autodetect")
                                        .style(ButtonStyle::Outlined)
                                        .tooltip(Tooltip::text(
                                            "Use the folder the server logs you into",
                                        ))
                                        .on_click(cx.listener(|this, _, window, cx| {
                                            this.autodetect_root(window, cx)
                                        })),
                                )
                            }),
                    ))
            })
            .child(field_row(
                "Web server URL:",
                h_flex()
                    .gap_1()
                    .child(div().flex_1().child(form.web_url.clone()))
                    .child(
                        IconButton::new("open-web-url", IconName::ArrowUpRight)
                            .icon_size(IconSize::Small)
                            .tooltip(Tooltip::text("Open in Browser"))
                            .on_click(cx.listener(|this, _, _, cx| this.open_web_url(cx))),
                    ),
            ))
            .when(server.kind != ServerType::Local, |column| {
                column
                    .child(
                        h_flex()
                            .id("advanced-toggle")
                            .mt_3()
                            .gap_1()
                            .cursor_pointer()
                            .child(
                                Disclosure::new("advanced-disclosure", self.advanced_open)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.advanced_open = !this.advanced_open;
                                        cx.notify();
                                    })),
                            )
                            .child(Label::new("Advanced"))
                            .child(div().flex_1().h_px().bg(cx.theme().colors().border_variant))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.advanced_open = !this.advanced_open;
                                cx.notify();
                            })),
                    )
                    .when(self.advanced_open, |column| {
                        column.child(self.render_advanced(server, form, window, cx))
                    })
            })
            .into_any_element()
    }

    /// A server group: its name and which servers it deploys to.
    fn render_group(&self, server: &Server, form: &Form, cx: &mut Context<Self>) -> AnyElement {
        let candidates: Vec<(String, String, ServerType)> = self
            .draft
            .servers
            .iter()
            .map(|resolved| &resolved.server)
            .filter(|candidate| candidate.kind != ServerType::Group)
            .map(|candidate| (candidate.id.clone(), candidate.name.clone(), candidate.kind))
            .collect();
        let (icon, color) = server_type_icon(ServerType::Group);
        v_flex()
            .gap_2()
            .child(field_row("Name:", form.name.clone()))
            .child(field_row(
                "Type:",
                h_flex()
                    .gap_1p5()
                    .child(Icon::new(icon).size(IconSize::Small).color(color))
                    .child(Label::new("Server group")),
            ))
            .child(
                Label::new("Uploads and downloads to this group go to every server checked below.")
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .child(
                v_flex()
                    .gap_1()
                    .p_2()
                    .rounded_md()
                    .border_1()
                    .border_color(cx.theme().colors().border_variant)
                    .when(candidates.is_empty(), |list| {
                        list.child(Label::new("Add servers first.").color(Color::Muted))
                    })
                    .children(candidates.into_iter().enumerate().map(
                        |(index, (id, name, kind))| {
                            let (icon, color) = server_type_icon(kind);
                            let checked = server.members.contains(&id);
                            h_flex()
                                .gap_1p5()
                                .child(
                                    Checkbox::new(
                                        ("group-member", index),
                                        ToggleState::from(checked),
                                    )
                                    .on_click(cx.listener(
                                        move |this, _, _, cx| this.toggle_member(id.clone(), cx),
                                    )),
                                )
                                .child(Icon::new(icon).size(IconSize::Small).color(color))
                                .child(Label::new(name))
                        },
                    )),
            )
            .into_any_element()
    }

    fn render_mappings(&self, form: &Form, cx: &mut Context<Self>) -> AnyElement {
        let server = self.selected_server();
        let root = server
            .map(|server| server.root_path.clone())
            .filter(|root| !root.is_empty())
            .unwrap_or_else(|| "/".into());
        let web = server
            .map(|server| server.web_url.clone())
            .unwrap_or_default();
        let missing_deployment_path = form
            .mappings
            .iter()
            .any(|[_, remote, _]| remote.read(cx).text(cx).trim().is_empty());
        let several = form.mappings.len() > 1;
        v_flex()
            .gap_3()
            .children(form.mappings.iter().enumerate().map(|(index, [local, remote, web])| {
                v_flex()
                    .gap_2()
                    .when(several, |block| {
                        block.child(
                            h_flex()
                                .justify_between()
                                .child(
                                    Label::new(format!("Mapping {}", index + 1))
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                )
                                .child(
                                    IconButton::new(("remove-mapping", index), IconName::Trash)
                                        .icon_size(IconSize::Small)
                                        .tooltip(Tooltip::text("Remove Mapping"))
                                        .on_click(cx.listener(move |this, _, _, cx| {
                                            this.remove_mapping(index, cx)
                                        })),
                                ),
                        )
                    })
                    .child(field_row(
                        "Local path:",
                        h_flex()
                            .gap_1()
                            .child(div().flex_1().child(local.clone()))
                            .child(
                                IconButton::new(("browse-local", index), IconName::Folder)
                                    .icon_size(IconSize::Small)
                                    .tooltip(Tooltip::text("Choose Folder…"))
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        this.browse_local_path(index, window, cx)
                                    })),
                            ),
                    ))
                    .child(field_row("Deployment path:", remote.clone()))
                    .child(field_row("Web path:", web.clone()))
            }))
            .child(
                v_flex()
                    .child(
                        Label::new(format!(
                            "Local path is relative to the project. Deployment path is relative to the server root path ({root})."
                        ))
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                    )
                    .when(!web.is_empty(), |notes| {
                        notes.child(
                            Label::new(format!("Web path is relative to the web server URL ({web})."))
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        )
                    }),
            )
            .child(
                h_flex().child(
                    Button::new("add-mapping", "Add New Mapping")
                        .style(ButtonStyle::Outlined)
                        .on_click(cx.listener(|this, _, window, cx| this.add_mapping(window, cx))),
                ),
            )
            .when(missing_deployment_path, |column| {
                column.child(
                    h_flex()
                        .gap_1()
                        .child(Icon::new(IconName::Warning).size(IconSize::Small).color(Color::Warning))
                        .child(Label::new("Deployment path is not specified.").size(LabelSize::Small)),
                )
            })
            .into_any_element()
    }

    fn render_excluded(&self, form: &Form, cx: &mut Context<Self>) -> AnyElement {
        let this = cx.weak_entity();
        let toolbar = h_flex().gap_0p5().child(
            PopoverMenu::new("add-excluded-menu")
                .trigger_with_tooltip(
                    IconButton::new("add-excluded", IconName::Plus)
                        .icon_size(IconSize::Small)
                        .icon_color(Color::Accent),
                    Tooltip::text("Add Excluded Path"),
                )
                .anchor(gpui::Anchor::TopLeft)
                .menu(move |window, cx| {
                    let this = this.clone();
                    Some(ContextMenu::build(window, cx, move |menu, _, _| {
                        let add = |side: ExcludedSide| {
                            let this = this.clone();
                            move |window: &mut Window, cx: &mut App| {
                                this.update(cx, |this, cx| this.add_excluded(side, window, cx))
                                    .ok();
                            }
                        };
                        menu.entry("Local Path", None, add(ExcludedSide::Local))
                            .entry("Deployment Path", None, add(ExcludedSide::Deployment))
                    }))
                }),
        );
        let list = if form.excluded.is_empty() {
            v_flex()
                .flex_1()
                .items_center()
                .justify_center()
                .gap_1()
                .py_8()
                .child(Label::new("No excluded paths added.").color(Color::Muted))
                .child(
                    Button::new("add-first-excluded", "Add excluded path")
                        .style(ButtonStyle::Transparent)
                        .color(Color::Accent)
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.add_excluded(ExcludedSide::Local, window, cx)
                        })),
                )
                .into_any_element()
        } else {
            v_flex()
                .gap_2()
                .children(
                    form.excluded
                        .iter()
                        .enumerate()
                        .map(|(index, (side, field))| {
                            h_flex()
                                .gap_2()
                                .child(
                                    div()
                                        .w(rems(LABEL_WIDTH))
                                        .flex_none()
                                        .child(Label::new(side.label()).color(Color::Muted)),
                                )
                                .child(div().flex_1().child(field.clone()))
                                .child(
                                    IconButton::new(("remove-excluded", index), IconName::Dash)
                                        .icon_size(IconSize::Small)
                                        .icon_color(Color::Error)
                                        .tooltip(Tooltip::text("Remove"))
                                        .on_click(cx.listener(move |this, _, _, cx| {
                                            this.remove_excluded(index, cx)
                                        })),
                                )
                        }),
                )
                .into_any_element()
        };
        v_flex()
            .gap_2()
            .min_h(rems(16.))
            .child(toolbar)
            .child(list)
            .child(
                Label::new("Any matching file, directory or its child is excluded from download and upload.")
                    .size(LabelSize::Small)
                    .color(Color::Muted),
            )
            .into_any_element()
    }

    /// Options that apply to the selected server's type. FTP passive mode
    /// (always on) and rsync transfers aren't offered until the engine uses them.
    fn render_advanced(
        &self,
        server: &Server,
        form: &Form,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let kind = server.kind;
        let is_sftp = kind == ServerType::Sftp;
        let is_ftp = kind.is_ftp();
        let is_ftps = matches!(kind, ServerType::Ftps | ServerType::FtpsImplicit);
        let is_webdav = kind == ServerType::Webdav;
        let uses_tls = is_ftps || is_webdav;

        let this = cx.weak_entity();
        fn menu<T: Copy + 'static>(
            this: WeakEntity<ServersModal>,
            options: &'static [(T, &'static str)],
            set: fn(&mut Server, T),
            window: &mut Window,
            cx: &mut App,
        ) -> Entity<ContextMenu> {
            ContextMenu::build(window, cx, move |mut menu, _, _| {
                for (value, label) in options {
                    let this = this.clone();
                    let value = *value;
                    menu = menu.entry(*label, None, move |_, cx| {
                        this.update(cx, |this, cx| {
                            this.update_server(cx, |server| set(server, value))
                        })
                        .ok();
                    });
                }
                menu
            })
        }
        let checkbox = |id: &'static str,
                        label: &'static str,
                        checked: bool,
                        set: fn(&mut Server, bool),
                        cx: &mut Context<Self>| {
            Checkbox::new(id, ToggleState::from(checked))
                .label(label)
                .on_click(cx.listener(move |this, state: &ToggleState, _, cx| {
                    let checked = state.selected();
                    this.update_server(cx, |server| set(server, checked));
                }))
        };

        v_flex()
            .gap_2()
            .pl_1()
            .when(is_sftp, |column| {
                let policy = server.strict_host_key_checking.unwrap_or_default();
                let backend = server.ssh_backend.unwrap_or_default();
                column
                    .child(field_row(
                        "Host keys:",
                        DropdownMenu::new(
                            "host-key-policy",
                            choice_label(&HOST_KEY_POLICIES, &policy),
                            menu(
                                this.clone(),
                                &HOST_KEY_POLICIES,
                                |server, policy| server.strict_host_key_checking = Some(policy),
                                window,
                                cx,
                            ),
                        ),
                    ))
                    .child(field_row(
                        "SSH client:",
                        DropdownMenu::new(
                            "ssh-backend",
                            choice_label(&SSH_BACKENDS, &backend),
                            menu(
                                this.clone(),
                                &SSH_BACKENDS,
                                |server, backend| server.ssh_backend = Some(backend),
                                window,
                                cx,
                            ),
                        ),
                    ))
                    .child(field_row("Jump host:", form.jump_host.clone()))
                    .child(field_row(
                        "Keep-alive:",
                        h_flex()
                            .gap_2()
                            .child(div().w(rems(6.)).child(form.keep_alive_seconds.clone()))
                            .child(Label::new("seconds").color(Color::Muted)),
                    ))
            })
            .when(is_ftp, |column| {
                column
                    .child(checkbox(
                        "ftp-use-list",
                        "Use LIST instead of MLSD (older servers)",
                        server.use_list,
                        |server, checked| server.use_list = checked,
                        cx,
                    ))
                    .child(field_row(
                        "File names:",
                        DropdownMenu::new(
                            "ftp-encoding",
                            choice_label(&ENCODINGS, &encoding_choice(server.encoding.as_deref())),
                            menu(
                                this.clone(),
                                &ENCODINGS,
                                |server, encoding| server.encoding = encoding.map(str::to_string),
                                window,
                                cx,
                            ),
                        ),
                    ))
                    .child(field_row(
                        "Keep-alive command:",
                        form.keep_alive_command.clone(),
                    ))
            })
            .when(is_ftps, |column| {
                let protection = server.data_protection.unwrap_or_default();
                column.child(field_row(
                    "Data connection:",
                    DropdownMenu::new(
                        "data-protection",
                        choice_label(&DATA_PROTECTION, &protection),
                        menu(
                            this.clone(),
                            &DATA_PROTECTION,
                            |server, protection| server.data_protection = Some(protection),
                            window,
                            cx,
                        ),
                    ),
                ))
            })
            .when(is_webdav, |column| {
                let auth = server.webdav_auth.unwrap_or_default();
                column.child(field_row(
                    "Authentication:",
                    DropdownMenu::new(
                        "webdav-auth",
                        choice_label(&WEBDAV_AUTH, &auth),
                        menu(
                            this.clone(),
                            &WEBDAV_AUTH,
                            |server, auth| server.webdav_auth = Some(auth),
                            window,
                            cx,
                        ),
                    ),
                ))
            })
            .when(uses_tls, |column| {
                column
                    .child(checkbox(
                        "disable-tls13",
                        "Disable TLS 1.3 (for servers that fail the handshake)",
                        server.disable_tls13,
                        |server, checked| server.disable_tls13 = checked,
                        cx,
                    ))
                    .child(checkbox(
                        "allow-untrusted-certificate",
                        "Ask to trust certificates that aren't signed by a known authority",
                        server.allow_untrusted_certificate,
                        |server, checked| server.allow_untrusted_certificate = checked,
                        cx,
                    ))
                    .child(field_row(
                        "Trusted certificate:",
                        form.trusted_fingerprint.clone(),
                    ))
            })
            .child(field_row(
                "Number of connections:",
                div().w(rems(6.)).child(form.max_connections.clone()),
            ))
            .into_any_element()
    }

    fn render_test_results(&self, host: &str) -> Option<AnyElement> {
        if self.testing.is_some() {
            return Some(
                h_flex()
                    .gap_2()
                    .child(
                        Icon::new(IconName::ArrowCircle)
                            .size(IconSize::Small)
                            .color(Color::Muted)
                            .with_rotate_animation(2),
                    )
                    .child(Label::new("Testing the connection…").color(Color::Muted))
                    .into_any_element(),
            );
        }
        let steps = self.test_results.as_ref()?;
        let worst = steps
            .iter()
            .map(|step| step.state)
            .fold(StepState::Ok, |worst, state| match (worst, state) {
                (StepState::Failed, _) | (_, StepState::Failed) => StepState::Failed,
                (StepState::Warning, _) | (_, StepState::Warning) => StepState::Warning,
                _ => StepState::Ok,
            });
        let (severity, title) = match worst {
            StepState::Failed => (Severity::Error, "Couldn't connect".to_string()),
            StepState::Warning => (
                Severity::Warning,
                format!("Connected to {host}, with warnings"),
            ),
            _ => (
                Severity::Success,
                format!("Successfully connected to {host}"),
            ),
        };
        let details = v_flex().gap_0p5().children(steps.iter().map(|step| {
            let (icon, color) = match step.state {
                StepState::Ok => (IconName::Check, Color::Success),
                StepState::Warning => (IconName::Warning, Color::Warning),
                StepState::Failed => (IconName::XCircle, Color::Error),
                StepState::Skipped => (IconName::Dash, Color::Muted),
            };
            h_flex()
                .gap_1p5()
                .items_start()
                .child(Icon::new(icon).size(IconSize::XSmall).color(color))
                .child(
                    v_flex()
                        .child(Label::new(step.label.clone()).size(LabelSize::Small))
                        .when(!step.detail.is_empty(), |column| {
                            column.child(
                                Label::new(step.detail.clone())
                                    .size(LabelSize::XSmall)
                                    .color(Color::Muted),
                            )
                        }),
                )
        }));
        Some(
            Callout::new()
                .severity(severity)
                .title(title)
                .description_slot(details)
                .into_any_element(),
        )
    }
}

fn input(placeholder: &str, text: &str, window: &mut Window, cx: &mut App) -> Entity<InputField> {
    cx.new(|cx| {
        let field = InputField::new(window, cx, placeholder);
        field.set_text(text, window, cx);
        field
    })
}

/// A PhpStorm-style form row: the label on the left, the field filling the rest.
fn field_row(label: &'static str, field: impl IntoElement) -> impl IntoElement {
    h_flex()
        .gap_2()
        .child(
            div()
                .w(rems(LABEL_WIDTH))
                .flex_none()
                .child(Label::new(label)),
        )
        .child(div().flex_1().min_w_0().child(field))
}

/// The icon and color a server type gets in lists and menus.
pub(crate) fn server_type_icon(kind: ServerType) -> (IconName, Color) {
    match kind {
        ServerType::Sftp => (IconName::Send, Color::Warning),
        ServerType::Ftp => (IconName::Send, Color::Created),
        ServerType::Ftps | ServerType::FtpsImplicit => (IconName::Send, Color::Accent),
        ServerType::Webdav => (IconName::CloudDownload, Color::Warning),
        ServerType::Local | ServerType::InPlace => (IconName::Folder, Color::Warning),
        ServerType::Group => (IconName::Server, Color::Accent),
    }
}

fn mapping_fields(mapping: &Mapping, window: &mut Window, cx: &mut App) -> [Entity<InputField>; 3] {
    [
        input("src", &mapping.local, window, cx),
        input("/var/www/site", &mapping.remote, window, cx),
        input("/", &mapping.web, window, cx),
    ]
}

fn new_form(server: &Server, window: &mut Window, cx: &mut App) -> Form {
    let port = server.port.map(|port| port.to_string()).unwrap_or_default();
    Form {
        name: input("My server", &server.name, window, cx),
        host: input("example.com", &server.host, window, cx),
        port: input(
            &server
                .kind
                .default_port()
                .map(|p| p.to_string())
                .unwrap_or_default(),
            &port,
            window,
            cx,
        ),
        username: input("deploy", &server.username, window, cx),
        password: cx
            .new(|cx| InputField::new(window, cx, "Saved in the keychain on OK").masked(true)),
        key_path: input(
            "~/.ssh/id_ed25519",
            server.private_key_path.as_deref().unwrap_or_default(),
            window,
            cx,
        ),
        root_path: input("/var/www", &server.root_path, window, cx),
        web_url: input("https://example.com", &server.web_url, window, cx),
        excluded: server
            .excluded_local
            .iter()
            .map(|path| (ExcludedSide::Local, path))
            .chain(
                server
                    .excluded_remote
                    .iter()
                    .map(|path| (ExcludedSide::Deployment, path)),
            )
            .map(|(side, path)| (side, input("", path, window, cx)))
            .collect(),
        mappings: server
            .mappings
            .iter()
            .map(|mapping| mapping_fields(mapping, window, cx))
            .collect(),
        jump_host: input(
            "user@bastion.example.com:22",
            server.jump_host.as_deref().unwrap_or_default(),
            window,
            cx,
        ),
        keep_alive_seconds: input(
            "30",
            &server
                .keep_alive_seconds
                .map(|seconds| seconds.to_string())
                .unwrap_or_default(),
            window,
            cx,
        ),
        keep_alive_command: input(
            "NOOP",
            server.keep_alive_command.as_deref().unwrap_or_default(),
            window,
            cx,
        ),
        trusted_fingerprint: input(
            "SHA-256 fingerprint",
            server
                .trusted_certificate_fingerprint
                .as_deref()
                .unwrap_or_default(),
            window,
            cx,
        ),
        max_connections: input(
            &server.max_connections_or_default().to_string(),
            &server
                .max_connections
                .map(|count| count.to_string())
                .unwrap_or_default(),
            window,
            cx,
        ),
    }
}

/// A server row in the list: type icon and name, in bold for the default.
#[derive(IntoElement)]
struct ListItemButton {
    index: usize,
    name: String,
    kind: ServerType,
    is_default: bool,
    selected: bool,
    on_click: Option<Box<dyn Fn(&gpui::ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl ListItemButton {
    fn new(index: usize, name: String, kind: ServerType, is_default: bool) -> Self {
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
        let (icon, color) = server_type_icon(self.kind);
        ListItem::new(("deployment-server", self.index))
            .inset(true)
            .spacing(ListItemSpacing::Dense)
            .toggle_state(self.selected)
            .when_some(self.on_click, |item, on_click| item.on_click(on_click))
            .start_slot(Icon::new(icon).size(IconSize::Small).color(color))
            .child(
                Label::new(self.name)
                    .truncate()
                    .when(self.is_default, |label| {
                        label.weight(gpui::FontWeight::BOLD)
                    }),
            )
            .when(self.is_default, |item| {
                item.tooltip(Tooltip::text("Default server"))
            })
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
        let focus = self.focus_handle.clone();
        let tab_button =
            |id: &'static str, label: &'static str, tab: Tab, cx: &mut Context<Self>| {
                Button::new(id, label)
                    .style(if self.tab == tab {
                        ButtonStyle::Outlined
                    } else {
                        ButtonStyle::Subtle
                    })
                    .toggle_state(self.tab == tab)
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.tab = tab;
                        cx.notify();
                    }))
            };
        let tabs = h_flex()
            .gap_1()
            .pb_2()
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
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
                let tab = if server.kind == ServerType::Group {
                    Tab::Connection
                } else {
                    self.tab
                };
                let body = match tab {
                    Tab::Connection => self.render_connection(server, &form, window, cx),
                    Tab::Mappings => self.render_mappings(&form, cx),
                    Tab::Excluded => self.render_excluded(&form, cx),
                };
                self.form = Some(form);
                body
            }
            _ => v_flex()
                .flex_1()
                .items_center()
                .justify_center()
                .gap_1()
                .child(Label::new("No servers yet.").color(Color::Muted))
                .child(
                    Label::new("Click + to add one.")
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
                .into_any_element(),
        };

        v_flex()
            .key_context("DeploymentServers")
            .track_focus(&self.focus_handle)
            .map(|frame| {
                if self.hosted {
                    frame.size_full()
                } else {
                    frame.w(rems(56.)).h(rems(38.)).elevation_3(cx)
                }
            })
            .on_action(cx.listener(|_, _: &menu::Cancel, _, cx| cx.emit(DismissEvent)))
            .on_action(cx.listener(|this, _: &menu::Confirm, window, cx| this.save(window, cx)))
            .child(
                h_flex()
                    .px_3()
                    .border_b_1()
                    .border_color(cx.theme().colors().border_variant)
                    .map(|header| {
                        // In its own window on macOS this row is the title bar, next to the
                        // traffic lights.
                        if self.hosted && cfg!(target_os = "macos") {
                            header.h(px(38.)).justify_center()
                        } else {
                            header.py_2()
                        }
                    })
                    .child(Label::new("Deployment").weight(gpui::FontWeight::SEMIBOLD)),
            )
            .child(
                h_flex()
                    .flex_1()
                    .min_h_0()
                    .items_start()
                    .child(
                        div()
                            .h_full()
                            .pt_2()
                            .pl_2()
                            .child(self.render_server_list(cx)),
                    )
                    .child(
                        v_flex()
                            .flex_1()
                            .h_full()
                            .min_w_0()
                            .pt_2()
                            .px_3()
                            .gap_3()
                            .when(
                                server
                                    .as_ref()
                                    .is_some_and(|server| server.kind != ServerType::Group),
                                |column| column.child(tabs),
                            )
                            .child(
                                div()
                                    .id("server-details")
                                    .flex_1()
                                    .overflow_y_scroll()
                                    .pb_2()
                                    .child(body),
                            ),
                    ),
            )
            .child(
                h_flex()
                    .px_3()
                    .py_2()
                    .gap_2()
                    .border_t_1()
                    .border_color(cx.theme().colors().border_variant)
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .when_some(self.error.clone(), |row, error| {
                                row.child(Label::new(error).color(Color::Error).truncate())
                            }),
                    )
                    .child(
                        dialog::cancel_button("cancel-servers", &focus, cx)
                            .on_click(cx.listener(|_, _, _, cx| cx.emit(DismissEvent))),
                    )
                    .child(
                        Button::new("apply-servers", "Apply")
                            .style(ButtonStyle::Outlined)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.apply(window, cx);
                            })),
                    )
                    .child(
                        dialog::primary_button("save-servers", "OK", &focus, cx)
                            .on_click(cx.listener(|this, _, window, cx| this.save(window, cx))),
                    ),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_name_used_by_the_settings_page() {
        use gpui::Action as _;
        assert_eq!(
            zed_actions::deployment::OpenDeploymentServers.name(),
            "deployment::OpenDeploymentServers"
        );
    }

    #[test]
    fn empty_or_invalid_advanced_fields_mean_the_default() {
        assert_eq!(optional_text(String::new()), None);
        assert_eq!(optional_text("NOOP".into()), Some("NOOP".into()));
        assert_eq!(optional_count(""), None);
        assert_eq!(optional_count("0"), None);
        assert_eq!(optional_count("four"), None);
        assert_eq!(optional_count("4"), Some(4));
    }

    #[test]
    fn every_choice_has_a_label() {
        for (value, _) in HOST_KEY_POLICIES {
            assert!(!choice_label(&HOST_KEY_POLICIES, &value).is_empty());
        }
        assert_eq!(
            choice_label(&HOST_KEY_POLICIES, &HostKeyPolicy::default()),
            "Ask for unknown hosts"
        );
        assert_eq!(
            choice_label(&SSH_BACKENDS, &SshBackend::default()),
            "Built-in"
        );
        assert_eq!(
            choice_label(&DATA_PROTECTION, &DataProtection::default()),
            "Private (encrypted)"
        );
        assert!(!choice_label(&WEBDAV_AUTH, &WebdavAuth::default()).is_empty());
        assert_eq!(choice_label(&ENCODINGS, &encoding_choice(None)), "UTF-8");
        assert_eq!(
            choice_label(&ENCODINGS, &encoding_choice(Some("latin1"))),
            "ISO-8859-1 (Latin-1)"
        );
    }

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
