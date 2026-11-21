//! The branch popup of the title bar, like PhpStorm's: a search field for
//! branches and actions; Update Project, Commit and Push; New Branch and
//! Checkout Tag or Revision; then Recent, Local, Remote and Tags groups. Each
//! branch shows its incoming and outgoing commits and its upstream, and has a
//! menu (›) with Checkout, New Branch from…, Merge, Rebase, Rename, Delete.
//!
//! Tags, recent checkouts, checking out a remote branch or a revision, merge
//! and rebase run `git` in the repository folder, so they are only offered for
//! local projects.

mod entries;

use std::path::Path;
use std::sync::Arc;

use collections::HashSet;
use entries::{ActionKind, BranchInfo, Data, Entry, Group};
use futures::channel::oneshot;
use gpui::{
    Action, AnyElement, App, AsyncApp, Context, DismissEvent, Entity, EventEmitter, FocusHandle,
    Focusable, Render, SharedString, Subscription, Task, WeakEntity, Window, prelude::*, rems,
};
use picker::{Picker, PickerDelegate};
use project::git_store::Repository;
use ui::{
    ContextMenu, HighlightedLabel, Icon, IconButton, IconName, IconSize, KeyBinding, Label,
    LabelSize, ListItem, ListItemSpacing, PopoverMenu, PopoverMenuHandle, prelude::*,
};
use util::ResultExt as _;
use vega_dialog::NameModal;
use workspace::notifications::NotificationId;
use workspace::{Toast, Workspace};

/// The popup the title bar's branch button opens.
pub fn build(
    workspace: WeakEntity<Workspace>,
    repository: Option<Entity<Repository>>,
    window: &mut Window,
    cx: &mut App,
) -> Entity<BranchPopup> {
    cx.new(|cx| BranchPopup::new(workspace, repository, window, cx))
}

pub struct BranchPopup {
    picker: Entity<Picker<BranchPopupDelegate>>,
    _subscription: Subscription,
}

impl BranchPopup {
    fn new(
        workspace: WeakEntity<Workspace>,
        repository: Option<Entity<Repository>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let work_dir = repository.as_ref().and_then(|repository| {
            let is_local = workspace
                .upgrade()
                .is_some_and(|workspace| workspace.read(cx).project().read(cx).is_local());
            is_local.then(|| repository.read(cx).work_directory_abs_path.clone())
        });
        let delegate = BranchPopupDelegate {
            ops: repository.map(|repository| Ops {
                workspace: workspace.clone(),
                repository,
                work_dir,
            }),
            data: Data::default(),
            expanded: Group::ALL
                .into_iter()
                .filter(|group| group.expanded_by_default())
                .collect(),
            query: String::new(),
            rows: Vec::new(),
            selected: 0,
            submenu: PopoverMenuHandle::default(),
        };
        let picker = cx.new(|cx| {
            let mut picker = Picker::list(delegate, window, cx)
                .initial_width(rems(30.))
                .max_height(rems(32.))
                .show_scrollbar(true);
            picker.delegate.load(window, cx);
            picker.refresh(window, cx);
            picker
        });
        let _subscription = cx.subscribe(&picker, |_, _, _: &DismissEvent, cx| {
            cx.emit(DismissEvent);
        });
        Self {
            picker,
            _subscription,
        }
    }
}

impl EventEmitter<DismissEvent> for BranchPopup {}

impl Focusable for BranchPopup {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for BranchPopup {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("VegaBranchPopup")
            .child(self.picker.clone())
    }
}

// ---- operations ---------------------------------------------------------------------------------

#[derive(Clone)]
struct Ops {
    workspace: WeakEntity<Workspace>,
    repository: Entity<Repository>,
    /// The repository folder, for local projects only.
    work_dir: Option<Arc<Path>>,
}

struct BranchToast;

impl Ops {
    fn show_error(&self, message: String, cx: &mut App) {
        self.workspace
            .update(cx, |workspace, cx| {
                workspace.show_toast(
                    Toast::new(NotificationId::unique::<BranchToast>(), message),
                    cx,
                )
            })
            .ok();
    }

