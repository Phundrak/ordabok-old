use juniper::FieldResult;

use crate::db::{models::users::User, DatabaseError};

use super::Context;

pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    fn api_version(context: &Context) -> String {
        if context.user_auth {
            "0.1 (authentified)"
        } else {
            "0.1 (not authentified)"
        }
        .into()
    }

    pub fn db_only_new_user(
        context: &Context,
        username: String,
        id: String,
        admin_key: String,
    ) -> FieldResult<User> {
        if admin_key == context.other_vars.admin_key {
            context
                .db
                .insert_user(username, id)
                .map_err(std::convert::Into::into)
        } else {
            Err(DatabaseError::new("Invalid admin key", "Invalid admin key")
                .into())
        }
    }

    pub fn db_only_delete_user(
        context: &Context,
        id: String,
        admin_key: String,
    ) -> FieldResult<String> {
        if admin_key == context.other_vars.admin_key {
            match context.db.delete_user(&id) {
                Ok(_) => Ok("done".into()),
                Err(e) => Err(e.into()),
            }
        } else {
            Err(DatabaseError::new("Invalid admin key", "Invalid admin key")
                .into())
        }
    }
}
