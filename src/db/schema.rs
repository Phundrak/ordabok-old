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
    #[diesel(postgres_type(name = "wordlearningstatus"))]
    pub struct Wordlearningstatus;

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
        language -> Uuid,
        relationship -> Agentlanguagerelation,
    }
}

diesel::table! {
    langtranslatesto (id) {
        id -> Int4,
        langfrom -> Uuid,
        langto -> Uuid,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Release;
    use super::sql_types::Dictgenre;

    languages (id) {
        id -> Uuid,
        name -> Varchar,
        native -> Nullable<Varchar>,
        release -> Release,
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
    userfollowlanguage (id) {
        id -> Int4,
        lang -> Uuid,
        userid -> Varchar,
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
    use super::sql_types::Wordlearningstatus;

    wordlearning (id) {
        id -> Int4,
        word -> Uuid,
        userid -> Varchar,
        status -> Wordlearningstatus,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Wordrelationship;

    wordrelation (id) {
        id -> Int4,
        wordsource -> Uuid,
        wordtarget -> Uuid,
        relationship -> Wordrelationship,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Partofspeech;

    words (id) {
        id -> Uuid,
        norm -> Varchar,
        native -> Nullable<Varchar>,
        lemma -> Nullable<Uuid>,
        language -> Uuid,
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
diesel::joinable!(userfollowlanguage -> languages (lang));
diesel::joinable!(userfollowlanguage -> users (userid));
diesel::joinable!(wordlearning -> users (userid));
diesel::joinable!(wordlearning -> words (word));
diesel::joinable!(words -> languages (language));

diesel::allow_tables_to_appear_in_same_query!(
    langandagents,
    langtranslatesto,
    languages,
    userfollowlanguage,
    userfollows,
    users,
    wordlearning,
    wordrelation,
    words,
);
