pub mod models;
pub mod schema;

use self::models::languages::Language;
use self::models::users::User;
use self::models::words::Word;

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::result::Error;
use diesel::{insert_into, prelude::*};

use dotenvy::dotenv;
use juniper::{graphql_value, DefaultScalarValue, FieldError, IntoFieldError};
use std::env;
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

impl std::error::Error for DatabaseError {}

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
        users.load::<User>(&mut self.conn()?).map_err(|e| {
            DatabaseError::new(
                format!("Failed to retrieve languages: {:?}", e),
                "Failed to retrieve languages",
            )
        })
    }

    pub fn find_language(
        &self,
        query: &str,
    ) -> Result<Vec<Language>, DatabaseError> {
        use self::schema::languages::dsl;
        dsl::languages
            .filter(dsl::name.ilike(format!("%{}%", query)))
            .load::<Language>(&mut self.conn()?)
            .map_err(|e| {
                DatabaseError::new(
                    format!(
                        "Failed to retrieve languages with query {}: {:?}",
                        query, e
                    ),
                    "Failed to retrieve languages",
                )
            })
    }

    pub fn find_user(&self, query: &str) -> Result<Vec<User>, DatabaseError> {
        use self::schema::users::dsl;
        dsl::users
            .filter(dsl::username.ilike(format!("%{}%", query)))
            .load::<User>(&mut self.conn()?)
            .map_err(|e| {
                DatabaseError::new(
                    format!(
                        "Failed to retrieve users with query {}: {:?}",
                        query, e
                    ),
                    "Failed to retrieve languages",
                )
            })
    }

    pub fn language(
        &self,
        name: &str,
        owner: &str,
    ) -> Result<Option<Language>, DatabaseError> {
        use self::schema::languages::dsl;
        match dsl::languages
            .filter(dsl::name.eq(name))
            .filter(dsl::owner.eq(owner))
            .first(&mut self.conn()?)
        {
            Ok(val) => Ok(Some(val)),
            Err(Error::NotFound) => Ok(None),
            Err(e) => Err(DatabaseError::new(
                format!(
                    "Failed to find language {} belonging to {}: {:?}",
                    name, owner, e
                ),
                "Database error",
            )),
        }
    }

    pub fn user(&self, id: &str) -> Result<Option<User>, DatabaseError> {
        use self::schema::users::dsl::users;
        match users.find(id).first::<User>(&mut self.conn()?) {
            Ok(val) => Ok(Some(val)),
            Err(Error::NotFound) => Ok(None),
            Err(e) => Err(DatabaseError::new(
                format!(
                    "Failed to retrieve user {} from database: {:?}",
                    id, e
                ),
                "Database Error",
            )),
        }
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

    pub fn word_id(&self, id: &str) -> Result<Option<Word>, DatabaseError> {
        use self::schema::words::dsl;
        match dsl::words.find(id).first::<Word>(&mut self.conn()?) {
            Ok(val) => Ok(Some(val)),
            Err(Error::NotFound) => Ok(None),
            Err(e) => Err(DatabaseError::new(
                format!(
                    "Failed to retrieve word {} from database: {:?}",
                    id, e
                ),
                "Database Error",
            )),
        }
    }

    pub fn words(
        &self,
        language: uuid::Uuid,
        word: &str,
    ) -> Result<Vec<Word>, DatabaseError> {
        use self::schema::words::dsl;
        dsl::words
            .filter(dsl::language.eq(language))
            .filter(dsl::norm.eq(word))
            .load::<Word>(&mut self.conn()?)
            .map_err(|e| {
                DatabaseError::new(
                    format!(
                        "Failed to retrieve word {} from language {}: {:?}",
                        word, language, e
                    ),
                    "Failed to retrieve languages",
                )
            })
    }

    pub fn find_word(
        &self,
        language: uuid::Uuid,
        query: &str,
    ) -> Result<Vec<Word>, DatabaseError> {
        use self::schema::words::dsl;
        dsl::words
            .filter(dsl::language.eq(language))
            .filter(dsl::norm.ilike(format!("%{}%", query)))
            .load::<Word>(&mut self.conn()?)
            .map_err(|e| {
                DatabaseError::new(
                    format!(
                        "Failed to retrieve words from language {} with query {}: {:?}",
                        language,
                        query, e
                    ),
                    "Failed to retrieve languages",
                )
            })
    }
}
