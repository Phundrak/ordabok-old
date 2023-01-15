use rocket::response::content::RawHtml;
use rocket::State;

use juniper::EmptySubscription;
use juniper_rocket::{GraphQLRequest, GraphQLResponse};

use crate::appwrite::APVariables;
use crate::db::Database;

#[derive(Default, Debug)]
pub struct Context {
    pub db: Database,
    pub appwrite: APVariables,
}

impl juniper::Context for Context {}

mod query;
use query::Query;

mod mutation;
use mutation::Mutation;

type Schema =
    juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

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
    context: &State<Context>,
    request: GraphQLRequest,
    schema: &State<Schema>,
) -> GraphQLResponse {
    request.execute(schema, context).await
}

#[allow(clippy::needless_pass_by_value)]
#[rocket::post("/graphql", data = "<request>")]
pub async fn post_graphql_handler(
    context: &State<Context>,
    request: GraphQLRequest,
    schema: &State<Schema>,
) -> GraphQLResponse {
    request.execute(schema, context).await
}