    /// Reports the error of a repository job, if it fails.
    fn report(&self, job: oneshot::Receiver<anyhow::Result<()>>, cx: &mut App) {
        let ops = self.clone();
        cx.spawn(async move |cx| {
            let result = job.await.map_err(anyhow::Error::from).and_then(|r| r);
            if let Err(error) = result {
                cx.update(|cx| ops.show_error(format!("{error:#}"), cx));
            }
        })
        .detach();
    }

    /// Runs `git args…` in the repository folder (local projects).
    fn run_git(&self, args: Vec<String>, cx: &mut App) {
        let Some(work_dir) = self.work_dir.clone() else {
            return;
        };
        let ops = self.clone();
        cx.spawn(async move |cx: &mut AsyncApp| {
            let output = util::command::new_command("git")
                .args(&args)
                .current_dir(work_dir.as_ref())
                .output()
                .await;
            let problem = match output {
                Ok(output) if output.status.success() => None,
                Ok(output) => Some(String::from_utf8_lossy(&output.stderr).trim().to_string()),
                Err(error) => Some(format!("couldn't run git: {error}")),
            };
            if let Some(problem) = problem {
                let command = format!("git {}", args.join(" "));
                cx.update(|cx| ops.show_error(format!("{command} failed: {problem}"), cx));
            }
        })
        .detach();
    }

    fn checkout(&self, branch: &BranchInfo, cx: &mut App) {
        if branch.is_current {
            return;
        }
        if branch.is_remote {
            // A local branch that tracks it, like PhpStorm's Checkout.
            self.run_git(
                vec!["checkout".into(), "--track".into(), branch.name.to_string()],
                cx,
            );
        } else {
            let job = self.repository.update(cx, |repository, _| {
                repository.change_branch(branch.name.to_string())
            });
            self.report(job, cx);
        }
    }

    /// Brings a branch that isn't checked out up to date with its upstream,
    /// fast-forward only (git refuses when the two have diverged).
    fn update_other(&self, name: &str, upstream: &str, cx: &mut App) {
        if let Some(args) = update_args(name, upstream) {
            self.run_git(args, cx);
        }
    }

    fn checkout_revision(&self, revision: String, cx: &mut App) {
        self.run_git(vec!["checkout".into(), revision], cx);
    }

    fn merge_into_current(&self, name: &str, cx: &mut App) {
        self.run_git(vec!["merge".into(), name.to_string()], cx);
    }

    fn rebase_current_onto(&self, name: &str, cx: &mut App) {
        self.run_git(vec!["rebase".into(), name.to_string()], cx);
    }

    fn delete(&self, name: &str, cx: &mut App) {
        let job = self.repository.update(cx, |repository, _| {
            repository.delete_branch(false, name.to_string(), false)
        });
        self.report(job, cx);
    }

    /// Asks for a name, then creates the branch from `base` (or the current
    /// commit) and checks it out.
    fn new_branch(&self, base: Option<String>, window: &mut Window, cx: &mut App) {
        let ops = self.clone();
        let title = match &base {
            Some(base) => format!("New Branch from {base}"),
            None => "New Branch".to_string(),
        };
        self.ask(
            title,
            "Branch name",
            "",
            branch_name_problem,
            window,
            cx,
            move |name, cx| {
                let job = ops
                    .repository
                    .update(cx, |repository, _| repository.create_branch(name, base));
                ops.report(job, cx);
            },
        );
    }

    fn rename(&self, old: String, window: &mut Window, cx: &mut App) {
        let ops = self.clone();
        let title = format!("Rename {old}");
        let initial = old.clone();
        self.ask(
            title,
            "New name",
            &initial,
            branch_name_problem,
            window,
            cx,
            move |name, cx| {
                if name == old {
                    return;
                }
                let job = ops
                    .repository
                    .update(cx, |repository, _| repository.rename_branch(old, name));
                ops.report(job, cx);
            },
        );
    }

    fn ask_revision(&self, window: &mut Window, cx: &mut App) {
        let ops = self.clone();
        self.ask(
            "Checkout Tag or Revision".to_string(),
            "Tag, branch or commit",
            "",
            revision_problem,
            window,
            cx,
            move |revision, cx| ops.checkout_revision(revision, cx),
        );
    }

