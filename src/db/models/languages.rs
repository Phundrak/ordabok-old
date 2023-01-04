use super::super::schema::{langandagents, languages};
use diesel::prelude::*;
use juniper::GraphQLEnum;

#[derive(diesel_derive_enum::DbEnum, Debug, Clone, PartialEq, Eq, GraphQLEnum)]
#[DieselTypePath = "crate::db::schema::sql_types::Release"]
pub enum Release {
    Public,
    #[graphql(name="NON_COMMERCIAL")]
    NonCommercial,
    Research,
    Private,
}

#[derive(diesel_derive_enum::DbEnum, Debug, Clone, PartialEq, Eq, GraphQLEnum)]
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

#[derive(diesel_derive_enum::DbEnum, Debug, Clone, PartialEq, Eq, GraphQLEnum)]
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

#[juniper::graphql_object]
impl Language {
    #[graphql(name = "release")]
    fn release(&self) -> Release {
        self.release.clone()
    }

    #[graphql(name = "created")]
    fn created(&self) -> String {
        self.created.to_string()
    }

    #[graphql(name = "name")]
    fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Queryable, Insertable, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = langandagents)]
pub struct LangAndAgent {
    id: i32,
    agent: String,
    language: String,
}
