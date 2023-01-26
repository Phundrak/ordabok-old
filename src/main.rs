#![warn(clippy::style, clippy::pedantic)]

mod appwrite;
mod db;
mod graphql;

use std::{collections::HashSet, env, error::Error};

use dotenvy::dotenv;
use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;

fn setup_logging() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Setting default subscriber failed");
}

fn make_cors() -> Result<rocket_cors::Cors, rocket_cors::Error> {
    use rocket::http::Method as HMethod;
    use rocket_cors::{AllowedHeaders, AllowedOrigins, Method};
    rocket_cors::CorsOptions {
        allowed_methods: HashSet::from([
            Method::from(HMethod::Get),
            Method::from(HMethod::Post),
        ]),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Content-Type",
        ]),
        allowed_origins: match env::var("ORDABOK_HOSTS") {
            Ok(val) => {
                if val.is_empty() {
                    AllowedOrigins::all()
                } else {
                    AllowedOrigins::some_exact(
                        val.split(',').collect::<Vec<_>>().as_ref(),
                    )
                }
            }
            Err(_) => AllowedOrigins::all(),
        },
        ..Default::default()
    }
    .to_cors()
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    use graphql::{
        create_schema, get_graphql_handler, graphiql, post_graphql_handler,
    };

    color_eyre::install()?;
    setup_logging();

    info!("Reading environment variables");
    dotenv().ok();

    let cors = make_cors()?;
    debug!("CORS: {:?}", cors);

    #[allow(clippy::let_underscore_drop, clippy::no_effect_underscore_binding)]
    let _ = rocket::build()
        .attach(cors)
        .manage(graphql::Context::default())
        .manage(create_schema())
        .mount(
            "/",
            rocket::routes![
                graphiql,
                get_graphql_handler,
                post_graphql_handler
            ],
        )
        .launch()
        .await?;
    Ok(())
}
