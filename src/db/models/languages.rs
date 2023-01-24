use crate::{
    db::{Database, DatabaseError},
    graphql::Context,
};
use diesel::prelude::*;
use juniper::{FieldResult, GraphQLEnum};
use tracing::info;

use uuid::Uuid;

use super::super::schema;
use super::users::User;

use std::{convert::Into, fmt::Display};

use schema::{langandagents, langtranslatesto, languages, userfollowlanguage};

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

impl Default for Release {
    fn default() -> Self {
        Self::Public
    }
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

#[derive(Default, Debug, Clone, juniper::GraphQLInputObject)]
pub struct NewLanguage {
    name: String,
    native: Option<String>,
    release: Option<Release>,
    genre: Vec<DictGenre>,
    abstract_: Option<String>,
    description: Option<String>,
    rights: Option<String>,
    license: Option<String>,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = languages)]
struct NewLanguageInternal {
    name: String,
    native: Option<String>,
    release: Release,
    genre: Vec<DictGenre>,
    abstract_: Option<String>,
    description: Option<String>,
    rights: Option<String>,
    license: Option<String>,
    owner: String,
}

impl From<NewLanguage> for NewLanguageInternal {
    fn from(val: NewLanguage) -> Self {
        NewLanguageInternal {
            name: val.name,
            native: val.native,
            release: if let Some(release) = val.release {
                release
            } else {
                Release::default()
            },
            genre: val.genre,
            abstract_: val.abstract_,
            description: val.description,
            rights: val.rights,
            license: val.license,
            owner: String::new(),
        }
    }
}

impl NewLanguage {
    pub fn insert(
        &self,
        db: &Database,
        owner: &str,
    ) -> Result<Language, DatabaseError> {
        use languages::dsl;
        let conn = &mut db.conn()?;
        match diesel::insert_into(dsl::languages)
            .values(NewLanguageInternal {
                owner: owner.to_string(),
                ..self.clone().into()
            })
            .execute(conn)
        {
            Ok(_) => dsl::languages
                .filter(dsl::name.eq(self.name.clone()))
                .filter(dsl::owner.eq(owner))
                .first::<Language>(conn)
                .map_err(|e| {
                    DatabaseError::new(
                        format!(
                            "Failed to find language {} by user {owner}: {e:?}",
                            self.name
                        ),
                        "Database Error",
                    )
                }),
            Err(e) => Err(DatabaseError::new(
                format!(
                    "Failed to insert language {} by user {owner}: {e:?}",
                    self.name
                ),
                "Database Error",
            )),
        }
    }
}

#[derive(Queryable, Insertable, Debug, Clone)]
pub struct Language {
    id: Uuid,
    name: String,
    native: Option<String>,
    release: Release,
    genre: Vec<Option<DictGenre>>,
    abstract_: Option<String>,
    created: chrono::NaiveDateTime,
    description: Option<String>,
    rights: Option<String>,
    license: Option<String>,
    owner: String,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{} ({})", self.owner, self.name, self.id)
    }
}

impl Language {
    pub fn is_owned_by(&self, owner: &str) -> bool {
        self.owner == owner
    }

    pub fn find(
        db: &Database,
        language: Uuid,
    ) -> Result<Language, DatabaseError> {
        use languages::dsl;
        dsl::languages.find(language).first::<Language>(&mut db.conn()?).map_err(|e| match e {
            diesel::NotFound => DatabaseError::new(
                format!("Language {language} not found"),
                "Not Found"
            ),
            e => DatabaseError::new(
                format!("Error fetching language {language} from database: {e:?}"),
                "Database Error"
            )
        })
    }

