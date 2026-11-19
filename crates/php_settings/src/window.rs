//! The PHP settings window (PhpStorm's Settings › PHP): language level, CLI
//! interpreter, include paths, the stubs the language server loads, and
//! Composer options. A resizable window like Deployment.

use std::path::PathBuf;
use std::sync::Arc;

use fs::Fs;
use gpui::{
    App, AppContext as _, Context, Entity, FocusHandle, Focusable, Hsla, PromptLevel, Render, Size,
    Task, TitlebarOptions, WindowBounds, WindowKind, WindowOptions, point, px,
};
use platform_title_bar::PlatformTitleBar;
use release_channel::ReleaseChannel;
use settings::Settings as _;
use ui::{Checkbox, ContextMenu, Tooltip, prelude::*};
use util::ResultExt as _;
use vega_dialog::dialog;
use workspace::WorkspaceSettings;

use crate::config::{LANGUAGE_LEVELS, PhpConfig, composer_php_level};
use crate::interpreters::{PhpInfo, find_interpreters, php_info, short_version};
use crate::store::PhpSettingsStore;
use crate::stubs::{download_stubs, stub_folders};

const DEFAULT_SIZE: Size<Pixels> = Size {
    width: px(900.),
    height: px(640.),
};
const MIN_SIZE: Size<Pixels> = Size {
    width: px(700.),
    height: px(460.),
};
const LABEL_WIDTH: f32 = 11.;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tab {
    IncludePath,
    Runtime,
    Composer,
}

pub struct PhpSettingsWindow {
    root: PathBuf,
    project_name: String,
    fs: Arc<dyn Fs>,
    store: Entity<PhpSettingsStore>,
    draft: PhpConfig,
    saved: PhpConfig,
    /// Interpreters found on this computer, with their versions.
    interpreters: Vec<(PathBuf, String)>,
    /// The chosen interpreter's version and extensions.
    info: Option<PhpInfo>,
    composer_level: Option<String>,
    stub_names: Vec<String>,
    downloading: bool,
    tab: Tab,
    selected_include: Option<usize>,
    error: Option<SharedString>,
    title_bar: Option<Entity<PlatformTitleBar>>,
    focus_handle: FocusHandle,
    _tasks: Vec<Task<()>>,
}

/// Opens the PHP settings of the project folder `root`, or brings the open
/// window forward.
pub fn open(root: PathBuf, fs: Arc<dyn Fs>, cx: &mut App) {
    let existing = cx
        .windows()
        .into_iter()
        .find_map(|window| window.downcast::<PhpSettingsWindow>());
    if let Some(existing) = existing {
        let reused = existing
            .update(cx, |this, window, cx| {
                if this.root == root {
                    window.activate_window();
                    this.focus_handle.focus(window, cx);
                    true
                } else {
                    window.remove_window();
                    false
                }
            })
            .unwrap_or(false);
        if reused {
            return;
        }
    }
    cx.defer(move |cx| {
        let display_id = cx.active_display().map(|display| display.id());
        let window_decorations = match WorkspaceSettings::get_global(cx).window_decorations {
            settings::WindowDecorations::Server => gpui::WindowDecorations::Server,
            settings::WindowDecorations::Client => gpui::WindowDecorations::Client,
        };
        cx.open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("VEGA — PHP".into()),
                    appears_transparent: true,
                    traffic_light_position: Some(point(px(12.), px(12.))),
                }),
                focus: true,
                show: true,
                is_movable: true,
                is_resizable: true,
                is_minimizable: true,
                kind: WindowKind::Normal,
                window_background: cx.theme().window_background_appearance(),
                app_id: Some(ReleaseChannel::global(cx).app_id().to_owned()),
                window_decorations: Some(window_decorations),
                window_min_size: Some(MIN_SIZE),
                window_bounds: Some(WindowBounds::centered(display_id, DEFAULT_SIZE, cx)),
                display_id,
                ..Default::default()
            },
            |window, cx| {
                cx.new(|cx| {
                    let this = PhpSettingsWindow::new(root, fs, cx);
                    this.focus_handle.focus(window, cx);
                    this
                })
            },
        )
        .log_err();
    });
}

