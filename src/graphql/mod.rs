use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::content::RawHtml;
use rocket::State;

use tracing::debug;

use juniper::EmptySubscription;
use juniper_rocket::{GraphQLRequest, GraphQLResponse};

pub mod context;
pub use context::Context;

mod mutation;
mod query;

#[derive(Copy, Clone, Debug)]
pub struct UserAuth<'r>(Option<&'r str>);

#[derive(Debug)]
pub enum UserAuthError {}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserAuth<'r> {
    type Error = UserAuthError;
    async fn from_request(
        request: &'r Request<'_>,
    ) -> Outcome<Self, Self::Error> {
        match request.headers().get_one("Authorization") {
            None => Outcome::Success(UserAuth(None)),
            Some(key) => Outcome::Success(UserAuth(Some(key))),
        }
    }
}

pub type Schema = juniper::RootNode<
    'static,
    query::Query,
    mutation::Mutation,
    EmptySubscription<Context>,
>;

pub fn create_schema() -> Schema {
    Schema::new(
        query::Query {},
        mutation::Mutation {},
        EmptySubscription::default(),
    )
}

#[rocket::get("/")]
pub fn graphiql() -> RawHtml<String> {
    let graphql_endpoint_url = "/graphql";
    juniper_rocket::graphiql_source(graphql_endpoint_url, None)
}

#[rocket::get("/graphql?<request>")]
pub async fn get_graphql_handler(
    context: &State<Context>,
    user_auth: UserAuth<'_>,
    request: GraphQLRequest,
    schema: &State<Schema>,
) -> GraphQLResponse {
    debug!("Current context: {:?}", context);
    request
        .execute(schema, &(*context).attach_auth(user_auth.0).await)
        .await
}

#[allow(clippy::needless_pass_by_value)]
#[rocket::post("/graphql", data = "<request>")]
pub async fn post_graphql_handler(
    context: &State<Context>,
    user_auth: UserAuth<'_>,
    request: GraphQLRequest,
    schema: &State<Schema>,
) -> GraphQLResponse {
    debug!("Current context: {:?}", context);
    request
        .execute(schema, &(*context).attach_auth(user_auth.0).await)
        .await
}
