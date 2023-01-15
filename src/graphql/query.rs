use super::Context;
use crate::db::models::{languages::Language, users::User, words::Word};

use std::str::FromStr;

#[derive(Debug)]
pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    #[graphql(
        name = "allLanguages",
        description = "Retrieve all languages defined in the database"
    )]
    fn all_languages(context: &Context) -> Vec<Language> {
        context.db.all_languages().unwrap()
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
    fn words(context: &Context, language: String, word: String) -> Vec<Word> {
        context
            .db
            .words(uuid::Uuid::from_str(&language).unwrap(), word.as_str())
    }
}
