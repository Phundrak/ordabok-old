use super::super::schema::{userfollows, users};
use diesel::prelude::*;
use juniper::FieldResult;
use tracing::debug;

use crate::{db::DatabaseError, graphql::Context};

#[derive(Queryable, Insertable, Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: String,
    pub username: String,
}

#[juniper::graphql_object(Context = Context)]
impl User {
    #[graphql(description = "Appwrite ID of the user")]
    pub fn id(&self) -> String {
        self.id.clone()
    }

    #[graphql(description = "The user's apparent name")]
    pub fn username(&self) -> String {
        self.username.clone()
    }

    #[graphql(description = "Who the user follows")]
    pub fn following(&self, context: &Context) -> FieldResult<Vec<User>> {
        use super::super::schema::{userfollows, users};
        let conn = &mut context.db.conn().map_err(|e| {
            DatabaseError::new(
                format!("Failed to connect to database: {e:?}"),
                "Database connection error",
            )
        })?;
        Ok(userfollows::dsl::userfollows
            .filter(userfollows::dsl::follower.eq(self.id.clone()))
            .load::<UserFollow>(conn)
           .map_err(|e| {
                DatabaseError::new(
                    format!(
                        "Failed to retrieve user follows from database: {e:?}"
                    ),
                    "Database reading error",
                )
            })?
           .iter()
           .filter_map(|f| {
               match users::dsl::users
                .find(f.following.clone())
                .first::<User>(conn) {
                    Ok(val) => Some(val),
                    Err(e) => {
                        let err = DatabaseError::new(
                            format!("Failed to retrieve user {} from database: {e:?}",
                                    f.following.clone()),
                            "Database reading error");
                        debug!("{}", err);
                        None
                    }
                }

           })
           .collect::<Vec<User>>())
    }
}

#[derive(Queryable, Insertable, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = userfollows)]
pub struct UserFollow {
    pub id: i32,
    pub follower: String,
    pub following: String,
}
