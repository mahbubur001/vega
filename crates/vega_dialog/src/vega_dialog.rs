//! VEGA's shared dialog look (buttons, footers, combo boxes) and the name
//! prompt, used by Deployment, Scratches and External Libraries.

pub mod dialog;
pub mod name_modal;

pub use name_modal::{NameModal, name_problem, path_problem};