    #[allow(clippy::too_many_arguments)]
    fn ask(
        &self,
        title: String,
        label: &'static str,
        initial: &str,
        validate: fn(&str) -> Option<&'static str>,
        window: &mut Window,
        cx: &mut App,
        on_confirm: impl FnOnce(String, &mut App) + 'static,
    ) {
        let initial = initial.to_string();
        self.workspace
            .update(cx, |workspace, cx| {
                workspace.toggle_modal(window, cx, move |window, cx| {
                    NameModal::with_validator(
                        title,
                        label,
                        &initial,
                        validate,
                        move |name, _, cx| on_confirm(name, cx),
                        window,
                        cx,
                    )
                });
            })
            .ok();
    }
}

/// Why `name` can't be a git branch name, if it can't (a short version of
/// `git check-ref-format`).
fn branch_name_problem(name: &str) -> Option<&'static str> {
    if name.is_empty() {
        return Some("Enter a name");
    }
    let bad = name
        .chars()
        .any(|c| c.is_whitespace() || "~^:?*[\\".contains(c))
        || name.contains("..")
        || name.contains("@{")
        || name.starts_with(['-', '/', '.'])
        || name.ends_with(['/', '.'])
        || name.ends_with(".lock");
    bad.then_some("Not a valid branch name")
}

fn revision_problem(revision: &str) -> Option<&'static str> {
    if revision.is_empty() {
        Some("Enter a tag, branch or commit")
    } else if revision.chars().any(char::is_whitespace) || revision.starts_with('-') {
        Some("Not a valid revision")
    } else {
        None
    }
}

// ---- the list -----------------------------------------------------------------------------------

pub struct BranchPopupDelegate {
    ops: Option<Ops>,
    data: Data,
    expanded: HashSet<Group>,
    query: String,
    rows: Vec<Entry>,
    selected: usize,
    submenu: PopoverMenuHandle<ContextMenu>,
}

impl BranchPopupDelegate {
    /// Loads branches (and, for local projects, tags and recent checkouts).
    fn load(&mut self, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(ops) = self.ops.clone() else {
            return;
        };
        let branches = ops
            .repository
            .update(cx, |repository, _| repository.branches());
        let work_dir = ops.work_dir.clone();
        cx.spawn_in(window, async move |picker, cx| {
            let branches = branches
                .await
                .map_err(anyhow::Error::from)
                .and_then(|result| result)
                .log_err()
                .map(|result| result.branches)
                .unwrap_or_default();
            let (recent, tags) = match work_dir {
                Some(dir) => (
                    git_output(&dir, &["reflog", "--format=%gs", "-n", "300"]).await,
                    git_output(&dir, &["tag", "--sort=-creatordate"]).await,
                ),
                None => (String::new(), String::new()),
            };
            let data = Data {
                branches: branches.iter().map(branch_info).collect(),
                recent: entries::parse_recent(&recent),
                tags: tags
                    .lines()
                    .map(str::trim)
                    .filter(|tag| !tag.is_empty())
                    .map(|tag| SharedString::from(tag.to_string()))
                    .collect(),
            };
            picker
                .update_in(cx, |picker, window, cx| {
                    picker.delegate.data = data;
                    picker.refresh(window, cx);
                })
                .ok();
        })
        .detach();
    }

    fn rebuild(&mut self) {
        self.rows = entries::entries(&self.data, &self.query, &self.expanded);
        self.selected = self.selected.min(self.rows.len().saturating_sub(1));
    }

    fn current_branch(&self) -> Option<&BranchInfo> {
        self.data.branches.iter().find(|branch| branch.is_current)
    }

    fn run_action(&self, action: ActionKind, window: &mut Window, cx: &mut App) {
        match action {
            ActionKind::UpdateProject => window.dispatch_action(git::Pull.boxed_clone(), cx),
            ActionKind::Commit => window.dispatch_action(git::ExpandCommitEditor.boxed_clone(), cx),
            ActionKind::Push => window.dispatch_action(git::Push.boxed_clone(), cx),
            ActionKind::NewBranch => {
                if let Some(ops) = &self.ops {
                    ops.new_branch(None, window, cx);
                }
            }
            ActionKind::CheckoutRevision => {
                if let Some(ops) = &self.ops {
                    ops.ask_revision(window, cx);
                }
            }
        }
    }

