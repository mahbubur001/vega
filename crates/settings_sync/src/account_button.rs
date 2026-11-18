//! The GitHub account item shown in the title bar.

use gpui::{Action as _, Animation, AnimationExt as _, Entity, Subscription, pulsating_between};
use std::time::Duration;
use ui::{Avatar, ButtonLike, ContextMenu, Indicator, PopoverMenu, Tooltip, prelude::*};

use crate::account::{AccountStatus, GitHubAccount};
use crate::engine::{SettingsSync, SyncStatus};
use crate::{
    SignIn, SignOut, SyncNow, TurnOffSync, TurnOnSync, UseLocalSettings, UseSyncedSettings,
};

pub struct AccountButton {
    account: Option<Entity<GitHubAccount>>,
    sync: Option<Entity<SettingsSync>>,
    _subscriptions: Vec<Subscription>,
}

impl AccountButton {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let account = GitHubAccount::global(cx);
        let sync = SettingsSync::global(cx);
        let mut subscriptions = Vec::new();
        if let Some(account) = &account {
            subscriptions.push(cx.observe(account, |_, _, cx| cx.notify()));
        }
        if let Some(sync) = &sync {
            subscriptions.push(cx.observe(sync, |_, _, cx| cx.notify()));
        }
        Self {
            account,
            sync,
            _subscriptions: subscriptions,
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
                let (sync_status, sync_enabled) = self
                    .sync
                    .as_ref()
                    .map(|sync| {
                        let sync = sync.read(cx);
                        (sync.status().clone(), sync.is_enabled())
                    })
                    .unwrap_or((SyncStatus::Off, false));
                let profile_url = format!("https://github.com/{}", user.login);
                let login = SharedString::from(format!("@{}", user.login));
                let status_label = status_label(&sync_status);

                let indicator_color = match &sync_status {
                    SyncStatus::Conflict(_) => Some(Color::Warning),
                    SyncStatus::Error(_) => Some(Color::Error),
                    _ => None,
                };
                let avatar =
                    Avatar::new(user.avatar_url).when_some(indicator_color, |avatar, color| {
                        avatar.indicator(
                            div()
                                .absolute()
                                .bottom_0()
                                .right_0()
                                .child(Indicator::dot().color(color)),
                        )
                    });

                PopoverMenu::new("github-account-menu")
                    .trigger_with_tooltip(
                        ButtonLike::new("github-account")
                            .aria_label("GitHub account")
                            .child(avatar),
                        Tooltip::text(format!("{login} · {status_label}")),
                    )
                    .menu(move |window, cx| {
                        let profile_url = profile_url.clone();
                        let login = login.clone();
                        let sync_status = sync_status.clone();
                        let status_label = status_label.clone();
                        Some(ContextMenu::build(window, cx, move |menu, _, _| {
                            let menu = menu
                                .entry(login, None, move |_, cx| cx.open_url(&profile_url))
                                .separator()
                                .header(status_label);
                            let menu = match &sync_status {
                                SyncStatus::Conflict(_) => menu
                                    .action(
                                        "Keep This Machine's Settings",
                                        UseLocalSettings.boxed_clone(),
                                    )
                                    .action(
                                        "Use Settings from GitHub",
                                        UseSyncedSettings.boxed_clone(),
                                    ),
                                _ if sync_enabled => menu.action("Sync Now", SyncNow.boxed_clone()),
                                _ => menu,
                            };
                            let menu = if sync_enabled {
                                menu.action("Turn Off Settings Sync", TurnOffSync.boxed_clone())
                            } else {
                                menu.action("Turn On Settings Sync", TurnOnSync.boxed_clone())
                            };
                            menu.separator()
                                .action("Sign Out of GitHub", SignOut.boxed_clone())
                        }))
                    })
                    .into_any_element()
            }
        }
    }
}

fn status_label(status: &SyncStatus) -> SharedString {
    match status {
        SyncStatus::Off => "Settings Sync is off".into(),
        SyncStatus::Idle => "Settings are synced".into(),
        SyncStatus::Syncing => "Syncing settings…".into(),
        SyncStatus::Conflict(labels) => format!("Sync conflict: {}", labels.join(", ")).into(),
        SyncStatus::Error(message) => format!("Sync failed: {message}").into(),
    }
}
