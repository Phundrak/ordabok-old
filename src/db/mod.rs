pub mod models;
pub mod schema;

use self::models::languages::Language;
use self::models::users::User;
use self::models::words::Word;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use dotenvy::dotenv;
use std::env;
use tracing::info;

macro_rules! find_element {
    ($conn:expr,$dsl:ident,$type:ty,$value:expr,$errmsg:expr) => {
        if let Ok(val) = $conn {
            $dsl.find($value).first::<$type>(val).map_or_else(
                |e| {
                    info!("{}: {:?}", $errmsg, e);
                    None
                },
                Some,
            )
        } else {
            info!("Failed to obtain connection for the database");
            None
        }
    };
}

use diesel::prelude::*;

pub struct Database {
    conn: Pool<ConnectionManager<PgConnection>>,
}

impl juniper::Context for Database {}

impl Default for Database {
    fn default() -> Self {
        Self {
            conn: Database::get_connection_pool(),
        }
    }
}

impl Database {
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

    pub fn language(&self, name: &str, owner: &str) -> Option<Language> {
        use self::schema::languages::dsl;
        match &mut self.conn() {
            Ok(conn) => match dsl::languages
                .filter(dsl::name.eq(name))
                .filter(dsl::owner.eq(owner))
                .first::<Language>(conn)
            {
                Ok(val) => Some(val),
                Err(e) => {
                    info!("Could not retrieve language {} of user {} from database: {:?}",
                          name, owner, e);
                    None
                }
            },
            Err(e) => {
                info!("Could not connect to the database: {:?}", e);
                None
            }
        }
    }

    pub fn user(&self, id: &str) -> Option<User> {
        use self::schema::users::dsl::users;
        find_element!(
            &mut self.conn(),
            users,
            User,
            id.to_string(),
            format!("Failed to retrieve user {} from database", id)
        )
    }

    pub fn word_id(&self, id: &str) -> Option<Word> {
        use self::schema::words::dsl;
        if let Ok(conn) = &mut self.conn() {
            match dsl::words.find(id).first::<Word>(conn) {
                Ok(val) => Some(val),
                Err(e) => {
                    info!("Error retrieving {}: {:?}", id, e);
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn words(&self, language: uuid::Uuid, word: &str) -> Vec<Word> {
        use self::schema::words::dsl;
        if let Ok(conn) = &mut self.conn() {
            match dsl::words
                .filter(dsl::language.eq(language))
                .filter(dsl::norm.eq(word))
                .load::<Word>(conn)
            {
                Ok(val) => val,
                Err(e) => {
                    info!(
                        "Error retrieving {} from language {}: {:?}",
                        word, language, e
                    );
                    Vec::new()
                }
            }
        } else {
            Vec::new()
        }
    }
}
