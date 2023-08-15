use crate::{error::Error, result::Result};
use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    SqliteConnection,
};
use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::{embed_migrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub type DatabasePool = Pool<ConnectionManager<SqliteConnection>>;

#[derive(Clone)]
pub struct Database {
    pool: DatabasePool,
}

impl Database {
    pub fn new(database_url: Option<String>) -> Self {
        let database_url = database_url.unwrap_or("./database/qdp.db".to_string());
        let manager = ConnectionManager::<SqliteConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("database URL should be valid path to SQLite DB file");
        Database { pool }
    }

    pub fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>> {
        self.pool.get().map_err(|_| Error::DatabaseTimeout)
    }

    pub fn run_migrations(&self) {
        let mut connection = self.get_connection().unwrap();
        log::info!("Running migrations");
        connection.run_pending_migrations(MIGRATIONS).unwrap();
    }
}
