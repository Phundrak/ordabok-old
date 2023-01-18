use juniper::FieldResult;
use uuid::Uuid;

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

    #[graphql(
        name = "findLanguage",
        description = "Find languages by username containing query",
        arguments(query(description = "String to find in language name"))
    )]
    fn find_language(
        context: &Context,
        query: String,
    ) -> FieldResult<Vec<Language>> {
        context.db.find_language(query.as_str()).map_err(Into::into)
    }

    #[graphql(
        name = "allUsers",
        description = "Fetch all users from database",
        arguments(admin_key(
            name = "adminKey",
            description = "Administrator key. Without it, the query cannot be executed"
        ))
    )]
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
        name = "findUser",
        description = "Find users by username containing query",
        arguments(query(description = "String to find in usernames"))
    )]
    fn find_user(context: &Context, query: String) -> FieldResult<Vec<User>> {
        context.db.find_user(query.as_str()).map_err(Into::into)
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
    ) -> FieldResult<Option<Language>> {
        context
            .db
            .language(name.as_str(), owner.as_str())
            .map_err(Into::into)
    }

    #[graphql(
        description = "Retrieve a specific user from its id",
        arguments(id(description = "Appwrite ID of a user"))
    )]
    fn user(context: &Context, id: String) -> FieldResult<Option<User>> {
        context.db.user(id.as_str()).map_err(Into::into)
    }

    #[graphql(
        description = "Retrieve a specific word from its id",
        arguments(id(description = "Unique identifier of a word"))
    )]
    fn word(context: &Context, id: String) -> FieldResult<Option<Word>> {
        match Uuid::from_str(&id) {
            Ok(uuid) => context.db.word_id(uuid).map_err(Into::into),
            Err(e) => Err(DatabaseError::new(
                format!("Failed to convert {id} to a UUID: {e:?}"),
                "Conversion Error",
            )
            .into()),
        }
    }

    #[graphql(
        name = "findWord",
        description = "Retrieve a word from a specific language",
        arguments(
            language(
                description = "UUID of the language to look the word for in"
            ),
            query(description = "String to find in the word")
        )
    )]
    fn find_word(
        context: &Context,
        language: String,
        query: String,
    ) -> FieldResult<Vec<Word>> {
        match Uuid::from_str(&language) {
            Ok(uuid) => context
                .db
                .find_word(uuid, query.as_str())
                .map_err(Into::into),
            Err(e) => Err(DatabaseError::new(
                format!("Failed to convert {language} to a UUID: {e:?}"),
                "Conversion Error",
            )
            .into()),
        }
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
        match Uuid::from_str(&language) {
            Ok(uuid) => {
                context.db.words(uuid, word.as_str()).map_err(Into::into)
            }
            Err(e) => Err(DatabaseError::new(
                format!("Failed to convert {language} to a UUID: {e:?}"),
                "Conversion Error",
            )
            .into()),
        }
    }
}
