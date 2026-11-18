//! Settings Sync: sign in with GitHub and keep VEGA's settings in sync across
//! machines through a secret Gist. See `vega/SETTINGS_SYNC.md` for the design.

mod account;
mod account_button;
mod github;
mod sign_in_window;

use std::sync::Arc;

use credentials_provider::CredentialsProvider;
use gpui::{App, TaskExt as _, Window, actions};
use http_client::HttpClient;
use workspace::Workspace;

pub use account::{AccountStatus, GitHubAccount};
pub use account_button::AccountButton;
pub use github::GitHubUser;

actions!(
    settings_sync,
    [
        /// Signs in with GitHub to sync your settings across machines.
        SignIn,
        /// Signs out of GitHub on this machine.
        SignOut,
    ]
);

pub fn init(
    http_client: Arc<dyn HttpClient>,
    credentials_provider: Arc<dyn CredentialsProvider>,
    cx: &mut App,
) {
    GitHubAccount::init(http_client, credentials_provider, cx);

    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        workspace.register_action(|_, _: &SignIn, window, cx| sign_in(window, cx));
        workspace.register_action(|_, _: &SignOut, _, cx| sign_out(cx));
    })
    .detach();
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