    fn branch_menu(
        &self,
        target: MenuTarget,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Entity<ContextMenu>> {
        let ops = self.ops.clone()?;
        let local = ops.work_dir.is_some();
        let current_label: SharedString = self
            .current_branch()
            .map(|branch| branch.name.clone())
            .unwrap_or_else(|| "HEAD".into());
        Some(ContextMenu::build(window, cx, move |menu, _, _| {
            let name = target.name();
            let (is_current, is_branch, is_remote) = match &target {
                MenuTarget::Branch(branch) => (branch.is_current, true, branch.is_remote),
                MenuTarget::Tag(_) => (false, false, false),
            };
            let other_upstream = match &target {
                MenuTarget::Branch(branch) if !branch.is_current && !branch.is_remote => {
                    branch.upstream.clone()
                }
                _ => None,
            };
            let mut menu = menu;
            if !is_current {
                let ops = ops.clone();
                let target = target.clone();
                menu = menu.entry("Checkout", None, move |_, cx| match &target {
                    MenuTarget::Branch(branch) => ops.checkout(branch, cx),
                    MenuTarget::Tag(tag) => ops.checkout_revision(tag.to_string(), cx),
                });
            }
            {
                let ops = ops.clone();
                let base = name.to_string();
                menu = menu.entry(
                    format!("New Branch from '{name}'…"),
                    None,
                    move |window, cx| ops.new_branch(Some(base.clone()), window, cx),
                );
            }
            if !is_current && local {
                menu = menu.separator();
                {
                    let ops = ops.clone();
                    let name = name.to_string();
                    menu = menu.entry(
                        format!("Rebase '{current_label}' onto '{name}'"),
                        None,
                        move |_, cx| ops.rebase_current_onto(&name, cx),
                    );
                }
                {
                    let ops = ops.clone();
                    let name = name.to_string();
                    menu = menu.entry(
                        format!("Merge '{name}' into '{current_label}'"),
                        None,
                        move |_, cx| ops.merge_into_current(&name, cx),
                    );
                }
                if let Some(upstream) = other_upstream {
                    let ops = ops.clone();
                    let name = name.to_string();
                    menu = menu.separator().entry("Update", None, move |_, cx| {
                        ops.update_other(&name, &upstream, cx)
                    });
                }
            }
            if is_current {
                menu = menu
                    .separator()
                    .entry("Update", Some(git::Pull.boxed_clone()), |window, cx| {
                        window.dispatch_action(git::Pull.boxed_clone(), cx)
                    })
                    .entry("Push…", Some(git::Push.boxed_clone()), |window, cx| {
                        window.dispatch_action(git::Push.boxed_clone(), cx)
                    });
            }
            if is_branch && !is_remote {
                menu = menu.separator();
                {
                    let ops = ops.clone();
                    let name = name.to_string();
                    menu = menu.entry("Rename…", None, move |window, cx| {
                        ops.rename(name.clone(), window, cx)
                    });
                }
                if !is_current {
                    let name = name.to_string();
                    menu = menu.entry("Delete", None, move |_, cx| ops.delete(&name, cx));
                }
            }
            menu
        }))
    }
}

/// What a branch row's › menu is for.
#[derive(Clone)]
enum MenuTarget {
    Branch(BranchInfo),
    Tag(SharedString),
}

impl MenuTarget {
    fn name(&self) -> SharedString {
        match self {
            MenuTarget::Branch(branch) => branch.name.clone(),
            MenuTarget::Tag(tag) => tag.clone(),
        }
    }
}

fn branch_info(branch: &git::repository::Branch) -> BranchInfo {
    let tracking = branch
        .upstream
        .as_ref()
        .and_then(|upstream| upstream.tracking.status());
    BranchInfo {
        name: branch.name().to_string().into(),
        is_current: branch.is_head,
        is_remote: branch.is_remote(),
        behind: tracking.map_or(0, |tracking| tracking.behind),
        ahead: tracking.map_or(0, |tracking| tracking.ahead),
        upstream: branch.upstream.as_ref().map(|upstream| {
            upstream
                .ref_name
                .strip_prefix("refs/remotes/")
                .unwrap_or(upstream.ref_name.as_ref())
                .to_string()
                .into()
        }),
    }
}

async fn git_output(dir: &Path, args: &[&str]) -> String {
    util::command::new_command("git")
        .args(args)
        .current_dir(dir)
        .output()
        .await
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).into_owned())
        .unwrap_or_default()
}

