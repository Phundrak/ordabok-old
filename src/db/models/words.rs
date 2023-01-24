use super::super::schema;
use crate::{
    db::{Database, DatabaseError},
    graphql::Context,
};
use diesel::prelude::*;
use juniper::{FieldResult, GraphQLEnum};
use schema::{wordlearning, wordrelation, words};
use tracing::info;
use uuid::Uuid;

use std::{convert::Into, str::FromStr};

use super::languages::Language;

#[derive(diesel_derive_enum::DbEnum, Debug, Clone, PartialEq, Eq)]
#[DieselTypePath = "crate::db::schema::sql_types::Wordrelationship"]
pub enum WordRelationship {
    Definition,
    Related,
}

#[derive(
    diesel_derive_enum::DbEnum,
    Debug,
    Clone,
    PartialEq,
    Eq,
    juniper::GraphQLEnum,
)]
#[DieselTypePath = "crate::db::schema::sql_types::Wordlearningstatus"]
pub enum WordLearningStatus {
    Learning,
    Learned,
}

#[derive(
    diesel_derive_enum::DbEnum, Debug, Clone, PartialEq, Eq, GraphQLEnum,
)]
#[DieselTypePath = "crate::db::schema::sql_types::Partofspeech"]
pub enum PartOfSpeech {
    Adjective,
    Adposition,
    Adverb,
    Auxilliary,
    #[graphql(name = "COORDINATING_CONJUNCTION")]
    CoordConj,
    Determiner,
    Interjection,
    Noun,
    Numeral,
    Particle,
    Pronoun,
    #[graphql(name = "PROPER_NOUN")]
    ProperNoun,
    Punctuation,
    #[graphql(name = "SUBORDINATING_CONJUNCTION")]
    SubjConj,
    Symbol,
    Verb,
    Other,
}

impl Default for PartOfSpeech {
    fn default() -> Self {
        Self::Noun
    }
}

#[derive(Debug, Clone, juniper::GraphQLInputObject)]
pub struct NewWord {
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

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = words)]
struct NewWordInternal {
    norm: String,
    native: Option<String>,
    lemma: Option<Uuid>,
    language: Uuid,
    partofspeech: PartOfSpeech,
    audio: Option<String>,
    video: Option<String>,
    image: Option<String>,
    description: Option<String>,
    etymology: Option<String>,
    lusage: Option<String>,
    morphology: Option<String>,
}

impl TryFrom<NewWord> for NewWordInternal {
    type Error = uuid::Error;

    fn try_from(value: NewWord) -> Result<Self, Self::Error> {
        let language = Uuid::from_str(&value.language)?;
        let lemma = if let Some(original_lemma) = value.lemma {
            Some(Uuid::from_str(&original_lemma)?)
        } else {
            None
        };
        Ok(Self {
            norm: value.norm,
            native: value.native,
            lemma,
            language,
            partofspeech: value.partofspeech,
            audio: value.audio,
            video: value.video,
            image: value.image,
            description: value.description,
            etymology: value.etymology,
            lusage: value.lusage,
            morphology: value.morphology,
        })
    }
}

#[derive(Queryable, Insertable, Debug, Clone, PartialEq, Eq)]
pub struct Word {
    id: Uuid,
    norm: String,
    native: Option<String>,
    lemma: Option<Uuid>,
    language: uuid::Uuid,
    partofspeech: PartOfSpeech,
    audio: Option<String>,
    video: Option<String>,
    image: Option<String>,
    description: Option<String>,
    etymology: Option<String>,
    lusage: Option<String>,
    morphology: Option<String>,
}

