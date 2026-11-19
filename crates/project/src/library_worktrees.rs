//! VEGA: hidden worktrees that hold libraries (PHP include paths, PHP stubs).
//! Find in Files searches them too when ignored files are included, like
//! PhpStorm's "Project and Libraries" scope.

use collections::HashSet;
use gpui::{App, Entity, EntityId, Global};
use worktree::Worktree;

use crate::worktree_store::WorktreeStore;

#[derive(Default)]
struct LibraryWorktrees(HashSet<EntityId>);

impl Global for LibraryWorktrees {}

/// Marks `worktree` as a library (or not) for searches.
pub fn mark_library_worktree(worktree: &Entity<Worktree>, library: bool, cx: &mut App) {
    let set = &mut cx.default_global::<LibraryWorktrees>().0;
    if library {
        set.insert(worktree.entity_id());
    } else {
        set.remove(&worktree.entity_id());
    }
}

/// The library worktrees of `store`.
pub(crate) fn library_worktrees(store: &WorktreeStore, cx: &App) -> Vec<Entity<Worktree>> {
    let Some(libraries) = cx.try_global::<LibraryWorktrees>() else {
        return Vec::new();
    };
    store
        .worktrees()
        .filter(|worktree| libraries.0.contains(&worktree.entity_id()))
        .collect()
}
