//! The GitHub account item shown in the title bar.

use gpui::{Action as _, Animation, AnimationExt as _, Entity, Subscription, pulsating_between};
use std::time::Duration;
use ui::{Avatar, ButtonLike, ContextMenu, PopoverMenu, Tooltip, prelude::*};

use crate::account::{AccountStatus, GitHubAccount};
use crate::{SignIn, SignOut};

pub struct AccountButton {
    account: Option<Entity<GitHubAccount>>,
    _subscription: Option<Subscription>,
}

impl AccountButton {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let account = GitHubAccount::global(cx);
        let subscription = account
            .as_ref()
            .map(|account| cx.observe(account, |_, _, cx| cx.notify()));
        Self {
            account,
            _subscription: subscription,
        }
    }
}

impl Render for AccountButton {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(account) = &self.account else {
            return div().into_any_element();
        };

        match account.read(cx).status().clone() {
            AccountStatus::Starting => div().into_any_element(),
            AccountStatus::SignedOut | AccountStatus::Error(_) => {
                Button::new("github-sign-in", "Sign In")
                    .label_size(LabelSize::Small)
                    .start_icon(Icon::new(IconName::Github).size(IconSize::Small))
                    .tooltip(Tooltip::text("Sign in with GitHub to sync settings"))
                    .on_click(|_, window, cx| window.dispatch_action(SignIn.boxed_clone(), cx))
                    .into_any_element()
            }
            AccountStatus::SigningIn { .. } => Label::new("Signing in…")
                .size(LabelSize::Small)
                .color(Color::Muted)
                .with_animation(
                    "github-signing-in",
                    Animation::new(Duration::from_secs(2))
                        .repeat()
                        .with_easing(pulsating_between(0.4, 0.8)),
                    |label, delta| label.alpha(delta),
                )
                .into_any_element(),
            AccountStatus::SignedIn(user) => {
                let profile_url = format!("https://github.com/{}", user.login);
                let login = SharedString::from(format!("@{}", user.login));

                PopoverMenu::new("github-account-menu")
                    .trigger_with_tooltip(
                        ButtonLike::new("github-account")
                            .aria_label("GitHub account")
                            .child(Avatar::new(user.avatar_url)),
                        Tooltip::text(format!("Signed in to GitHub as {login}")),
                    )
                    .menu(move |window, cx| {
                        let profile_url = profile_url.clone();
                        let login = login.clone();
                        Some(ContextMenu::build(window, cx, move |menu, _, _| {
                            menu.entry(login, None, move |_, cx| cx.open_url(&profile_url))
                                .separator()
                                .action("Sign Out of GitHub", SignOut.boxed_clone())
                        }))
                    })
                    .into_any_element()
            }
        }
    }
}