impl Word {
    fn relationship(
        &self,
        db: &Database,
        relationship: WordRelationship,
    ) -> Result<Vec<Word>, DatabaseError> {
        use schema::wordrelation::dsl;
        match &mut db.conn() {
            Ok(conn) => Ok(dsl::wordrelation
                .filter(dsl::wordsource.eq(self.id))
                .filter(dsl::relationship.eq(relationship))
                .load::<WordRelation>(conn)
                .map_err(|e| {
                    DatabaseError::new(
                        format!("Failed to retrieve word relations: {e:?}"),
                        "Database reading failed",
                    )
                })?
                .into_iter()
                .flat_map(|word| {
                    use schema::words::dsl;
                    dsl::words.find(word.wordtarget).first::<Word>(conn)
                })
                .collect::<Vec<Word>>()),
            Err(e) => Err(DatabaseError::new(
                format!("Failed to connect to the database: {e:?}"),
                "Database connection error",
            )),
        }
    }
}

#[juniper::graphql_object(Context = Context)]
impl Word {
    #[graphql(description = "Normal form of the word")]
    fn norm(&self) -> String {
        self.norm.clone()
    }

    #[graphql(description = "Native representation of the word")]
    fn native(&self) -> Option<String> {
        self.native.clone()
    }

    #[graphql(description = "Base form of the current word")]
    fn lemma(&self, context: &Context) -> Option<Word> {
        use schema::words::dsl;
        match self.lemma {
            Some(lemma) => match &mut context.db.conn() {
                Ok(conn) => match dsl::words.find(lemma).first::<Word>(conn) {
                    Ok(word) => Some(word),
                    Err(e) => {
                        info!(
                            "Failed to retrieve lemma {} of word {}: {:?}",
                            lemma, self.norm, e
                        );
                        None
                    }
                },
                Err(e) => {
                    info!("Could not connect to the database: {:?}", e);
                    None
                }
            },
            None => None,
        }
    }

    #[graphql(description = "Language to which the word belongs")]
    fn language(&self, context: &Context) -> FieldResult<Language> {
        use schema::languages::dsl;
        use std::convert::Into;
        dsl::languages
            .find(self.language)
            .first::<Language>(&mut context.db.conn()?)
            .map_err(|e| DatabaseError::new(
                format!(
                    "Failed to retrieve language {} of word {} from database: {e:?}",
                    self.language, self.norm
                ),
                "Database Error"
            ).into())
    }

    #[graphql(
        name = "partOfSpeech",
        description = "Part of speech the word belongs to"
    )]
    fn part_of_speech(&self) -> PartOfSpeech {
        self.partofspeech.clone()
    }

    #[graphql(description = "Link to an audio file related to the word")]
    fn audio(&self) -> Option<String> {
        self.audio.clone()
    }

    #[graphql(description = "Link to an video file related to the word")]
    fn video(&self) -> Option<String> {
        self.video.clone()
    }

    #[graphql(description = "Link to an image file related to the word")]
    fn image(&self) -> Option<String> {
        self.image.clone()
    }

    #[graphql(description = "Etymology of the word, can be in Markdown format")]
    fn etymology(&self) -> Option<String> {
        self.etymology.clone()
    }

    #[graphql(description = "Usage of the word, can be in Markdown format")]
    fn usage(&self) -> Option<String> {
        self.lusage.clone()
    }

    #[graphql(
        description = "Morphology of the word, can be in Markdown format"
    )]
    fn morphology(&self) -> Option<String> {
        self.morphology.clone()
    }

    #[graphql(
        name = "related",
        description = "Words related to the current word"
    )]
    fn related_words(&self, context: &Context) -> FieldResult<Vec<Word>> {
        self.relationship(&context.db, WordRelationship::Related)
            .map_err(Into::into)
    }

    #[graphql(
        name = "definitions",
        description = "Words that define the current word"
    )]
    fn definitions(&self, context: &Context) -> FieldResult<Vec<Word>> {
        self.relationship(&context.db, WordRelationship::Definition)
            .map_err(Into::into)
    }
}

#[derive(Queryable, Insertable, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = wordrelation)]
pub struct WordRelation {
    id: i32,
    wordsource: Uuid,
    wordtarget: Uuid,
    relationship: WordRelationship,
}

#[derive(Queryable, Insertable, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = wordlearning)]
pub struct WordLearning {
    pub id: i32,
    pub word: Uuid,
    pub userid: String,
    pub status: WordLearningStatus,
}