/// `git fetch origin dev:dev` for local branch `dev` tracking `origin/dev`:
/// fetches and fast-forwards a branch that isn't checked out.
fn update_args(name: &str, upstream: &str) -> Option<Vec<String>> {
    let (remote, branch) = upstream.split_once('/')?;
    if remote.is_empty() || branch.is_empty() {
        return None;
    }
    Some(vec![
        "fetch".into(),
        remote.into(),
        format!("{branch}:{name}"),
    ])
}

fn count(n: u32) -> String {
    if n > 99 { "99+".into() } else { n.to_string() }
}

impl PickerDelegate for BranchPopupDelegate {
    type ListItem = AnyElement;

    fn name() -> &'static str {
        "vega-branch-popup"
    }

    fn placeholder_text(&self, _: &mut Window, _: &mut App) -> Arc<str> {
        "Search for branches and actions".into()
    }

    fn no_matches_text(&self, _: &mut Window, _: &mut App) -> Option<SharedString> {
        Some("Nothing found".into())
    }

    fn has_another_open_menu(&self, window: &Window, cx: &App) -> bool {
        self.submenu.is_deployed() || self.submenu.is_focused(window, cx)
    }

    fn match_count(&self) -> usize {
        self.rows.len()
    }

    fn selected_index(&self) -> usize {
        self.selected
    }

    fn set_selected_index(&mut self, ix: usize, _: &mut Window, _: &mut Context<Picker<Self>>) {
        self.selected = ix;
    }

    fn separators_after_indices(&self) -> Vec<usize> {
        self.rows
            .iter()
            .enumerate()
            .filter_map(|(ix, row)| match row {
                Entry::Action(action, _) if action.ends_block() => Some(ix),
                _ => None,
            })
            .collect()
    }

    fn update_matches(
        &mut self,
        query: String,
        _: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        if query != self.query {
            self.selected = 0;
        }
        self.query = query;
        self.rebuild();
        Task::ready(())
    }

    fn confirm(&mut self, _secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(row) = self.rows.get(self.selected).cloned() else {
            return;
        };
        match row {
            Entry::Header(group, _) => {
                if !self.expanded.remove(&group) {
                    self.expanded.insert(group);
                }
                self.rebuild();
                cx.notify();
            }
            Entry::Action(action, _) => {
                cx.emit(DismissEvent);
                self.run_action(action, window, cx);
            }
            Entry::Branch(_, branch, _) => {
                cx.emit(DismissEvent);
                if branch.is_current {
                    // Already checked out: a click pulls what's waiting (↓N).
                    if branch.behind > 0 {
                        window.dispatch_action(git::Pull.boxed_clone(), cx);
                    }
                } else if let Some(ops) = &self.ops {
                    ops.checkout(&branch, cx);
                }
            }
            Entry::Tag(tag, _) => {
                cx.emit(DismissEvent);
                if let Some(ops) = &self.ops {
                    ops.checkout_revision(tag.to_string(), cx);
                }
            }
        }
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let row = self.rows.get(ix)?;
        let item = ListItem::new(ix)
            .inset(true)
            .spacing(ListItemSpacing::Sparse)
            .toggle_state(selected);
        let element =
            match row {
                Entry::Action(action, positions) => {
                    let (icon, color) = match action {
                        ActionKind::UpdateProject => (Some(IconName::ArrowDown), Color::Accent),
                        ActionKind::Commit => (Some(IconName::GitCommit), Color::Accent),
                        ActionKind::Push => (Some(IconName::ArrowUp), Color::Success),
                        ActionKind::NewBranch => (Some(IconName::Plus), Color::Muted),
                        ActionKind::CheckoutRevision => (None, Color::Muted),
                    };
                    let binding = match action {
                        ActionKind::UpdateProject => Some(KeyBinding::for_action(&git::Pull, cx)),
                        ActionKind::Commit => {
                            Some(KeyBinding::for_action(&git::ExpandCommitEditor, cx))
                        }
                        ActionKind::Push => Some(KeyBinding::for_action(&git::Push, cx)),
                        _ => None,
                    };
                    item.start_slot(h_flex().w_4().children(
                        icon.map(|icon| Icon::new(icon).size(IconSize::Small).color(color)),
                    ))
                    .child(HighlightedLabel::new(action.label(), positions.clone()))
                    .end_slot::<KeyBinding>(binding)
                    .into_any_element()
                }
                Entry::Header(group, open) => item
                    .start_slot(
                        Icon::new(if *open {
                            IconName::ChevronDown
                        } else {
                            IconName::ChevronRight
                        })
                        .size(IconSize::Small)
                        .color(Color::Muted),
                    )
                    .child(Label::new(group.label()).color(Color::Muted))
                    .into_any_element(),
                Entry::Branch(group, branch, positions) => {
                    let icon = if branch.is_current {
                        Icon::new(IconName::StarFilled)
                            .size(IconSize::Small)
                            .color(Color::Warning)
                    } else {
                        Icon::new(IconName::GitBranch)
                            .size(IconSize::Small)
                            .color(Color::Muted)
                    };
                    let target = MenuTarget::Branch(branch.clone());
                    item.indent_level(1)
                        .start_slot(icon)
                        .child(
                            h_flex()
                                .gap_2()
                                .child(HighlightedLabel::new(
                                    branch.name.clone(),
                                    positions.clone(),
                                ))
                                .when(branch.behind > 0, |row| {
                                    row.child(
                                        Label::new(format!("↓{}", count(branch.behind)))
                                            .size(LabelSize::Small)
                                            .color(Color::Accent),
                                    )
                                })
                                .when(branch.ahead > 0, |row| {
                                    row.child(
                                        Label::new(format!("↑{}", count(branch.ahead)))
                                            .size(LabelSize::Small)
                                            .color(Color::Success),
                                    )
                                }),
                        )
                        .end_slot(
                            h_flex()
                                .gap_1()
                                .when_some(branch.upstream.clone(), |row, upstream| {
                                    row.child(
                                        Label::new(upstream)
                                            .size(LabelSize::Small)
                                            .color(Color::Muted),
                                    )
                                })
                                .child(self.menu_button(ix, *group, target, cx)),
                        )
                        .into_any_element()
                }
                Entry::Tag(tag, positions) => {
                    let target = MenuTarget::Tag(tag.clone());
                    item.indent_level(1)
                        .start_slot(
                            Icon::new(IconName::Hash)
                                .size(IconSize::Small)
                                .color(Color::Muted),
                        )
                        .child(HighlightedLabel::new(tag.clone(), positions.clone()))
                        .end_slot(self.menu_button(ix, Group::Tags, target, cx))
                        .into_any_element()
                }
            };
        Some(element)
    }
}

