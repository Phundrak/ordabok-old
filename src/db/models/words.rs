use super::super::schema;
use crate::db::Database;
use diesel::prelude::*;
use juniper::GraphQLEnum;
use schema::{wordrelation, words};
use tracing::info;

use super::languages::Language;

#[derive(diesel_derive_enum::DbEnum, Debug, Clone, PartialEq, Eq)]
#[DieselTypePath = "crate::db::schema::sql_types::Wordrelationship"]
pub enum WordRelationship {
    Definition,
    Related,
}

#[derive(diesel_derive_enum::DbEnum, Debug, Clone, PartialEq, Eq, GraphQLEnum)]
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

impl Word {
    fn relationship(&self, context: &Database, relationship: WordRelationship) -> Vec<Word> {
        use schema::wordrelation::dsl;
        match &mut context.conn() {
            Ok(conn) => {
                dsl::wordrelation
                    .filter(dsl::wordsource.eq(self.norm.clone()))
                    .filter(dsl::relationship.eq(relationship))
                    .load::<WordRelation>(conn)
                    .unwrap()
                    .into_iter()
                    .flat_map(|w| {
                        use schema::words::dsl;
                        dsl::words.find(w.wordtarget).first::<Word>(conn)
                    })
                    .collect::<Vec<Word>>()
            },
            Err(e) => {
                info!("Could not connect to database: {:?}", e);
                Vec::new()
            }
        }
    }

}

#[juniper::graphql_object(Context = Database)]
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
    fn lemma(&self, context: &Database) -> Option<Word> {
        use schema::words::dsl;
        match self.lemma.clone() {
            Some(lemma) => match &mut context.conn() {
                Ok(conn) => match dsl::words.find(lemma.clone()).first::<Word>(conn) {
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
    fn language(&self, context: &Database) -> Language {
        use schema::languages::dsl;
        match &mut context.conn() {
            Ok(conn) => {
                match dsl::languages
                    .find(self.language.clone())
                    .first::<Language>(conn)
                {
                    Ok(lang) => lang,
                    Err(e) => {
                        panic!("Failed to retrieve language {} of word {} from database: {:?}",
                               self.language, self.norm, e
                        )
                    }
                }
            }
            Err(e) => panic!("Failed to connect to database: {:?}", e),
        }
    }

    #[graphql(name = "partOfSpeech", description = "Part of speech the word belongs to")]
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


    #[graphql(description = "Morphology of the word, can be in Markdown format")]
    fn morphology(&self) -> Option<String> {
        self.morphology.clone()
    }

    #[graphql(name = "related", description = "Words related to the current word")]
    fn related_words(&self, context: &Database) -> Vec<Word> {
        self.relationship(context, WordRelationship::Related)
    }

    #[graphql(name = "definitions", description = "Words that define the current word")]
    fn definitions(&self, context: &Database) -> Vec<Word> {
        self.relationship(context, WordRelationship::Definition)
    }
}

#[derive(Queryable, Insertable, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = wordrelation)]
pub struct WordRelation {
    id: i32,
    wordsource: String,
    wordtarget: String,
    relationship: WordRelationship,
}