impl PhpSettingsWindow {
    fn new(root: PathBuf, fs: Arc<dyn Fs>, cx: &mut Context<Self>) -> Self {
        let store = PhpSettingsStore::global(cx);
        let saved = store.update(cx, |store, cx| store.config(&root, fs.clone(), cx));
        let project_name = root
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        let mut this = Self {
            root,
            project_name,
            fs,
            store: store.clone(),
            draft: saved.clone(),
            saved,
            interpreters: Vec::new(),
            info: None,
            composer_level: None,
            stub_names: Vec::new(),
            downloading: false,
            tab: Tab::IncludePath,
            selected_include: None,
            error: None,
            title_bar: (!cfg!(target_os = "macos"))
                .then(|| cx.new(|cx| PlatformTitleBar::new("php-settings-title-bar", cx))),
            focus_handle: cx.focus_handle(),
            _tasks: Vec::new(),
        };
        // The settings may still be loading; take them when they arrive, unless edited.
        cx.observe(&store, |this, store, cx| {
            let fs = this.fs.clone();
            let config = store.update(cx, |store, cx| store.config(&this.root, fs, cx));
            if this.draft == this.saved && config != this.saved {
                this.draft = config.clone();
                this.saved = config;
                this.refresh_interpreter(cx);
            }
        })
        .detach();
        this.load_environment(cx);
        this
    }

    /// Finds interpreters, stubs and composer.json's PHP requirement.
    fn load_environment(&mut self, cx: &mut Context<Self>) {
        let fs = self.fs.clone();
        let composer = self.root.join("composer.json");
        self._tasks.push(cx.spawn(async move |this, cx| {
            let interpreters = find_interpreters(fs.as_ref()).await;
            let stubs = stub_folders(fs.as_ref()).await;
            let composer_level = match fs.load(&composer).await {
                Ok(text) => composer_php_level(&text),
                Err(_) => None,
            };
            this.update(cx, |this, cx| {
                this.interpreters = interpreters;
                this.stub_names = stubs.into_iter().map(|(name, _)| name).collect();
                this.composer_level = composer_level;
                cx.notify();
            })
            .ok();
        }));
        self.refresh_interpreter(cx);
    }

    fn refresh_interpreter(&mut self, cx: &mut Context<Self>) {
        let php = self.draft.php();
        self.info = None;
        self._tasks.push(cx.spawn(async move |this, cx| {
            let info = php_info(&php).await;
            this.update(cx, |this, cx| {
                if this.draft.php() == php {
                    this.info = info;
                    cx.notify();
                }
            })
            .ok();
        }));
    }

    fn set_interpreter(&mut self, interpreter: Option<String>, cx: &mut Context<Self>) {
        self.draft.interpreter = interpreter;
        self.refresh_interpreter(cx);
        cx.notify();
    }

