pub mod models;
pub mod schema;

use self::models::languages::Language;
use self::models::users::User;
use self::models::words::Word;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::{insert_into, prelude::*};

use dotenvy::dotenv;
use juniper::{graphql_value, DefaultScalarValue, FieldError, IntoFieldError};
use std::env;
use std::error::Error;
use tracing::info;

#[derive(Debug)]
pub struct DatabaseError {
    long: String,
    #[allow(dead_code)]
    short: String,
}

impl DatabaseError {
    #[allow(clippy::needless_pass_by_value)]
    pub fn new<S, T>(long: S, short: T) -> Self
    where
        T: ToString,
        S: ToString,
    {
        Self {
            long: long.to_string(),
            short: short.to_string(),
        }
    }
}

impl Error for DatabaseError {}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.long)
    }
}

impl IntoFieldError for DatabaseError {
    fn into_field_error(self) -> juniper::FieldError<DefaultScalarValue> {
        FieldError::new(
            self.long,
            graphql_value!({ "error": "Connection refused" }),
        )
    }
}

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

#[derive(Debug, Clone)]
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
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, DatabaseError>
    {
        self.conn.get().map_err(|e| {
            DatabaseError::new(
                format!("Failed to connect to database: {:?}", e),
                "Database connection error",
            )
        })
    }

    pub fn all_languages(&self) -> Result<Vec<Language>, DatabaseError> {
        use self::schema::languages::dsl::languages;
        languages
            .load::<Language>(&mut self.conn()?)
            .map_err(|e| {
                info!("Failed to retrieve languages from database: {:?}", e);
            })
            .map_err(|e| {
                DatabaseError::new(
                    format!("Failed to retrieve languages: {:?}", e),
                    "Failed to retrieve languages",
                )
            })
    }

    pub fn all_users(&self) -> Result<Vec<User>, DatabaseError> {
        use self::schema::users::dsl::users;
        users
            .load::<User>(&mut self.conn()?)
            .map_err(|e| {
                info!("Failed to retrieve languages from database: {:?}", e);
            })
            .map_err(|e| {
                DatabaseError::new(
                    format!("Failed to retrieve languages: {:?}", e),
                    "Failed to retrieve languages",
                )
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

    pub fn insert_user(
        &self,
        username: String,
        id: String,
    ) -> Result<User, DatabaseError> {
        use self::schema::users::dsl::users;
        let user = User { id, username };
        match insert_into(users).values(user.clone()).execute(
            &mut self.conn().map_err(|e| {
                DatabaseError::new(
                    format!("Failed to connect to the database: {:?}", e),
                    "Connection error",
                )
            })?,
        ) {
            Ok(_) => Ok(user),
            Err(e) => Err(DatabaseError {
                long: format!("Failed to insert user {:?}: {:?}", user, e),
                short: "Data insertion error".to_string(),
            }),
        }
    }

    pub fn delete_user(&self, id: &str) -> Result<(), DatabaseError> {
        use self::schema::users::dsl::users;
        match diesel::delete(users.find(id.to_string()))
            .execute(&mut self.conn()?)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(DatabaseError::new(
                format!("Failed to delete user {}: {:?}", id, e),
                "User deletion error",
            )),
        }
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
