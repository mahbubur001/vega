//! Decides what to do with one synced file by comparing this machine's copy,
//! the copy on GitHub, and the copy both last agreed on (the *base*).

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Outcome {
    /// Nothing changed.
    Unchanged,
    /// Both sides already match; record that as the new base.
    Adopt,
    /// Only this machine changed: upload it.
    Push,
    /// Only GitHub changed: download it.
    Pull,
    /// Both sides changed differently. Never resolved silently.
    Conflict,
}

/// `local` is the file on disk (empty if missing), `remote` the gist file
/// (`None` if the gist doesn't have it yet), and `base` the content both
/// sides had after the last sync (`None` before the first sync).
pub(crate) fn reconcile(local: &str, remote: Option<&str>, base: Option<&str>) -> Outcome {
    let Some(remote) = remote else {
        return if is_untouched(local) {
            Outcome::Unchanged
        } else {
            Outcome::Push
        };
    };

    if local == remote {
        return if base == Some(local) {
            Outcome::Unchanged
        } else {
            Outcome::Adopt
        };
    }

    match base {
        Some(base) if local == base => Outcome::Pull,
        Some(base) if remote == base => Outcome::Push,
        Some(_) => Outcome::Conflict,
        // First sync on this machine: a fresh install takes GitHub's copy.
        None if is_untouched(local) => Outcome::Pull,
        None => Outcome::Conflict,
    }
}

/// Whether a JSON config file holds nothing but comments and an empty
/// object or array, like the template VEGA writes on first launch.
pub(crate) fn is_untouched(contents: &str) -> bool {
    let meaningful: String = contents
        .lines()
        .map(str::trim)
        .filter(|line| !line.starts_with("//"))
        .collect();
    matches!(meaningful.as_str(), "" | "{}" | "[]")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unchanged_and_adopt() {
        assert_eq!(reconcile("a", Some("a"), Some("a")), Outcome::Unchanged);
        assert_eq!(reconcile("b", Some("b"), Some("a")), Outcome::Adopt);
        assert_eq!(reconcile("b", Some("b"), None), Outcome::Adopt);
    }

    #[test]
    fn test_one_side_changed() {
        assert_eq!(
            reconcile("local", Some("base"), Some("base")),
            Outcome::Push
        );
        assert_eq!(
            reconcile("base", Some("remote"), Some("base")),
            Outcome::Pull
        );
    }

    #[test]
    fn test_both_changed_is_a_conflict() {
        assert_eq!(
            reconcile("local", Some("remote"), Some("base")),
            Outcome::Conflict
        );
    }

    #[test]
    fn test_first_sync() {
        // Nothing on GitHub yet.
        assert_eq!(reconcile("{\"a\": 1}", None, None), Outcome::Push);
        assert_eq!(
            reconcile("// comment\n{}\n", None, None),
            Outcome::Unchanged
        );
        // A fresh machine takes GitHub's copy.
        assert_eq!(
            reconcile("// Settings\n{\n}\n", Some("{\"a\": 1}"), None),
            Outcome::Pull
        );
        assert_eq!(reconcile("", Some("{\"a\": 1}"), None), Outcome::Pull);
        // A machine with its own settings has to choose.
        assert_eq!(
            reconcile("{\"b\": 2}", Some("{\"a\": 1}"), None),
            Outcome::Conflict
        );
    }

    #[test]
    fn test_is_untouched() {
        assert!(is_untouched(""));
        assert!(is_untouched("// Zed settings\n//\n{\n}\n"));
        assert!(is_untouched("[\n]"));
        assert!(!is_untouched("{ \"vim_mode\": true }"));
    }
}
