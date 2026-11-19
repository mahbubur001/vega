//! VEGA: docs links.
//!
//! Upstream code links to the docs on zed.dev in many places. VEGA has no docs
//! site and must never send people to zed.dev, so [`rewrite_docs_url`] maps
//! those links onto the same page of VEGA's own docs (`docs/src` in the VEGA
//! repo). The page list comes from `docs/src/SUMMARY.md` and the redirects in
//! `docs/book.toml`, embedded at build time, so a link never points at a page
//! that doesn't exist; anything unknown goes to the docs' table of contents.

use std::{collections::HashMap, sync::LazyLock};

/// Where VEGA's docs pages are read. Change this one line once the docs are
/// hosted somewhere else.
pub const VEGA_DOCS_BASE_URL: &str = "https://github.com/mahbubur001/vega/blob/vega/docs/src/";

const ZED_DOCS_PREFIXES: &[&str] = &["https://zed.dev/docs", "https://www.zed.dev/docs"];

/// Old slugs the app links to that `book.toml` doesn't redirect.
const ALIASES: &[(&str, &str)] = &[("settings", "reference/all-settings")];

const SUMMARY: &str = include_str!("../../../docs/src/SUMMARY.md");
const BOOK_TOML: &str = include_str!("../../../docs/book.toml");

/// Pages (paths under `docs/src`, without `.md`) listed in `SUMMARY.md`.
static PAGES: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    SUMMARY
        .split("](")
        .skip(1)
        .filter_map(|rest| rest.split(')').next())
        .filter_map(|target| target.strip_suffix(".md"))
        .map(|page| page.trim_start_matches("./"))
        .collect()
});

/// `book.toml` redirects, from an old slug to the target URL path.
static REDIRECTS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    BOOK_TOML
        .lines()
        .skip_while(|line| line.trim() != "[output.zed-html.redirect]")
        .skip(1)
        .take_while(|line| !line.trim_start().starts_with('['))
        .filter_map(|line| {
            let (from, to) = line.split_once(" = ")?;
            let from = from.trim().trim_matches('"').trim_start_matches('/');
            let to = to.trim().trim_matches('"');
            Some((from.strip_suffix(".html").unwrap_or(from), to))
        })
        .collect()
});

/// Returns VEGA's docs URL for a zed.dev docs URL, or `None` for any other URL.
pub fn rewrite_docs_url(url: &str) -> Option<String> {
    let rest = ZED_DOCS_PREFIXES
        .iter()
        .find_map(|prefix| url.strip_prefix(prefix))?;
    if !(rest.is_empty() || rest.starts_with(['/', '#', '?'])) {
        return None;
    }
    let (path, anchor) = match rest.split_once('#') {
        Some((path, anchor)) => (path, Some(anchor)),
        None => (rest, None),
    };
    let path = path.split('?').next().unwrap_or_default();
    let mut slug = path.trim_matches('/');
    for channel in ["nightly", "preview"] {
        if slug == channel {
            slug = "";
        } else if let Some(rest) = slug.strip_prefix(channel).and_then(|s| s.strip_prefix('/')) {
            slug = rest;
        }
    }
    let slug = slug
        .strip_suffix(".html")
        .or_else(|| slug.strip_suffix(".md"))
        .unwrap_or(slug);
    Some(docs_page_url(slug, anchor))
}

fn docs_page_url(slug: &str, anchor: Option<&str>) -> String {
    let mut anchor = anchor;
    let page = find_page(slug)
        .or_else(|| {
            let (_, target) = ALIASES.iter().find(|(from, _)| *from == slug)?;
            find_page(target)
        })
        .or_else(|| {
            // Follow one redirect, e.g. `ai` → `/docs/ai/overview.html`.
            let target = REDIRECTS.get(slug)?.strip_prefix("/docs/")?;
            let (target, target_anchor) = match target.split_once('#') {
                Some((target, target_anchor)) => (target, Some(target_anchor)),
                None => (target, None),
            };
            anchor = anchor.or(target_anchor);
            find_page(target.strip_suffix(".html").unwrap_or(target))
        });
    match (page, anchor) {
        (Some(page), Some(anchor)) => format!("{VEGA_DOCS_BASE_URL}{page}.md#{anchor}"),
        (Some(page), None) => format!("{VEGA_DOCS_BASE_URL}{page}.md"),
        (None, _) => format!("{VEGA_DOCS_BASE_URL}SUMMARY.md"),
    }
}

fn find_page(slug: &str) -> Option<&'static str> {
    if slug.is_empty() {
        return None;
    }
    PAGES.iter().copied().find(|page| *page == slug)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn page(path: &str) -> String {
        format!("{VEGA_DOCS_BASE_URL}{path}")
    }

    #[test]
    fn maps_zed_docs_pages_to_vega_docs() {
        assert_eq!(
            rewrite_docs_url("https://zed.dev/docs/remote-development"),
            Some(page("remote-development.md"))
        );
        assert_eq!(
            rewrite_docs_url("https://zed.dev/docs/linux#i-cant-open-any-files"),
            Some(page("linux.md#i-cant-open-any-files"))
        );
        assert_eq!(
            rewrite_docs_url("https://zed.dev/docs/preview/ai/overview.html"),
            Some(page("ai/overview.md"))
        );
        assert_eq!(
            rewrite_docs_url("https://zed.dev/docs/nightly/languages/python"),
            Some(page("languages/python.md"))
        );
    }

    #[test]
    fn follows_book_redirects_and_aliases() {
        assert_eq!(
            rewrite_docs_url("https://zed.dev/docs/ai.html"),
            Some(page("ai/overview.md"))
        );
        assert_eq!(
            rewrite_docs_url("https://zed.dev/docs/ai/temperature.html"),
            Some(page("ai/agent-settings.md#model-temperature"))
        );
        assert_eq!(
            rewrite_docs_url("https://zed.dev/docs/nightly/settings"),
            Some(page("reference/all-settings.md"))
        );
        assert_eq!(
            rewrite_docs_url("https://zed.dev/docs/ai/mcp.md"),
            Some(page("ai/mcp.md"))
        );
    }

    #[test]
    fn unknown_pages_and_the_root_go_to_the_table_of_contents() {
        for url in [
            "https://zed.dev/docs",
            "https://zed.dev/docs/",
            "https://zed.dev/docs/nightly",
            "https://zed.dev/docs/no-such-page",
        ] {
            assert_eq!(rewrite_docs_url(url), Some(page("SUMMARY.md")), "{url}");
        }
    }

    #[test]
    fn leaves_other_urls_alone() {
        for url in [
            "https://zed.dev/blog/acp-registry",
            "https://zed.dev/docsearch",
            "https://example.com/docs/settings",
        ] {
            assert_eq!(rewrite_docs_url(url), None, "{url}");
        }
    }

    #[test]
    fn every_upstream_docs_slug_the_app_builds_resolves_to_a_page() {
        // `ReleaseChannel::docs_url` slugs used across the app.
        for slug in ["ai/sandboxing", "ai/llm-providers", "tasks", "debugger"] {
            let url = rewrite_docs_url(&format!("https://zed.dev/docs/{slug}")).unwrap();
            assert!(!url.ends_with("SUMMARY.md"), "{slug} → {url}");
        }
    }
}
