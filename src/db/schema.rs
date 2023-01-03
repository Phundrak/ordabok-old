// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "agentlanguagerelation"))]
    pub struct Agentlanguagerelation;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "dictgenre"))]
    pub struct Dictgenre;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "partofspeech"))]
    pub struct Partofspeech;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "release"))]
    pub struct Release;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "wordrelationship"))]
    pub struct Wordrelationship;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Agentlanguagerelation;

    langandagents (id) {
        id -> Int4,
        agent -> Varchar,
        language -> Varchar,
        relationship -> Agentlanguagerelation,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Release;
    use super::sql_types::Dictgenre;

    languages (name) {
        name -> Varchar,
        native -> Nullable<Varchar>,
        release -> Release,
        targetlanguage -> Array<Nullable<Text>>,
        genre -> Array<Nullable<Dictgenre>>,
        #[sql_name = "abstract"]
        abstract_ -> Nullable<Text>,
        created -> Timestamp,
        description -> Nullable<Text>,
        rights -> Nullable<Text>,
        license -> Nullable<Text>,
        owner -> Varchar,
    }
}

diesel::table! {
    userfollows (id) {
        id -> Int4,
        follower -> Varchar,
        following -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Varchar,
        username -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Wordrelationship;

    wordrelation (id) {
        id -> Int4,
        wordsource -> Varchar,
        wordtarget -> Varchar,
        relationship -> Wordrelationship,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Partofspeech;

    words (norm) {
        norm -> Varchar,
        native -> Nullable<Varchar>,
        lemma -> Nullable<Varchar>,
        language -> Varchar,
        partofspeech -> Partofspeech,
        audio -> Nullable<Varchar>,
        video -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        description -> Nullable<Text>,
        etymology -> Nullable<Text>,
        lusage -> Nullable<Text>,
        morphology -> Nullable<Text>,
    }
}

diesel::joinable!(langandagents -> languages (language));
diesel::joinable!(langandagents -> users (agent));
diesel::joinable!(languages -> users (owner));
diesel::joinable!(words -> languages (language));

diesel::allow_tables_to_appear_in_same_query!(
    langandagents,
    languages,
    userfollows,
    users,
    wordrelation,
    words,
);
