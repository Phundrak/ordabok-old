use rocket::response::content::RawHtml;
use rocket::State;

use juniper::EmptySubscription;
use juniper_rocket::{GraphQLRequest, GraphQLResponse};

use crate::db::models::{languages::Language, users::User, words::Word};
use crate::db::Database;

#[derive(Debug)]
pub struct Query;

#[juniper::graphql_object(Context = Database)]
impl Query {
    #[graphql(name = "allLanguages")]
    fn all_languages(context: &Database) -> Vec<Language> {
        context.all_languages().unwrap()
    }

    #[graphql(
        description = "Retrieve a specific language from its name and its owner's id"
    )]
    fn language(
        context: &Database,
        name: String,
        owner: String,
    ) -> Option<Language> {
        context.language(name.as_str(), owner.as_str())
    }

    #[graphql(description = "Retrieve a specific user from its id")]
    fn user(context: &Database, id: String) -> Option<User> {
        context.user(id.as_str())
    }

    #[graphql(description = "Retrieve a specific word from its id")]
    fn word(context: &Database, id: String) -> Option<Word> {
        context.word_id(id.as_str())
    }

    #[graphql(
        description = "Retrieve all words with a set normal form from a set language"
    )]
    fn words(context: &Database, language: String, word: String) -> Vec<Word> {
        context.words(language.as_str(), word.as_str())
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
