use std::str::FromStr;

use juniper::FieldResult;
use uuid::Uuid;

use crate::db::{
    models::{
        languages::{Language, NewLanguage, UserFollowLanguage},
        users::User,
        words::{NewWord, Word},
    },
    DatabaseError,
};

use super::Context;

pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    fn api_version(context: &Context) -> String {
        if context.user_auth.is_some() {
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

    pub fn user_follow_language(
        context: &Context,
        language: String,
    ) -> FieldResult<Language> {
        if let Some(userid) = &context.user_auth {
            match Uuid::from_str(&language) {
                Err(e) => Err(DatabaseError::new(
                    format!(
                        "Could not parse {language} as a valid UUID: {e:?}"
                    ),
                    "Bad Request",
                )
                .into()),
                Ok(lang) => UserFollowLanguage::user_follow_language(
                    context,
                    &userid.to_string(),
                    lang,
                )
                .map_err(Into::into),
            }
        } else {
            Err(DatabaseError::new(
                "User not authentificated, cannot proceed",
                "Unauthorized",
            )
            .into())
        }
    }

    pub fn user_unfollow_language(
        context: &Context,
        language: String,
    ) -> FieldResult<Language> {
        if let Some(userid) = &context.user_auth {
            match Uuid::from_str(&language) {
                Err(e) => Err(DatabaseError::new(
                    format!(
                        "Could not parse {language} as a valid UUID: {e:?}"
                    ),
                    "Bad Request",
                )
                .into()),
                Ok(lang) => UserFollowLanguage::user_unfollow_language(
                    context,
                    &userid.to_string(),
                    lang,
                )
                .map_err(Into::into),
            }
        } else {
            Err(DatabaseError::new(
                "User not authentificated, cannot proceed",
                "Unauthorized",
            )
            .into())
        }
    }

    pub fn new_language(
        context: &Context,
        language: NewLanguage,
    ) -> FieldResult<Language> {
        if let Some(owner) = &context.user_auth {
            language.insert(&context.db, owner).map_err(Into::into)
        } else {
            Err(DatabaseError::new(
                "User not authentificated, cannot create new language",
                "Unauthorized",
            )
            .into())
        }
    }

    pub fn delete_language(
        context: &Context,
        language: String,
    ) -> FieldResult<Option<Language>> {
        if context.user_auth.is_some() {
            match Uuid::from_str(&language) {
                Ok(uuid) => Language::delete(context, uuid)
                    .map(|_| None)
                    .map_err(Into::into),
                Err(e) => Err(DatabaseError::new(
                    format!(
                        "Could not parse {language} as a valid UUID: {e:?}"
                    ),
                    "Bad Request",
                )
                .into()),
            }
        } else {
            Err(DatabaseError::new(
                "User not authentificated, cannot create new language",
                "Unauthorized",
            )
            .into())
        }
    }

    pub fn new_word(context: &Context, word: NewWord) -> FieldResult<Word> {
        if let Some(user) = &context.user_auth {
            word.insert(context, user).map_err(Into::into)
        } else {
            Err(DatabaseError::new(
                "User not authentificated, cannot create new language",
                "Unauthorized",
            )
            .into())
        }
    }

    pub fn delete_word(
        context: &Context,
        word: String,
    ) -> FieldResult<Option<Word>> {
        if let Some(user) = &context.user_auth {
            match Uuid::from_str(&word) {
                Ok(id) => Word::delete(context, id, user)
                    .map(|_| None)
                    .map_err(Into::into),
                Err(e) => Err(DatabaseError::new(
                    format!("Could not parse {word} as a valid UUID: {e:?}"),
                    "Bad Request",
                )
                .into()),
            }
        } else {
            Err(DatabaseError::new(
                "User not authentificated, cannot create new language",
                "Unauthorized",
            )
            .into())
        }
    }
}
