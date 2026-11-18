//! Settings Sync: sign in with GitHub and keep VEGA's settings in sync across
//! machines through a secret Gist. See `vega/SETTINGS_SYNC.md` for the design.

mod account;
mod account_button;
mod engine;
mod gist;
mod github;
mod history_picker;
mod merge;
mod reconcile;
mod resources;
mod sign_in_window;

use std::sync::Arc;

use credentials_provider::CredentialsProvider;
use fs::Fs;
use gpui::{App, AppContext as _, DismissEvent, Entity, TaskExt as _, Window, actions};
use http_client::HttpClient;
use ui::IconName;
use workspace::Workspace;
use workspace::notifications::{
    NotificationId, dismiss_app_notification, show_app_notification,
    simple_message_notification::MessageNotification,
};

pub use account::{AccountStatus, GitHubAccount};
pub use account_button::{
    AccountButton, account_avatar, add_account_entries, add_sign_out_entry, observe_account,
};
pub use engine::{SettingsSync, SyncStatus};
pub use github::GitHubUser;

use engine::Resolution;

actions!(
    settings_sync,
    [
        /// Signs in with GitHub to sync your settings across machines.
        SignIn,
        /// Signs out of GitHub on this machine.
        SignOut,
        /// Syncs your settings with GitHub now.
        SyncNow,
        /// Turns Settings Sync on for this machine.
        TurnOnSync,
        /// Turns Settings Sync off for this machine. Your settings stay on GitHub.
        TurnOffSync,
        /// Settles a sync conflict by keeping this machine's settings.
        UseLocalSettings,
        /// Settles a sync conflict by taking the settings from GitHub.
        UseSyncedSettings,
        /// Shows earlier versions of your synced settings and restores one.
        ShowHistory,
    ]
);

struct ConflictNotification;

pub fn init(
    fs: Arc<dyn Fs>,
    http_client: Arc<dyn HttpClient>,
    credentials_provider: Arc<dyn CredentialsProvider>,
    cx: &mut App,
) {
    GitHubAccount::init(http_client.clone(), credentials_provider, cx);
    let Some(account) = GitHubAccount::global(cx) else {
        return;
    };
    SettingsSync::init(account, fs, http_client, cx);
    if let Some(sync) = SettingsSync::global(cx) {
        notify_on_conflict(&sync, cx);
    }

    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        workspace.register_action(|_, _: &SignIn, window, cx| sign_in(window, cx));
        workspace.register_action(|_, _: &SignOut, _, cx| sign_out(cx));
        workspace.register_action(|workspace, _: &ShowHistory, window, cx| {
            history_picker::toggle(workspace, window, cx)
        });
        workspace
            .register_action(|_, _: &SyncNow, _, cx| update_sync(cx, |sync, cx| sync.sync(cx)));
        workspace.register_action(|_, _: &TurnOnSync, _, cx| {
            update_sync(cx, |sync, cx| sync.set_enabled(true, cx))
        });
        workspace.register_action(|_, _: &TurnOffSync, _, cx| {
            update_sync(cx, |sync, cx| sync.set_enabled(false, cx))
        });
        workspace.register_action(|_, _: &UseLocalSettings, _, cx| {
            update_sync(cx, |sync, cx| sync.resolve(Resolution::UseLocal, cx))
        });
        workspace.register_action(|_, _: &UseSyncedSettings, _, cx| {
            update_sync(cx, |sync, cx| sync.resolve(Resolution::UseRemote, cx))
        });
    })
    .detach();
}

fn update_sync(
    cx: &mut App,
    update: impl FnOnce(&mut SettingsSync, &mut gpui::Context<SettingsSync>),
) {
    if let Some(sync) = SettingsSync::global(cx) {
        sync.update(cx, update);
    }
}

fn sign_in(window: &mut Window, cx: &mut App) {
    let Some(account) = GitHubAccount::global(cx) else {
        return;
    };
    account.update(cx, |account, cx| account.sign_in(cx));
    sign_in_window::open(&account, window, cx);
}

fn sign_out(cx: &mut App) {
    let Some(account) = GitHubAccount::global(cx) else {
        return;
    };
    account
        .update(cx, |account, cx| account.sign_out(cx))
        .detach_and_log_err(cx);
}

/// Asks the user how to settle a conflict whenever one appears.
fn notify_on_conflict(sync: &Entity<SettingsSync>, cx: &mut App) {
    let mut showing_conflict = false;
    cx.observe(sync, move |sync, cx| {
        let id = NotificationId::unique::<ConflictNotification>();
        match sync.read(cx).status().clone() {
            SyncStatus::Conflict(labels) => {
                if showing_conflict {
                    return;
                }
                showing_conflict = true;
                let message = format!(
                    "{} changed on this machine and on GitHub since the last sync. \
                     Which version should VEGA keep? The other one is backed up.",
                    labels.join(" and ")
                );
                show_app_notification(id, cx, move |cx| {
                    cx.new(|cx| {
                        MessageNotification::new(message.clone(), cx)
                            .primary_message("Keep This Machine's")
                            .primary_icon(IconName::Check)
                            .primary_on_click(|window, cx| {
                                window.dispatch_action(Box::new(UseLocalSettings), cx);
                                cx.emit(DismissEvent);
                            })
                            .secondary_message("Use GitHub's")
                            .secondary_icon(IconName::Github)
                            .secondary_on_click(|window, cx| {
                                window.dispatch_action(Box::new(UseSyncedSettings), cx);
                                cx.emit(DismissEvent);
                            })
                    })
                });
            }
            SyncStatus::Syncing => {}
            _ => {
                if std::mem::take(&mut showing_conflict) {
                    dismiss_app_notification(&id, cx);
                }
            }
        }
    })
    .detach();
}
