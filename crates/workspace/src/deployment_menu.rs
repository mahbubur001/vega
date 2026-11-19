//! VEGA: the PhpStorm-style "Deployment" context submenu, shared by the project
//! panel, editor and tab context menus so each upstream menu only needs one line.

use std::path::Path;

use gpui::{App, FocusHandle};
use ui::{ContextMenu, FluentBuilder as _};
use zed_actions::deployment;

pub trait DeploymentMenuExt: Sized {
    /// Appends a separator and the "Deployment" submenu when `path` is inside a
    /// project root that has a deployment configuration. `focus` is where the
    /// submenu's actions are dispatched, like the parent menu's `context`.
    fn deployment_submenu(
        self,
        path: Option<&Path>,
        is_file: bool,
        focus: Option<FocusHandle>,
        cx: &App,
    ) -> Self;
}

impl DeploymentMenuExt for ContextMenu {
    fn deployment_submenu(
        self,
        path: Option<&Path>,
        is_file: bool,
        focus: Option<FocusHandle>,
        cx: &App,
    ) -> Self {
        let configured = path.is_some_and(|path| deployment::is_configured(path, cx));
        self.when(configured, |menu| {
            menu.separator()
                .submenu(deployment::MENU_LABEL, move |menu, _, _| {
                    let menu = match &focus {
                        Some(focus) => menu.context(focus.clone()),
                        None => menu,
                    };
                    deployment::menu_items(is_file).into_iter().fold(
                        menu,
                        |menu, item| match item {
                            Some((label, action)) => menu.action(label, action),
                            None => menu.separator(),
                        },
                    )
                })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn labels(is_file: bool) -> Vec<&'static str> {
        deployment::menu_items(is_file)
            .into_iter()
            .map(|item| item.map_or("---", |(label, _)| label))
            .collect()
    }

    #[test]
    fn menu_items_follow_phpstorm_order() {
        assert_eq!(
            labels(true),
            [
                "Upload to Default Server",
                "Download from Default Server",
                "Compare with Deployed Version",
                "Sync with Deployed…",
                "---",
                "Configuration…",
            ]
        );
    }

    #[test]
    fn folders_cannot_be_compared() {
        assert!(!labels(false).contains(&"Compare with Deployed Version"));
    }

    #[gpui::test]
    fn is_configured_matches_whole_root_paths(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            cx.set_global(deployment::ConfiguredRoots(
                [std::path::PathBuf::from("/project")].into_iter().collect(),
            ));
            assert!(deployment::is_configured(
                Path::new("/project/src/main.rs"),
                cx
            ));
            assert!(!deployment::is_configured(Path::new("/other/main.rs"), cx));
            assert!(!deployment::is_configured(
                Path::new("/project-two/a.rs"),
                cx
            ));
        });
    }
}
