//! VEGA: checks that run before a commit (IntelliJ's "Before Commit" options,
//! such as Check TODO). Other crates register a check; the git panel runs them
//! all before it commits and commits only when every check agrees.

use std::path::PathBuf;
use std::rc::Rc;

use gpui::{App, Entity, Global, Task, WeakEntity, Window};
use project::Project;
use workspace::Workspace;

/// What is about to be committed.
pub struct CommitCheckRequest {
    pub workspace: WeakEntity<Workspace>,
    pub project: Entity<Project>,
    /// The repository's working directory.
    pub work_directory: PathBuf,
    /// Only the staged changes are committed (otherwise every tracked change is).
    pub staged_only: bool,
}

/// Resolves to `true` to go on with the commit, `false` to stop it.
pub type CommitCheck = Rc<dyn Fn(&CommitCheckRequest, &mut Window, &mut App) -> Task<bool>>;

#[derive(Default)]
struct CommitChecks(Vec<CommitCheck>);

impl Global for CommitChecks {}

/// Adds a check that runs before every commit from the git panel.
pub fn register_commit_check(check: CommitCheck, cx: &mut App) {
    cx.default_global::<CommitChecks>().0.push(check);
}

/// Runs the registered checks one after another. `None` when there are none.
pub(crate) fn run_commit_checks(
    request: CommitCheckRequest,
    window: &mut Window,
    cx: &mut App,
) -> Option<Task<bool>> {
    let checks = cx.try_global::<CommitChecks>()?.0.clone();
    if checks.is_empty() {
        return None;
    }
    let tasks: Vec<Task<bool>> = checks
        .iter()
        .map(|check| check(&request, window, cx))
        .collect();
    Some(cx.background_executor().spawn(async move {
        for task in tasks {
            if !task.await {
                return false;
            }
        }
        true
    }))
}