    pub fn delete(
        context: &Context,
        language_id: Uuid,
    ) -> Result<(), DatabaseError> {
        use languages::dsl;
        let conn = &mut context.db.conn()?;
        match dsl::languages.find(language_id)
                            .first::<Language>(conn)
        {
            Ok(language) if context.user_auth == Some(language.owner.clone()) => {
                match diesel::delete(dsl::languages.find(language_id))
                    .execute(conn) {
                        Ok(_) => Ok(()),
                        Err(e) => Err(DatabaseError::new(
                            format!("Failed to delete language {language_id}: {e:?}"),
                            "Database Error"
                        ))
                    }
            },
            Ok(language) => {
                Err(DatabaseError::new(
                    format!(
                        "User {} not allowed to delete other user's language {language_id}",
                        language.owner),
                    "Unauthorized"
                ))
            },
            Err(e) => {
                Err(DatabaseError::new(
                    format!("Failed to delete language {language_id}: {e:?}"),
                    "Database Error"
                ))
            }
        }
    }

    fn relationship(
        &self,
        db: &Database,
        relationship: AgentLanguageRelation,
    ) -> Result<Vec<User>, DatabaseError> {
        use schema::langandagents::dsl;
        let conn = &mut db.conn()?;
        Ok(dsl::langandagents
            .filter(dsl::language.eq(self.id))
            .filter(dsl::relationship.eq(relationship))
            .load::<LangAndAgent>(conn)
            .map_err(|e| {
                DatabaseError::new(
                    format!("Failed to retrieve language relationship: {e:?}"),
                    "Database reading error",
                )
            })?
            .iter()
            .map(|v| {
                use schema::users::dsl;
                dsl::users.find(v.agent.clone()).first::<User>(conn)
            })
            .filter_map(|author| match author {
                Ok(val) => Some(val),
                Err(e) => {
                    info!("Failed ot retrieve author from database: {:?}", e);
                    None
                }
            })
            .collect::<Vec<User>>())
    }
}

#[juniper::graphql_object(Context = Context)]
impl Language {
    #[graphql(description = "Unique identifier of the language")]
    fn id(&self) -> String {
        self.id.to_string()
    }

    #[graphql(
        description = "Name in the main target language (often English) of the described language"
    )]
    fn name(&self) -> String {
        self.name.clone()
    }

    #[graphql(description = "Native name of the language")]
    fn native(&self) -> Option<String> {
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
    fn target_language(&self, context: &Context) -> FieldResult<Vec<Language>> {
        use schema::langtranslatesto::dsl;
        match &mut context.db.conn() {
            Ok(conn) => Ok(dsl::langtranslatesto
                .filter(dsl::langfrom.eq(self.id))
                .load::<LangTranslatesTo>(conn)
                .map_err(|e| {
                    DatabaseError::new(
                        format!(
                            "Failed to retrieve language translations: {e:?}"
                        ),
                        "Database reading failure",
                    )
                })?
                .into_iter()
                .flat_map(|l| {
                    use schema::languages::dsl;
                    dsl::languages.find(l.langto).first::<Language>(conn)
                })
                .collect::<Vec<Language>>()),
            Err(e) => Err(DatabaseError::new(
                format!("Failed to connect to the database: {e:?}"),
                "Database connection failure",
            )
            .into()),
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
    fn owner(&self, context: &Context) -> FieldResult<User> {
        use schema::users::dsl;
        match &mut context.db.conn() {
            Ok(conn) => Ok(dsl::users
                .find(self.owner.clone())
                .first::<User>(conn)
                .map_err(|e| {
                    DatabaseError::new(
                        format!(
                            "Failed to retrieve owner {} of language {}: {e:?}",
                            self.owner, self.name
                        ),
                        "Database error",
                    )
                })?),
            Err(e) => Err(DatabaseError::new(
                format!("Failed to connect to the database: {e:?}"),
                "Database connection failure",
            )
            .into()),
        }
    }

    #[graphql(
        description = "People who participate in the elaboration of the language's dictionary"
    )]
    fn authors(&self, context: &Context) -> FieldResult<Vec<User>> {
        self.relationship(&context.db, AgentLanguageRelation::Author)
            .map_err(Into::into)
    }

    #[graphql(
        description = "People who can and do redistribute the language's dictionary"
    )]
    fn publishers(&self, context: &Context) -> FieldResult<Vec<User>> {
        self.relationship(&context.db, AgentLanguageRelation::Publisher)
            .map_err(Into::into)
    }

    #[graphql(description = "People following the language")]
    fn followers(&self, context: &Context) -> FieldResult<Vec<User>> {
        use schema::userfollowlanguage::dsl;
        match &mut context.db.conn() {
            Ok(conn) => {
                Ok(dsl::userfollowlanguage
                   .filter(dsl::lang.eq(self.id))
                   .load::<UserFollowLanguage>(conn)
                   .map_err(|e| {
                       DatabaseError::new(format!("Failed to retrieve language followers for language {}: {e:?}", self.id),
                       "Database error")
                   })?
                   .into_iter()
                   .filter_map(|follow| {
                       use schema::users::dsl;
                       match dsl::users
                           .find(follow.userid.clone())
                           .first::<User>(conn) {
                               Ok(user) => Some(user),
                               Err(e) => {
                                   info!("Failed to retrieve user {} from database: {e:?}", follow.userid);
                                   None
                               }
                           }
                   })
                   .collect::<Vec<User>>()
                )
            }
            Err(e) => Err(DatabaseError::new(
                format!("Failed to connect to the database: {e:?}"),
                "Database connection failure",
            )
            .into()),
        }
    }
}

