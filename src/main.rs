#![warn(clippy::style, clippy::pedantic)]

mod db;

use std::{env, error::Error};

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
    use rocket_cors::{AllowedHeaders, AllowedOrigins};
    use std::str::FromStr;
    rocket_cors::CorsOptions {
        allowed_methods: ["Get", "Post"]
            .iter()
            .map(|s| FromStr::from_str(s).unwrap())
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
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
    setup_logging();

    info!("Reading environment variables");
    dotenv().ok();

    let cors = make_cors()?;
    debug!("CORS: {:?}", cors);

    #[allow(clippy::let_underscore_drop)]
    let _ = rocket::build().attach(cors).launch().await?;
    Ok(())
}
