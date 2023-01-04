use rocket::response::content::RawHtml;
use rocket::State;

use juniper::EmptySubscription;
use juniper_rocket::{GraphQLRequest, GraphQLResponse};

use crate::db::models::languages::Language;
use crate::db::Database;

#[derive(Debug)]
pub struct Query;

#[juniper::graphql_object(Context = Database)]
impl Query {
    #[graphql(name = "allLanguages")]
    fn all_languages(context: &Database) -> Vec<Language> {
        context.all_languages().unwrap()
    }

    fn language(context: &Database, name: String) -> Option<Language> {
        context.language(name.as_str())
    }
}

pub struct Mutation;

#[juniper::graphql_object(Context = Database)]
impl Mutation {
    fn api_version() -> String {
        "0.1".into()
    }
}

type Schema = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Database>>;

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
    schema: &State<Schema>
) -> GraphQLResponse {
    request.execute_sync(schema, context)
}