#[derive(Queryable, Insertable, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = langandagents)]
pub struct LangAndAgent {
    id: i32,
    agent: String,
    language: Uuid,
    relationship: AgentLanguageRelation,
}

#[derive(Queryable, Insertable, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = langtranslatesto)]
pub struct LangTranslatesTo {
    id: i32,
    langfrom: Uuid,
    langto: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = userfollowlanguage)]
pub struct UserFollowLanguageInsert {
    pub lang: Uuid,
    pub userid: String,
}

#[derive(Queryable, Insertable, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = userfollowlanguage)]
pub struct UserFollowLanguage {
    pub id: i32,
    pub lang: Uuid,
    pub userid: String,
}

impl UserFollowLanguage {
    pub fn user_follow_language(
        context: &Context,
        userid: &str,
        lang: Uuid,
    ) -> Result<Language, DatabaseError> {
        let conn = &mut context.db.conn()?;
        match languages::dsl::languages.find(lang).first::<Language>(conn) {
            Err(diesel::NotFound) => Err(DatabaseError::new(
                format!("Cannot follow non-existing language {lang}"),
                "Invalid Language",
            )),
            Err(e) => Err(DatabaseError::new(
                format!(
                    "Could not retrieve language {lang} from database: {e:?}"
                ),
                "Database error",
            )),
            Ok(language) => {
                use userfollowlanguage::dsl;
                match diesel::insert_into(dsl::userfollowlanguage)
                    .values(UserFollowLanguageInsert { lang, userid: userid.to_string() })
                    .execute(conn) {
                        Ok(_) => Ok(language),
                        Err(e) => Err(DatabaseError::new(
                            format!("Failed to follow language {lang} as user {userid}: {e:?}"),
                            "Database Error"
                        ))
                    }
            }
        }
    }

    pub fn user_unfollow_language(
        context: &Context,
        userid: &str,
        lang: Uuid,
    ) -> Result<Language, DatabaseError> {
        use userfollowlanguage::dsl;
        let conn = &mut context.db.conn()?;
        match dsl::userfollowlanguage
            .filter(dsl::userid.eq(userid.to_string()))
            .filter(dsl::lang.eq(lang))
            .first::<UserFollowLanguage>(conn) {
                Ok(relationship) => {
                    match diesel::delete(dsl::userfollowlanguage.find(relationship.id))
                        .execute(conn) {
                            Ok(_) => Language::find(&context.db, lang),
                            Err(e) => Err(DatabaseError::new(
                                format!("Failed to make user {userid} unfollow language {lang}: {e:?}"),
                                "Database Error"
                            ))
                        }
                },
                Err(diesel::NotFound) => {
                    Err(DatabaseError::new(
                        format!("User {userid} does not follow language {lang}"),
                        "Invalid",
                    ))
                }
                Err(e) => Err(DatabaseError::new(
                    format!("Failed to retrieve relationship between user {userid} and language {lang} from database: {e:?}"),
                    "Database Error",
                ))
            }
    }
}
