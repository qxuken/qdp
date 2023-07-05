pub mod entities;
pub mod error;
pub mod schema;

use diesel::{r2d2, SqliteConnection};

use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::{embed_migrations, MigrationHarness};

pub use entities::*;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub type DatabasePool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;

pub fn run_migrations(connection: DatabasePool) -> () {
    if let Ok(mut connection) = connection.get() {
        log::info!("Running migrations");
        connection.run_pending_migrations(MIGRATIONS).unwrap();
    }
}

pub fn initialize_db_pool(database_url: Option<String>) -> DatabasePool {
    let database_url = database_url.unwrap_or("./database/qdp.db".to_string());
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("database URL should be valid path to SQLite DB file")
}
