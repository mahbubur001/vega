//! The rows of the branch popup: actions, then the Recent, Local, Remote and
//! Tags groups (collapsible), filtered by the search query.

use collections::HashSet;
use gpui::SharedString;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Group {
    Recent,
    Local,
    Remote,
    Tags,
}

impl Group {
    pub(crate) const ALL: [Group; 4] = [Group::Recent, Group::Local, Group::Remote, Group::Tags];

    pub(crate) fn label(self) -> &'static str {
        match self {
            Group::Recent => "Recent",
            Group::Local => "Local",
            Group::Remote => "Remote",
            Group::Tags => "Tags",
        }
    }

    /// PhpStorm opens with Recent unfolded and the rest folded.
    pub(crate) fn expanded_by_default(self) -> bool {
        self == Group::Recent
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ActionKind {
    UpdateProject,
    Commit,
    Push,
    NewBranch,
    CheckoutRevision,
}

impl ActionKind {
    pub(crate) const ALL: [ActionKind; 5] = [
        ActionKind::UpdateProject,
        ActionKind::Commit,
        ActionKind::Push,
        ActionKind::NewBranch,
        ActionKind::CheckoutRevision,
    ];

    pub(crate) fn label(self) -> &'static str {
        match self {
            ActionKind::UpdateProject => "Update Project…",
            ActionKind::Commit => "Commit…",
            ActionKind::Push => "Push…",
            ActionKind::NewBranch => "New Branch…",
            ActionKind::CheckoutRevision => "Checkout Tag or Revision…",
        }
    }

    /// A separator follows the last action of each block.
    pub(crate) fn ends_block(self) -> bool {
        matches!(self, ActionKind::Push | ActionKind::CheckoutRevision)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct BranchInfo {
    /// `dev`, or `origin/dev` for a remote branch.
    pub name: SharedString,
    pub is_current: bool,
    pub is_remote: bool,
    /// Commits on the upstream that aren't here (to pull).
    pub behind: u32,
    /// Commits here that aren't on the upstream (to push).
    pub ahead: u32,
    /// The upstream, e.g. `origin/dev`.
    pub upstream: Option<SharedString>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Data {
    pub branches: Vec<BranchInfo>,
    /// Recently checked out local branches, newest first.
    pub recent: Vec<SharedString>,
    /// Tags, newest first.
    pub tags: Vec<SharedString>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Entry {
    Action(ActionKind, Vec<usize>),
    Header(Group, bool),
    Branch(Group, BranchInfo, Vec<usize>),
    Tag(SharedString, Vec<usize>),
}

/// How many branches Recent lists.
const RECENT_LIMIT: usize = 5;

impl Data {
    fn group_items(&self, group: Group) -> Vec<Entry> {
        match group {
            Group::Recent => self
                .recent
                .iter()
                .filter_map(|name| {
                    self.branches
                        .iter()
                        .find(|branch| !branch.is_remote && branch.name == *name)
                })
                .take(RECENT_LIMIT)
                .map(|branch| Entry::Branch(group, branch.clone(), Vec::new()))
                .collect(),
            Group::Local | Group::Remote => {
                let remote = group == Group::Remote;
                let mut branches: Vec<&BranchInfo> = self
                    .branches
                    .iter()
                    .filter(|branch| branch.is_remote == remote)
                    .filter(|branch| !branch.name.ends_with("/HEAD"))
                    .collect();
                branches.sort_by(|a, b| {
                    b.is_current
                        .cmp(&a.is_current)
                        .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
                });
                branches
                    .into_iter()
                    .map(|branch| Entry::Branch(group, branch.clone(), Vec::new()))
                    .collect()
            }
            Group::Tags => self
                .tags
                .iter()
                .map(|tag| Entry::Tag(tag.clone(), Vec::new()))
                .collect(),
        }
    }
}

/// The rows for `query`. Without a query: the actions, then each group that
/// has items as a header, followed by its items when it is in `expanded`.
/// With a query: the matching actions and items, each group unfolded.
pub(crate) fn entries(data: &Data, query: &str, expanded: &HashSet<Group>) -> Vec<Entry> {
    let query = query.trim();
    let mut rows = Vec::new();
    for action in ActionKind::ALL {
        if let Some(positions) = match_positions(query, action.label()) {
            rows.push(Entry::Action(action, positions));
        }
    }
    for group in Group::ALL {
        let items: Vec<Entry> = data
            .group_items(group)
            .into_iter()
            .filter_map(|entry| with_positions(entry, query))
            .collect();
        if items.is_empty() {
            continue;
        }
        let open = !query.is_empty() || expanded.contains(&group);
        rows.push(Entry::Header(group, open));
        if open {
            rows.extend(items);
        }
    }
    rows
}

fn with_positions(entry: Entry, query: &str) -> Option<Entry> {
    match entry {
        Entry::Branch(group, branch, _) => {
            let positions = match_positions(query, &branch.name)?;
            Some(Entry::Branch(group, branch, positions))
        }
        Entry::Tag(tag, _) => {
            let positions = match_positions(query, &tag)?;
            Some(Entry::Tag(tag, positions))
        }
        other => Some(other),
    }
}

/// Byte offsets of `query`'s characters found in order in `text`, ignoring
/// case (`Some(vec![])` for an empty query, `None` when it doesn't match).
pub(crate) fn match_positions(query: &str, text: &str) -> Option<Vec<usize>> {
    let mut wanted = query.chars().filter(|c| !c.is_whitespace()).peekable();
    let mut positions = Vec::new();
    for (offset, c) in text.char_indices() {
        let Some(next) = wanted.peek() else {
            break;
        };
        if c.to_lowercase().eq(next.to_lowercase()) {
            positions.push(offset);
            wanted.next();
        }
    }
    wanted.peek().is_none().then_some(positions)
}

/// Recently checked out branches from `git reflog --format=%gs` output
/// ("checkout: moving from main to dev"), newest first, without repeats.
pub(crate) fn parse_recent(reflog: &str) -> Vec<SharedString> {
    let mut seen = HashSet::default();
    let mut recent = Vec::new();
    for line in reflog.lines() {
        let Some(moves) = line.strip_prefix("checkout: moving from ") else {
            continue;
        };
        let Some((_, to)) = moves.rsplit_once(" to ") else {
            continue;
        };
        let to = to.trim();
        if !to.is_empty() && seen.insert(to.to_string()) {
            recent.push(SharedString::from(to.to_string()));
        }
    }
    recent
}

#[cfg(test)]
mod tests {
    use super::*;

    fn branch(name: &str, is_current: bool, is_remote: bool) -> BranchInfo {
        BranchInfo {
            name: name.to_string().into(),
            is_current,
            is_remote,
            behind: 0,
            ahead: 0,
            upstream: None,
        }
    }

    fn data() -> Data {
        Data {
            branches: vec![
                branch("main", false, false),
                branch("dev", true, false),
                branch("formBuilderUpdate", false, false),
                branch("origin/HEAD", false, true),
                branch("origin/dev", false, true),
                branch("origin/main", false, true),
            ],
            recent: vec!["dev".into(), "formBuilderUpdate".into(), "gone".into()],
            tags: vec!["v6.0.2".into()],
        }
    }

    fn describe(rows: &[Entry]) -> Vec<String> {
        rows.iter()
            .map(|row| match row {
                Entry::Action(action, _) => format!("action {}", action.label()),
                Entry::Header(group, open) => {
                    format!("{} {}", if *open { "v" } else { ">" }, group.label())
                }
                Entry::Branch(group, branch, _) => {
                    format!("  {} {}", group.label(), branch.name)
                }
                Entry::Tag(tag, _) => format!("  tag {tag}"),
            })
            .collect()
    }

    #[test]
    fn opens_with_actions_and_recent_unfolded() {
        let expanded = HashSet::from_iter([Group::Recent]);
        assert_eq!(
            describe(&entries(&data(), "", &expanded)),
            vec![
                "action Update Project…",
                "action Commit…",
                "action Push…",
                "action New Branch…",
                "action Checkout Tag or Revision…",
                "v Recent",
                "  Recent dev",
                "  Recent formBuilderUpdate",
                "> Local",
                "> Remote",
                "> Tags",
            ]
        );
    }

    #[test]
    fn unfolded_groups_list_current_first_and_skip_remote_head() {
        let expanded = HashSet::from_iter([Group::Local, Group::Remote]);
        let rows = describe(&entries(&data(), "", &expanded));
        assert!(rows.ends_with(&[
            "v Local".to_string(),
            "  Local dev".to_string(),
            "  Local formBuilderUpdate".to_string(),
            "  Local main".to_string(),
            "v Remote".to_string(),
            "  Remote origin/dev".to_string(),
            "  Remote origin/main".to_string(),
            "> Tags".to_string(),
        ]));
    }

    #[test]
    fn a_query_unfolds_every_group_with_matches() {
        assert_eq!(
            describe(&entries(&data(), "fbu", &HashSet::default())),
            vec![
                "v Recent",
                "  Recent formBuilderUpdate",
                "v Local",
                "  Local formBuilderUpdate",
            ]
        );
        assert_eq!(
            describe(&entries(&data(), "push", &HashSet::default())),
            vec!["action Push…"]
        );
    }

    #[test]
    fn matches_characters_in_order_ignoring_case() {
        assert_eq!(
            match_positions("fbu", "formBuilderUpdate"),
            Some(vec![0, 4, 5])
        );
        assert_eq!(match_positions("", "main"), Some(vec![]));
        assert_eq!(match_positions("xyz", "main"), None);
    }

    #[test]
    fn reads_recent_checkouts_from_the_reflog() {
        let reflog = "checkout: moving from main to dev\n\
                      commit: Fix\n\
                      checkout: moving from dev to main\n\
                      checkout: moving from feature to dev\n";
        assert_eq!(parse_recent(reflog), vec!["dev", "main"]);
    }
}
