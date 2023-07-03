pub mod entities;
pub mod error;
pub mod schema;

use diesel::{prelude::*, r2d2};
use std::env;

pub use entities::*;

pub type DatabasePool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").unwrap_or("./database/qdp.db".to_string());
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}

pub fn initialize_db_pool() -> DatabasePool {
    let database_url = env::var("DATABASE_URL").unwrap_or("./database/qdp.db".to_string());
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file")
}
