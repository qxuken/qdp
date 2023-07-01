pub mod models;
pub mod schema;

use diesel::prelude::*;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").unwrap_or("./database/qdp.db".to_string());
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {database_url}"))
}
