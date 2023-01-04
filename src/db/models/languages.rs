use super::super::schema::{langandagents, languages};
use diesel::prelude::*;

#[derive(diesel_derive_enum::DbEnum, Debug, Clone, PartialEq, Eq)]
#[DieselTypePath = "crate::db::schema::sql_types::Release"]
pub enum Release {
    Public,
    NonCommercial,
    Research,
    Private,
}

#[derive(diesel_derive_enum::DbEnum, Debug, Clone, PartialEq, Eq)]
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

#[derive(diesel_derive_enum::DbEnum, Debug, Clone, PartialEq, Eq)]
#[DieselTypePath = "crate::db::schema::sql_types::Agentlanguagerelation"]
pub enum AgentLanguageRelation {
    Publisher,
    Author,
}

#[derive(Queryable, Insertable, Debug, Clone, PartialEq, Eq)]
pub struct Language {
    release: Release,
    created: chrono::NaiveDateTime,
    name: String,
    owner: String,
    targetlanguage: Vec<String>,
    genre: Vec<DictGenre>,
    native: Option<String>,
    abstract_: Option<String>,
    description: Option<String>,
    rights: Option<String>,
    license: Option<String>,
}

#[derive(Queryable, Insertable, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = langandagents)]
pub struct LangAndAgent {
    id: i32,
    agent: String,
    language: String,
}
