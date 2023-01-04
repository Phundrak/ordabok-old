use crate::db::Database;
use diesel::prelude::*;
use juniper::GraphQLEnum;
use tracing::info;

use super::super::schema;
use super::users::User;

use schema::{langandagents, languages};

#[derive(
    diesel_derive_enum::DbEnum, Debug, Clone, PartialEq, Eq, GraphQLEnum,
)]
#[DieselTypePath = "crate::db::schema::sql_types::Release"]
pub enum Release {
    Public,
    #[graphql(name = "NON_COMMERCIAL")]
    NonCommercial,
    Research,
    Private,
}

#[derive(
    diesel_derive_enum::DbEnum, Debug, Clone, PartialEq, Eq, GraphQLEnum,
)]
#[DieselTypePath = "crate::db::schema::sql_types::Dictgenre"]
pub enum DictGenre {
    General,
    Learning,
    Etymology,
    Specialized,
    Historical,
    Orthography,
    Terminology,
}

#[derive(
    diesel_derive_enum::DbEnum, Debug, Clone, PartialEq, Eq, GraphQLEnum,
)]
#[DieselTypePath = "crate::db::schema::sql_types::Agentlanguagerelation"]
pub enum AgentLanguageRelation {
    Publisher,
    Author,
}

#[derive(Queryable, Insertable, Debug, Clone, PartialEq, Eq)]
pub struct Language {
    name: String,
    native: Option<String>,
    release: Release,
    targetlanguage: Vec<Option<String>>,
    genre: Vec<Option<DictGenre>>,
    abstract_: Option<String>,
    created: chrono::NaiveDateTime,
    description: Option<String>,
    rights: Option<String>,
    license: Option<String>,
    owner: String,
}

impl Language {
    fn relationship(
        &self,
        context: &Database,
        relationship: AgentLanguageRelation,
    ) -> Vec<User> {
        use schema::langandagents::dsl;
        match &mut context.conn() {
            Ok(conn) => dsl::langandagents
                .filter(dsl::language.eq(self.name.clone()))
                .filter(dsl::relationship.eq(relationship))
                .load::<LangAndAgent>(conn)
                .unwrap()
                .iter()
                .map(|v| {
                    use schema::users::dsl;
                    dsl::users.find(v.agent.clone()).first::<User>(conn)
                })
                .filter_map(|author| match author {
                    Ok(val) => Some(val),
                    Err(e) => {
                        info!(
                            "Failed ot retrieve author from database: {:?}",
                            e
                        );

                        None
                    }
                })
                .collect::<Vec<User>>(),
            Err(e) => {
                panic!("Could not connect to the database: {:?}", e);
            }
        }
    }
}

#[juniper::graphql_object(Context = Database)]
impl Language {
    #[graphql(
        description = "Name in the main target language (often English) of the described language"
    )]
    fn name(&self) -> String {
        self.name.clone()
    }

    #[graphql(description = "Native name of the language")]
    fn native() -> Option<String> {
        self.native.clone()
    }

    #[graphql(description = "How the dictionary is currently released")]
    fn release(&self) -> Release {
        self.release.clone()
    }

    #[graphql(
        name = "targetLanguage",
        description = "Languages in which the current language is translated"
    )]
    fn target_language(&self, context: &Database) -> Vec<Language> {
        use schema::languages::dsl;
        match &mut context.conn() {
            Ok(conn) => self
                .targetlanguage
                .clone()
                .into_iter()
                .flatten()
                .map(|l| dsl::languages.find(l).first::<Language>(conn))
                .filter_map(|l| match l {
                    Ok(language) => Some(language),
                    Err(e) => {
                        info!(
                            "Failed to retrieve language from database: {:?}",
                            e
                        );
                        None
                    }
                })
                .collect::<Vec<Language>>(),
            Err(e) => {
                info!("Failed to connect to the database: {:?}", e);
                Vec::new()
            }
        }
    }

    #[graphql(description = "What kind of dictionary this is")]
    fn genre(&self) -> Vec<DictGenre> {
        self.genre.clone().into_iter().flatten().collect()
    }

    #[graphql(
        name = "abstract",
        description = "Short description of the language"
    )]
    fn abstract_(&self) -> Option<String> {
        self.abstract_.clone()
    }

    #[graphql(
        description = "Time at which the language's dictionary was created"
    )]
    fn created(&self) -> String {
        self.created.to_string()
    }

    #[graphql(
        description = "Longer description of the language, its content can be formatted as Markdown"
    )]
    fn description(&self) -> Option<String> {
        self.description.clone()
    }

    #[graphql(
        description = "Copyrights held by various people over the language's dictionary and its content"
    )]
    fn rights(&self) -> Option<String> {
        self.rights.clone()
    }

    #[graphql(description = "License under which the dictionary is released")]
    fn license(&self) -> Option<String> {
        self.license.clone()
    }

    #[graphql(
        description = "User with administrative rights over the language"
    )]
    fn owner(&self, context: &Database) -> User {
        use schema::users::dsl;
        match &mut context.conn() {
            Ok(conn) => dsl::users
                .find(self.owner.clone())
                .first::<User>(conn)
                .unwrap_or_else(|e| {
                    panic!(
                        "Failed to retrieve owner {} of language {}: {:?}",
                        self.owner, self.name, e
                    )
                }),
            Err(e) => panic!("Failed to connect to the database: {:?}", e),
        }
    }

    #[graphql(
        description = "People who participate in the elaboration of the language's dictionary"
    )]
    fn authors(&self, context: &Database) -> Vec<User> {
        self.relationship(context, AgentLanguageRelation::Author)
    }

    #[graphql(
        description = "People who can and do redistribute the language's dictionary"
    )]
    fn publishers(&self, context: &Database) -> Vec<User> {
        self.relationship(context, AgentLanguageRelation::Publisher)
    }
}

#[derive(Queryable, Insertable, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = langandagents)]
pub struct LangAndAgent {
    id: i32,
    agent: String,
    language: String,
    relationship: AgentLanguageRelation,
}
