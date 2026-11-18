//! The GitHub account in the title bar: a Sign In button while signed out,
//! and entries for the title bar's user menu while signed in.

use gpui::{
    Action as _, Animation, AnimationExt as _, App, Entity, Subscription, pulsating_between,
};
use std::time::Duration;
use ui::{Avatar, ContextMenu, Indicator, Tooltip, prelude::*};

use crate::account::{AccountStatus, GitHubAccount};
use crate::engine::{SettingsSync, SyncStatus};
use crate::{
    SignIn, SignOut, SyncNow, TurnOffSync, TurnOnSync, UseLocalSettings, UseSyncedSettings,
};

/// Shows Sign In, or progress while signing in. Renders nothing once signed
/// in; the account then lives in the user menu.
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

        match account.read(cx).status() {
            AccountStatus::Starting | AccountStatus::SignedIn(_) => div().into_any_element(),
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
        }
    }
}

/// Re-renders the observing view when the account or sync status changes.
pub fn observe_account<T: 'static>(cx: &mut Context<T>) -> Vec<Subscription> {
    let mut subscriptions = Vec::new();
    if let Some(account) = GitHubAccount::global(cx) {
        subscriptions.push(cx.observe(&account, |_, _, cx| cx.notify()));
    }
    if let Some(sync) = SettingsSync::global(cx) {
        subscriptions.push(cx.observe(&sync, |_, _, cx| cx.notify()));
    }
    subscriptions
}

/// The signed-in user's avatar, with a dot when sync needs attention.
pub fn account_avatar(cx: &App) -> Option<Avatar> {
    let user = GitHubAccount::global(cx)?.read(cx).user()?.clone();
    let indicator_color = match sync_state(cx).0 {
        SyncStatus::Conflict(_) => Some(Color::Warning),
        SyncStatus::Error(_) => Some(Color::Error),
        _ => None,
    };
    Some(
        Avatar::new(user.avatar_url).when_some(indicator_color, |avatar, color| {
            avatar.indicator(
                div()
                    .absolute()
                    .bottom_0()
                    .right_0()
                    .child(Indicator::dot().color(color)),
            )
        }),
    )
}

/// Adds the GitHub account and Settings Sync entries to the top of the user
/// menu. Adds nothing while signed out.
pub fn add_account_entries(menu: ContextMenu, cx: &App) -> ContextMenu {
    let Some(user) = GitHubAccount::global(cx).and_then(|account| account.read(cx).user().cloned())
    else {
        return menu;
    };
    let (sync_status, sync_enabled, last_synced) = sync_state(cx);
    let profile_url = format!("https://github.com/{}", user.login);

    let menu = menu
        .custom_entry(
            {
                let login = format!("@{}", user.login);
                move |_, _| {
                    h_flex()
                        .w_full()
                        .gap_2()
                        .child(Icon::new(IconName::Github).size(IconSize::Small))
                        .child(Label::new(login.clone()))
                        .into_any_element()
                }
            },
            move |_, cx| cx.open_url(&profile_url),
        )
        .separator()
        .header(status_label(&sync_status, last_synced));
    let menu = match &sync_status {
        SyncStatus::Conflict(_) => menu
            .action(
                "Keep This Machine's Settings",
                UseLocalSettings.boxed_clone(),
            )
            .action("Use Settings from GitHub", UseSyncedSettings.boxed_clone()),
        _ if sync_enabled => menu.action("Sync Now", SyncNow.boxed_clone()),
        _ => menu,
    };
    let menu = if sync_enabled {
        menu.action("Turn Off Settings Sync", TurnOffSync.boxed_clone())
    } else {
        menu.action("Turn On Settings Sync", TurnOnSync.boxed_clone())
    };
    menu.separator()
}

/// Adds "Sign Out" at the bottom of the user menu while signed in.
pub fn add_sign_out_entry(menu: ContextMenu, cx: &App) -> ContextMenu {
    let is_signed_in =
        GitHubAccount::global(cx).is_some_and(|account| account.read(cx).user().is_some());
    if is_signed_in {
        menu.separator().action("Sign Out", SignOut.boxed_clone())
    } else {
        menu
    }
}

fn sync_state(cx: &App) -> (SyncStatus, bool, Option<std::time::SystemTime>) {
    SettingsSync::global(cx)
        .map(|sync| {
            let sync = sync.read(cx);
            (sync.status().clone(), sync.is_enabled(), sync.last_synced())
        })
        .unwrap_or((SyncStatus::Off, false, None))
}

/// "just now", "5 min ago", "3 h ago" or "2 days ago".
fn time_ago(elapsed: std::time::Duration) -> String {
    let minutes = elapsed.as_secs() / 60;
    match minutes {
        0 => "just now".into(),
        1..=59 => format!("{minutes} min ago"),
        60..=1439 => format!("{} h ago", minutes / 60),
        _ => match minutes / 1440 {
            1 => "1 day ago".into(),
            days => format!("{days} days ago"),
        },
    }
}

fn status_label(status: &SyncStatus, last_synced: Option<std::time::SystemTime>) -> SharedString {
    let since = last_synced
        .and_then(|time| time.elapsed().ok())
        .map(time_ago);
    match status {
        SyncStatus::Off => "Settings Sync is off".into(),
        SyncStatus::Idle => match since {
            Some(since) => format!("Settings are synced · {since}").into(),
            None => "Settings are synced".into(),
        },
        SyncStatus::Syncing => "Syncing settings…".into(),
        SyncStatus::Conflict(labels) => format!("Sync conflict: {}", labels.join(", ")).into(),
        SyncStatus::Error(message) => format!("Sync failed: {message}").into(),
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn time_ago_reads_naturally() {
        assert_eq!(time_ago(Duration::from_secs(20)), "just now");
        assert_eq!(time_ago(Duration::from_secs(5 * 60)), "5 min ago");
        assert_eq!(time_ago(Duration::from_secs(3 * 3600 + 10)), "3 h ago");
        assert_eq!(time_ago(Duration::from_secs(26 * 3600)), "1 day ago");
        assert_eq!(time_ago(Duration::from_secs(3 * 86400)), "3 days ago");
    }

    #[test]
    fn idle_status_shows_the_last_sync() {
        let an_hour_ago = std::time::SystemTime::now() - Duration::from_secs(3600);
        assert_eq!(
            status_label(&SyncStatus::Idle, Some(an_hour_ago)).as_ref(),
            "Settings are synced · 1 h ago"
        );
        assert_eq!(
            status_label(&SyncStatus::Idle, None).as_ref(),
            "Settings are synced"
        );
    }
}