    fn browse_interpreter(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let paths = cx.prompt_for_paths(gpui::PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: Some("Choose PHP".into()),
        });
        self._tasks.push(cx.spawn_in(window, async move |this, cx| {
            let Ok(Ok(Some(paths))) = paths.await else {
                return;
            };
            let Some(path) = paths.into_iter().next() else {
                return;
            };
            this.update(cx, |this, cx| {
                this.set_interpreter(Some(path.to_string_lossy().into_owned()), cx)
            })
            .ok();
        }));
    }

    fn add_include_path(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let paths = cx.prompt_for_paths(gpui::PathPromptOptions {
            files: false,
            directories: true,
            multiple: true,
            prompt: Some("Add".into()),
        });
        self._tasks.push(cx.spawn_in(window, async move |this, cx| {
            let Ok(Ok(Some(paths))) = paths.await else {
                return;
            };
            this.update(cx, |this, cx| {
                for path in paths {
                    let path = path.to_string_lossy().into_owned();
                    if !this.draft.include_paths.contains(&path) {
                        this.draft.include_paths.push(path);
                    }
                }
                this.selected_include = this.draft.include_paths.len().checked_sub(1);
                cx.notify();
            })
            .ok();
        }));
    }

    fn remove_include_path(&mut self, cx: &mut Context<Self>) {
        if let Some(index) = self.selected_include
            && index < self.draft.include_paths.len()
        {
            self.draft.include_paths.remove(index);
            self.selected_include = if self.draft.include_paths.is_empty() {
                None
            } else {
                Some(index.min(self.draft.include_paths.len() - 1))
            };
            cx.notify();
        }
    }

    /// Every extension the Runtime tab lists: downloaded stubs and the interpreter's.
    fn runtime_extensions(&self) -> Vec<String> {
        let mut names: Vec<String> = self.stub_names.clone();
        if let Some(info) = &self.info {
            for extension in &info.extensions {
                if !names
                    .iter()
                    .any(|name| name.eq_ignore_ascii_case(extension))
                {
                    names.push(extension.clone());
                }
            }
        }
        names.sort_by_key(|name| name.to_lowercase());
        names
    }

    /// The interpreter's extensions that have stubs (or all of them when no
    /// stubs are downloaded).
    fn interpreter_stubs(&self) -> Vec<String> {
        let Some(info) = &self.info else {
            return Vec::new();
        };
        info.extensions
            .iter()
            .map(|extension| {
                self.stub_names
                    .iter()
                    .find(|name| name.eq_ignore_ascii_case(extension))
                    .cloned()
                    .unwrap_or_else(|| extension.clone())
            })
            .collect()
    }

    fn toggle_stub(&mut self, name: &str, on: bool, cx: &mut Context<Self>) {
        let interpreter_stubs = self.interpreter_stubs();
        let stubs = self.draft.stubs.get_or_insert(interpreter_stubs);
        stubs.retain(|stub| stub != name);
        if on {
            stubs.push(name.to_string());
            stubs.sort_by_key(|stub| stub.to_lowercase());
        }
        cx.notify();
    }

    fn download_stubs(&mut self, cx: &mut Context<Self>) {
        self.downloading = true;
        self.error = None;
        cx.notify();
        let fs = self.fs.clone();
        let download = cx.background_spawn(download_stubs(fs.clone(), cx.http_client()));
        self._tasks.push(cx.spawn(async move |this, cx| {
            let result = download.await;
            let stubs = stub_folders(fs.as_ref()).await;
            this.update(cx, |this, cx| {
                this.downloading = false;
                this.stub_names = stubs.into_iter().map(|(name, _)| name).collect();
                if let Err(error) = result {
                    this.error = Some(format!("Couldn't download the stubs: {error:#}").into());
                }
                cx.notify();
            })
            .ok();
        }));
    }

    fn apply(&mut self, close: bool, window: &mut Window, cx: &mut Context<Self>) {
        let save = self.store.update(cx, |store, cx| {
            store.save(self.root.clone(), self.draft.clone(), self.fs.clone(), cx)
        });
        self.saved = self.draft.clone();
        self.error = None;
        cx.notify();
        self._tasks
            .push(cx.spawn_in(window, async move |this, cx| match save.await {
                Ok(()) if close => {
                    this.update_in(cx, |_, window, _| window.remove_window())
                        .ok();
                }
                Ok(()) => {}
                Err(error) => {
                    this.update(cx, |this, cx| {
                        this.error = Some(format!("{error:#}").into());
                        cx.notify();
                    })
                    .ok();
                }
            }));
    }

    fn cancel(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.draft == self.saved {
            window.remove_window();
            return;
        }
        let answer = window.prompt(
            PromptLevel::Warning,
            "Discard the changes to the PHP settings?",
            None,
            &["Discard", "Keep Editing"],
            cx,
        );
        self._tasks.push(cx.spawn_in(window, async move |_, cx| {
            if answer.await.ok() == Some(0) {
                cx.update(|window, _| window.remove_window()).ok();
            }
        }));
    }

    fn level_label(&self) -> String {
        match &self.draft.language_level {
            Some(level) => format!("PHP {level}"),
            None => match (&self.composer_level, &self.info) {
                (Some(level), _) => format!("Auto — PHP {level} (composer.json)"),
                (None, Some(info)) => {
                    format!("Auto — PHP {} (interpreter)", short_version(&info.version))
                }
                (None, None) => "Auto".into(),
            },
        }
    }

    fn interpreter_label(&self) -> String {
        let version = self
            .info
            .as_ref()
            .map(|info| format!("PHP {}", info.version));
        match (&self.draft.interpreter, version) {
            (Some(path), Some(version)) => format!("{version} · {path}"),
            (Some(path), None) => format!("{path} (not working)"),
            (None, Some(version)) => format!("{version} · php from PATH"),
            (None, None) => "php from PATH (not found)".into(),
        }
    }

    fn render_general(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let this = cx.entity().downgrade();
        let levels_menu = move |window: &mut Window, cx: &mut App| {
            let this = this.clone();
            ContextMenu::build(window, cx, move |menu, _, _| {
                let set = |level: Option<&'static str>| {
                    let this = this.clone();
                    move |_: &mut Window, cx: &mut App| {
                        this.update(cx, |this, cx| {
                            this.draft.language_level = level.map(str::to_string);
                            cx.notify();
                        })
                        .ok();
                    }
                };
                let menu = menu.entry("Auto (composer.json or interpreter)", None, set(None));
                LANGUAGE_LEVELS
                    .iter()
                    .rev()
                    .fold(menu.separator(), |menu, level| {
                        menu.entry(format!("PHP {level}"), None, set(Some(level)))
                    })
            })
        };
        let this = cx.entity().downgrade();
        let interpreters = self.interpreters.clone();
        let interpreters_menu = move |window: &mut Window, cx: &mut App| {
            let this = this.clone();
            let interpreters = interpreters.clone();
            ContextMenu::build(window, cx, move |menu, _, _| {
                let set = |path: Option<String>| {
                    let this = this.clone();
                    move |_: &mut Window, cx: &mut App| {
                        this.update(cx, |this, cx| this.set_interpreter(path.clone(), cx))
                            .ok();
                    }
                };
                let menu = menu.entry("php from PATH", None, set(None)).separator();
                interpreters.iter().fold(menu, |menu, (path, version)| {
                    let path = path.to_string_lossy().into_owned();
                    menu.entry(format!("PHP {version} · {path}"), None, set(Some(path)))
                })
            })
        };
        v_flex()
            .gap_2()
            .child(field_row(
                "Language level",
                dialog::combo_box(
                    "php-language-level",
                    None,
                    self.level_label(),
                    None,
                    levels_menu,
                    cx,
                ),
            ))
            .child(field_row(
                "CLI interpreter",
                h_flex()
                    .gap_1()
                    .child(div().flex_1().min_w_0().child(dialog::combo_box(
                        "php-interpreter",
                        None,
                        self.interpreter_label(),
                        None,
                        interpreters_menu,
                        cx,
                    )))
                    .child(
                        IconButton::new("php-interpreter-browse", IconName::Ellipsis)
                            .tooltip(Tooltip::text("Choose a PHP binary…"))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.browse_interpreter(window, cx)
                            })),
                    ),
            ))
            .child(
                div().pl(rems(LABEL_WIDTH + 0.75)).child(
                    Label::new(
                        "Runs scratches and lists PHP Runtime under External Libraries. The language level goes to the PHP language server.",
                    )
                    .size(LabelSize::Small)
                    .color(Color::Muted),
                ),
            )
    }

    fn render_tabs(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let accent = cx.theme().colors().text_accent;
        let tab_button =
            |id: &'static str, label: &'static str, tab: Tab, cx: &mut Context<Self>| {
                let active = self.tab == tab;
                v_flex()
                    .id(id)
                    .cursor_pointer()
                    .gap_1()
                    .child(
                        Label::new(label)
                            .color(if active { Color::Default } else { Color::Muted })
                            .weight(if active {
                                gpui::FontWeight::SEMIBOLD
                            } else {
                                gpui::FontWeight::NORMAL
                            }),
                    )
                    .child(
                        div()
                            .h(px(2.))
                            .rounded_full()
                            .when(active, |bar| bar.bg(accent)),
                    )
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.tab = tab;
                        cx.notify();
                    }))
            };
        h_flex()
            .mt_4()
            .gap_5()
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .child(tab_button(
                "tab-include",
                "Include Path",
                Tab::IncludePath,
                cx,
            ))
            .child(tab_button("tab-runtime", "PHP Runtime", Tab::Runtime, cx))
            .child(tab_button("tab-composer", "Composer", Tab::Composer, cx))
    }

    fn render_include_paths(&self, cx: &mut Context<Self>) -> AnyElement {
        let colors = cx.theme().colors();
        let border = colors.border_variant;
        let selected_bg = colors.element_selected;
        let rows = self
            .draft
            .include_paths
            .iter()
            .enumerate()
            .map(|(index, path)| {
                let selected = self.selected_include == Some(index);
                h_flex()
                    .id(("include-path", index))
                    .px_2()
                    .py_1()
                    .gap_2()
                    .cursor_pointer()
                    .when(selected, |row| row.bg(selected_bg))
                    .child(
                        Icon::new(IconName::Folder)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    )
                    .child(Label::new(path.clone()).buffer_font(cx).truncate())
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.selected_include = Some(index);
                        cx.notify();
                    }))
            })
            .collect::<Vec<_>>();
        v_flex()
            .gap_2()
            .child(
                v_flex()
                    .rounded_md()
                    .border_1()
                    .border_color(border)
                    .overflow_hidden()
                    .child(
                        h_flex()
                            .px_1()
                            .py_0p5()
                            .gap_0p5()
                            .bg(colors.panel_background)
                            .border_b_1()
                            .border_color(border)
                            .child(
                                IconButton::new("include-add", IconName::Plus)
                                    .icon_size(IconSize::Small)
                                    .tooltip(Tooltip::text("Add folders…"))
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.add_include_path(window, cx)
                                    })),
                            )
                            .child(
                                IconButton::new("include-remove", IconName::Dash)
                                    .icon_size(IconSize::Small)
                                    .disabled(self.selected_include.is_none())
                                    .tooltip(Tooltip::text("Remove"))
                                    .on_click(
                                        cx.listener(|this, _, _, cx| this.remove_include_path(cx)),
                                    ),
                            ),
                    )
                    .child(
                        v_flex()
                            .min_h(rems(8.))
                            .bg(colors.editor_background)
                            .children(rows)
                            .when(self.draft.include_paths.is_empty(), |list| {
                                list.child(
                                    div().p_3().child(
                                        Label::new("No include paths. Click + to add a folder.")
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    ),
                                )
                            }),
                    ),
            )
            .child(
                Label::new(
                    "Folders outside the project whose PHP code the project uses (a WordPress core checkout, a shared library). They appear under External Libraries, open read-only, and the language server indexes them.",
                )
                .size(LabelSize::Small)
                .color(Color::Muted),
            )
            .into_any_element()
    }

    fn render_runtime(&self, cx: &mut Context<Self>) -> AnyElement {
        let names = self.runtime_extensions();
        let custom = self.draft.stubs.is_some();
        let enabled: Vec<String> = self
            .draft
            .stubs
            .clone()
            .unwrap_or_else(|| self.interpreter_stubs());
        let has_stubs = !self.stub_names.is_empty();
        let mut column = v_flex().gap_2();
        if !has_stubs {
            column = column.child(
                h_flex()
                    .p_2()
                    .gap_2()
                    .rounded_md()
                    .border_1()
                    .border_color(cx.theme().status().info_border)
                    .bg(cx.theme().status().info_background)
                    .child(
                        Icon::new(IconName::Info)
                            .size(IconSize::Small)
                            .color(Color::Accent),
                    )
                    .child(
                        Label::new(
                            "Download JetBrains' phpstorm-stubs (Apache-2.0) to browse PHP's built-in functions under External Libraries.",
                        )
                        .size(LabelSize::Small),
                    )
                    .child(div().flex_1())
                    .child(
                        dialog::secondary_button(
                            "php-download-stubs",
                            if self.downloading {
                                "Downloading…"
                            } else {
                                "Download Stubs"
                            },
                            cx,
                        )
                        .on_click(cx.listener(|this, _, _, cx| {
                            if !this.downloading {
                                this.download_stubs(cx)
                            }
                        })),
                    ),
            );
        }
        column = column.child(
            h_flex()
                .gap_2()
                .child(
                    Checkbox::new("php-custom-stubs", ToggleState::from(custom))
                        .label("Choose the extensions the language server knows")
                        .on_click(cx.listener(|this, state: &ToggleState, _, cx| {
                            this.draft.stubs = state.selected().then(|| this.interpreter_stubs());
                            cx.notify();
                        })),
                )
                .child(div().flex_1())
                .child(
                    dialog::secondary_button(
                        "php-sync-stubs",
                        "Sync Extensions with Interpreter",
                        cx,
                    )
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.draft.stubs = Some(this.interpreter_stubs());
                        cx.notify();
                    })),
                ),
        );
        let checkboxes = names.into_iter().map(|name| {
            let on = enabled
                .iter()
                .any(|enabled| enabled.eq_ignore_ascii_case(&name));
            let id = SharedString::from(format!("stub-{name}"));
            let toggled = name.clone();
            div().w(rems(11.)).child(
                Checkbox::new(id, ToggleState::from(on))
                    .label(name)
                    .disabled(!custom)
                    .on_click(cx.listener(move |this, state: &ToggleState, _, cx| {
                        this.toggle_stub(&toggled, state.selected(), cx)
                    })),
            )
        });
        column
            .child(h_flex().flex_wrap().gap_y_1().children(checkboxes))
            .when(!custom, |column| {
                column.child(
                    Label::new(
                        "Off: the language server uses its default set. Checked boxes show the interpreter's extensions.",
                    )
                    .size(LabelSize::Small)
                    .color(Color::Muted),
                )
            })
            .into_any_element()
    }

    fn render_composer(&self, cx: &mut Context<Self>) -> AnyElement {
        v_flex()
            .gap_2()
            .child(
                Checkbox::new(
                    "php-composer-libraries",
                    ToggleState::from(self.draft.composer_libraries),
                )
                .label("Add packages as libraries")
                .on_click(cx.listener(|this, state: &ToggleState, _, cx| {
                    this.draft.composer_libraries = state.selected();
                    cx.notify();
                })),
            )
            .child(
                Label::new(
                    "Lists composer.lock's packages under External Libraries; their files in vendor/ open read-only.",
                )
                .size(LabelSize::Small)
                .color(Color::Muted),
            )
            .child(
                Label::new(match &self.composer_level {
                    Some(level) => format!(
                        "composer.json requires PHP {level}; with the language level on Auto, the language server follows it."
                    ),
                    None => "composer.json doesn't set a PHP version.".into(),
                })
                .size(LabelSize::Small)
                .color(Color::Muted),
            )
            .into_any_element()
    }
}

