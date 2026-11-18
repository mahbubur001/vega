//! Deployment configuration: model, lenient normalisation and the file store.

mod model;
pub mod normalize;
mod store;

pub use model::*;
pub use store::ConfigStore;