impl BranchPopupDelegate {
    fn menu_button(
        &self,
        ix: usize,
        group: Group,
        target: MenuTarget,
        cx: &mut Context<Picker<Self>>,
    ) -> impl IntoElement {
        let picker = cx.entity().downgrade();
        PopoverMenu::new(SharedString::from(format!(
            "branch-menu-{}-{ix}",
            group.label()
        )))
        .with_handle(self.submenu.clone())
        .anchor(gpui::Anchor::TopLeft)
        .trigger(
            IconButton::new(
                SharedString::from(format!("branch-menu-button-{ix}")),
                IconName::ChevronRight,
            )
            .icon_size(IconSize::Small)
            .icon_color(Color::Muted),
        )
        .menu(move |window, cx| {
            let picker = picker.upgrade()?;
            let target = target.clone();
            picker.update(cx, |picker, cx| {
                picker.delegate.branch_menu(target, window, cx)
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn updates_another_branch_by_fast_forward_fetch() {
        assert_eq!(
            update_args("main", "origin/main"),
            Some(vec!["fetch".into(), "origin".into(), "main:main".into()])
        );
        assert_eq!(
            update_args("fb", "origin/feature/fb"),
            Some(vec![
                "fetch".into(),
                "origin".into(),
                "feature/fb:fb".into()
            ])
        );
        assert_eq!(update_args("main", "main"), None);
    }

    #[test]
    fn checks_branch_names_like_git() {
        assert_eq!(branch_name_problem("feature/login"), None);
        assert!(branch_name_problem("").is_some());
        assert!(branch_name_problem("has space").is_some());
        assert!(branch_name_problem("a..b").is_some());
        assert!(branch_name_problem("-x").is_some());
        assert!(branch_name_problem("x.lock").is_some());
        assert!(branch_name_problem("x/").is_some());
        assert_eq!(revision_problem("v6.0.2"), None);
        assert!(revision_problem("a b").is_some());
    }
}
