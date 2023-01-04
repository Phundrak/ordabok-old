pub mod models;
pub mod schema;

use self::models::languages::Language;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use dotenvy::dotenv;
use std::env;
use tracing::info;

use diesel::prelude::*;
// use diesel::query_dsl::RunQueryDsl;

pub struct Database {
    conn: Pool<ConnectionManager<PgConnection>>,
}

impl juniper::Context for Database {}

impl Database {
    pub fn new() -> Self {
        Self {
            conn: Database::get_connection_pool(),
        }
    }

    pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
        dotenv().ok();
        let database_url =
            env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
        info!("Connecting to {}", database_url);
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        Pool::builder()
            .test_on_check_out(true)
            .build(manager)
            .expect("Could not build connection pool")
    }

    fn conn(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, ()> {
        self.conn
            .get()
            .map_err(|e| info!("Failed to connect to database: {:?}", e))
    }

    pub fn all_languages(&self) -> Result<Vec<Language>, ()> {
        use self::schema::languages::dsl::languages;
        languages.load::<Language>(&mut self.conn()?).map_err(|e| {
            info!("Failed to retrieve languages from database: {:?}", e);
        })
    }

    pub fn language(&self, name: &str) -> Option<Language> {
        use self::schema::languages::dsl::languages;
        match &mut self.conn() {
            Ok(val) => languages
                .find(name.to_string())
                .first::<Language>(val)
                .map_or_else(
                    |e| {
                        info!(
                        "Failed to retrieve language {} from database: {:?}",
                        name, e
                    );
                        None
                    },
                    Some,
                ),
            Err(_) => None,
        }
    }
}