fn field_row(label: &'static str, field: impl IntoElement) -> impl IntoElement {
    h_flex()
        .w_full()
        .gap_3()
        .child(
            div()
                .w(rems(LABEL_WIDTH))
                .flex_none()
                .child(Label::new(label).color(Color::Muted)),
        )
        .child(div().flex_1().min_w_0().child(field))
}

fn section_title(title: &'static str, rule: Hsla) -> impl IntoElement {
    h_flex()
        .pt_3()
        .pb_1()
        .gap_2()
        .child(
            Label::new(title.to_uppercase())
                .size(LabelSize::XSmall)
                .weight(gpui::FontWeight::SEMIBOLD)
                .color(Color::Muted),
        )
        .child(div().flex_1().h_px().bg(rule))
}

impl Focusable for PhpSettingsWindow {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for PhpSettingsWindow {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ui_font = theme_settings::setup_ui_font(window, cx);
        let colors = cx.theme().colors();
        let (rule, panel_background, editor_background, element_background, text) = (
            colors.border_variant,
            colors.panel_background,
            colors.editor_background,
            colors.element_background,
            colors.text,
        );
        let focus = self.focus_handle.clone();
        let summary = format!(
            "{} · {}",
            self.project_name,
            self.info
                .as_ref()
                .map(|info| format!("PHP {}", info.version))
                .unwrap_or_else(|| "no PHP interpreter found".into())
        );
        let body = match self.tab {
            Tab::IncludePath => self.render_include_paths(cx),
            Tab::Runtime => self.render_runtime(cx),
            Tab::Composer => self.render_composer(cx),
        };
        let content = v_flex()
            .size_full()
            .key_context("PhpSettings")
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(|this, _: &menu::Cancel, window, cx| this.cancel(window, cx)))
            .bg(panel_background)
            .child(
                h_flex()
                    .px_3()
                    .border_b_1()
                    .border_color(rule)
                    .map(|header| {
                        if cfg!(target_os = "macos") {
                            header.h(px(38.)).justify_center()
                        } else {
                            header.py_2()
                        }
                    })
                    .child(Label::new("PHP").weight(gpui::FontWeight::SEMIBOLD)),
            )
            .child(
                v_flex()
                    .flex_1()
                    .min_h_0()
                    .bg(editor_background)
                    .child(
                        h_flex()
                            .px_6()
                            .pt_5()
                            .pb_3()
                            .gap_3()
                            .child(
                                div()
                                    .size_9()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .rounded_lg()
                                    .bg(element_background)
                                    .child(
                                        file_icons::FileIcons::get_icon(
                                            std::path::Path::new("settings.php"),
                                            cx,
                                        )
                                        .map(Icon::from_path)
                                        .unwrap_or_else(|| Icon::new(IconName::FileCode))
                                        .size(IconSize::Medium)
                                        .color(Color::Muted),
                                    ),
                            )
                            .child(
                                v_flex()
                                    .min_w_0()
                                    .child(Headline::new("PHP").size(HeadlineSize::Small))
                                    .child(
                                        Label::new(summary)
                                            .size(LabelSize::Small)
                                            .color(Color::Muted)
                                            .truncate(),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .id("php-settings-body")
                            .flex_1()
                            .min_h_0()
                            .w_full()
                            .overflow_y_scroll()
                            .px_6()
                            .pb_6()
                            .child(
                                v_flex()
                                    .w_full()
                                    .max_w(rems(50.))
                                    .child(section_title("Interpreter", rule))
                                    .child(self.render_general(cx))
                                    .child(self.render_tabs(cx))
                                    .child(div().pt_3().child(body)),
                            ),
                    ),
            )
            .child(
                h_flex()
                    .px_3()
                    .py_2()
                    .gap_2()
                    .border_t_1()
                    .border_color(rule)
                    .child(
                        dialog::help_button("php-help", cx)
                            .tooltip(Tooltip::text("PHP settings help"))
                            .on_click(|_, _, cx| {
                                cx.open_url(
                                    "https://github.com/mahbubur001/vega/blob/vega/vega/LIBRARIES_SCRATCHES_PLAN.md",
                                )
                            }),
                    )
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .when_some(self.error.clone(), |row, error| {
                                row.child(Label::new(error).color(Color::Error).truncate())
                            }),
                    )
                    .child(
                        dialog::cancel_button("php-cancel", &focus, cx)
                            .on_click(cx.listener(|this, _, window, cx| this.cancel(window, cx))),
                    )
                    .child(
                        dialog::secondary_button("php-apply", "Apply", cx)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.apply(false, window, cx)
                            })),
                    )
                    .child(
                        dialog::primary_button("php-ok", "OK", &focus, cx)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.apply(true, window, cx)
                            })),
                    ),
            );
        workspace::client_side_decorations(
            v_flex()
                .size_full()
                .font(ui_font)
                .text_color(text)
                .children(self.title_bar.clone())
                .child(content),
            window,
            cx,
        )
    }
}
