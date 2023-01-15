use juniper::FieldResult;

use super::Context;
use crate::db::{
    models::{languages::Language, users::User, words::Word},
    DatabaseError,
};

use std::str::FromStr;

use std::convert::Into;

#[derive(Debug)]
pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    #[graphql(
        name = "allLanguages",
        description = "Retrieve all languages defined in the database"
    )]
    fn all_languages(context: &Context) -> FieldResult<Vec<Language>> {
        context.db.all_languages().map_err(Into::into)
    }

    fn all_users(
        context: &Context,
        admin_key: String,
    ) -> FieldResult<Vec<User>> {
        if admin_key == context.other_vars.admin_key {
            context.db.all_users().map_err(Into::into)
        } else {
            Err(DatabaseError::new("Invalid admin key", "Invalid admin key")
                .into())
        }
    }

    #[graphql(
        description = "Retrieve a specific language from its name and its owner's id",
        arguments(
            name(description = "Name of the language"),
            owner(description = "ID of the owner of the language")
        )
    )]
    fn language(
        context: &Context,
        name: String,
        owner: String,
    ) -> Option<Language> {
        context.db.language(name.as_str(), owner.as_str())
    }

    #[graphql(
        description = "Retrieve a specific user from its id",
        arguments(id(description = "Appwrite ID of a user"))
    )]
    fn user(context: &Context, id: String) -> Option<User> {
        context.db.user(id.as_str())
    }

    #[graphql(
        description = "Retrieve a specific word from its id",
        arguments(id(description = "Unique identifier of a word"))
    )]
    fn word(context: &Context, id: String) -> Option<Word> {
        context.db.word_id(id.as_str())
    }

    #[graphql(
        description = "Retrieve all words with a set normal form from a set language",
        arguments(
            owner(
                description = "ID of the owner of the language to search a word in"
            ),
            language(description = "Name of the language to search a word in"),
            word(description = "Word to search")
        )
    )]
    fn words(
        context: &Context,
        language: String,
        word: String,
    ) -> FieldResult<Vec<Word>> {
        match uuid::Uuid::from_str(&language) {
            Ok(uuid) => Ok(context.db.words(uuid, word.as_str())),
            Err(e) => Err(DatabaseError::new(
                format!(
                    "Failed to convert {} to a proper UUID: {:?}",
                    language, e
                ),
                "Conversion Error",
            )
            .into()),
        }
    }
}
