pub mod app_state;
pub mod assets;
pub mod database;
pub mod entities;
pub mod error;
pub mod result;
pub mod routes;
pub mod schema;
pub mod templates;

pub use app_state::{AppState, SharedAppState};
pub use database::Database;
