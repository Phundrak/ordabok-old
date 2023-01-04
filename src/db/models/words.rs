use super::super::schema::{wordrelation, words};
use diesel::prelude::*;

#[derive(diesel_derive_enum::DbEnum, Debug, Clone, PartialEq, Eq)]
#[DieselTypePath = "crate::db::schema::sql_types::Wordrelationship"]
pub enum WordRelationship {
    Definition,
    Related,
}

#[derive(diesel_derive_enum::DbEnum, Debug, Clone, PartialEq, Eq)]
#[DieselTypePath = "crate::db::schema::sql_types::Partofspeech"]
pub enum PartOfSpeech {
    Adjective,
    Adposition,
    Adverb,
    Auxilliary,
    CoordConj,
    Determiner,
    Interjection,
    Noun,
    Numeral,
    Particle,
    Pronoun,
    ProperNoun,
    Punctuation,
    SubjConj,
    Symbol,
    Verb,
    Other,
}

#[derive(Queryable, Insertable, Debug, Clone, PartialEq, Eq)]
pub struct Word {
    norm: String,
    native: Option<String>,
    lemma: Option<String>,
    language: String,
    partofspeech: PartOfSpeech,
    audio: Option<String>,
    video: Option<String>,
    image: Option<String>,
    description: Option<String>,
    etymology: Option<String>,
    lusage: Option<String>,
    morphology: Option<String>,
}

#[derive(Queryable, Insertable, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = wordrelation)]
pub struct WordRelation {
    id: i32,
    wordsource: String,
    wordtarget: String,
    relationship: WordRelationship,
}
