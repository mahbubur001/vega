//! VEGA: settings for Settings Sync (see `vega/SETTINGS_SYNC.md`).

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, with_fallible_options};

#[with_fallible_options]
#[derive(Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug)]
pub struct SettingsSyncContent {
    /// Settings this machine never uploads to GitHub or takes from it, as
    /// top-level keys or dotted paths, e.g. `"buffer_font_size"` or
    /// `"terminal.font_size"`. This block itself is never synced, so each
    /// machine has its own list.
    ///
    /// Default: []
    pub ignored_settings: Option<Vec<String>>,
}
