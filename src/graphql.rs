use rocket::response::content::RawHtml;
use rocket::State;

use juniper::EmptySubscription;
use juniper_rocket::{GraphQLRequest, GraphQLResponse};

use crate::db::models::{languages::Language, users::User, words::Word};
use crate::db::Database;

use std::str::FromStr;

#[derive(Debug)]
pub struct Query;

#[juniper::graphql_object(Context = Database)]
impl Query {
    #[graphql(
        name = "allLanguages",
        description = "Retrieve all languages defined in the database"
    )]
    fn all_languages(context: &Database) -> Vec<Language> {
        context.all_languages().unwrap()
    }

    #[graphql(
        description = "Retrieve a specific language from its name and its owner's id",
        arguments(
            name(description = "Name of the language"),
            owner(description = "ID of the owner of the language")
        )
    )]
    fn language(
        context: &Database,
        name: String,
        owner: String,
    ) -> Option<Language> {
        context.language(name.as_str(), owner.as_str())
    }

    #[graphql(
        description = "Retrieve a specific user from its id",
        arguments(id(description = "Appwrite ID of a user"))
    )]
    fn user(context: &Database, id: String) -> Option<User> {
        context.user(id.as_str())
    }

    #[graphql(
        description = "Retrieve a specific word from its id",
        arguments(id(description = "Unique identifier of a word"))
    )]
    fn word(context: &Database, id: String) -> Option<Word> {
        context.word_id(id.as_str())
    }

    #[graphql(
        description = "Retrieve all words with a set normal form from a set language",
        arguments(
            owner(
                description = "ID of the owner of the language to search a word in"
            ),
            language(description = "Name of the language to search a word in"),
            word(description = "Word to search")
        )
    )]
    fn words(
        context: &Database,
        language: String,
        word: String,
    ) -> Vec<Word> {
        context.words(uuid::Uuid::from_str(&language).unwrap(), word.as_str())
    }
}

pub struct Mutation;

#[juniper::graphql_object(Context = Database)]
impl Mutation {
    fn api_version() -> String {
        "0.1".into()
    }
}

type Schema =
    juniper::RootNode<'static, Query, Mutation, EmptySubscription<Database>>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {}, EmptySubscription::default())
}

#[rocket::get("/")]
pub fn graphiql() -> RawHtml<String> {
    let graphql_endpoint_url = "/graphql";
    juniper_rocket::graphiql_source(graphql_endpoint_url, None)
}

#[rocket::get("/graphql?<request>")]
pub async fn get_graphql_handler(
    context: &State<Database>,
    request: GraphQLRequest,
    schema: &State<Schema>,
) -> GraphQLResponse {
    request.execute(schema, context).await
}

#[allow(clippy::needless_pass_by_value)]
#[rocket::post("/graphql", data = "<request>")]
pub fn post_graphql_handler(
    context: &State<Database>,
    request: GraphQLRequest,
    schema: &State<Schema>,
) -> GraphQLResponse {
    request.execute_sync(schema, context)
}
