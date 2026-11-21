//! VEGA: settings for the TODO tool window (IntelliJ's TODO patterns).

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings_macros::{MergeFrom, with_fallible_options};

#[with_fallible_options]
#[derive(Clone, PartialEq, Default, Serialize, Deserialize, JsonSchema, MergeFrom, Debug)]
pub struct TodoSettingsContent {
    /// Words that mark a TODO item in a comment, matched as whole words.
    ///
    /// Default: ["TODO", "FIXME"]
    pub patterns: Option<Vec<String>>,
    /// Whether the words must match case exactly (`TODO` but not `todo`).
    ///
    /// Default: false
    pub case_sensitive: Option<bool>,
    /// Whether TODO items are highlighted in the editor.
    ///
    /// Default: true
    pub highlight: Option<bool>,
}
