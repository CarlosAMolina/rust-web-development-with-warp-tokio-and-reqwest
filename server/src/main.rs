#![warn(clippy::all)]

use clap::Parser;
use dotenv;
use handle_errors::return_error;
use std::env;
// use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, Filter};

mod profanity;
mod routes;
mod store;
mod types;

/// Q&A web service API
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// URL for the postgres database
    #[clap(long, default_value = "localhost")]
    database_host: String,
    /// Database name
    #[clap(long, default_value = "rustwebdev")]
    database_name: String,
    /// Database password
    #[clap(long, default_value = "pw")]
    database_password: String,
    /// PORT number for the database connection
    #[clap(long, default_value = "5432")]
    database_port: u16,
    /// Database user
    #[clap(long, default_value = "postgres")]
    database_user: String,
    /// Which errors we want to log (info, warn or error)
    /// Log level handle errors
    #[clap(long, default_value = "warn")]
    log_level_handle_errors: String,
    /// Log level rust-web-dev
    #[clap(long, default_value = "info")]
    log_level_rust_web_dev: String,
    /// Log level warp
    #[clap(long, default_value = "error")]
    log_level_warp: String,
    /// Web server port
    #[clap(long, default_value = "3030")]
    web_server_port: u16,
}

#[tokio::main]
async fn main() {
    // Initialize the .env file via the dotenv crate.
    dotenv::dotenv().ok();
    if let Err(_) = env::var("BAD_WORDS_API_KEY") {
        panic!("BadWords API key not set");
    }
    if let Err(_) = env::var("PASETO_KEY") {
        panic!("PASETO key not set");
    }
    let port = std::env::var("PORT")
        .ok()
        .map(|val| val.parse::<u16>())
        .unwrap_or(Ok(8080))
        .map_err(|e| handle_errors::Error::ParseError(e)).expect("Cannot parse port");
    let args = Args::parse();
    // Set log level for the application.
    // We pass three:
    // - One for the server implementation: indicated by the
    // application name (rust-web-dev) set in Cargo.toml.
    // - One for Warp.
    let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| {
        format!(
            "handle_errors={},rust_web_dev={},warp={}",
            args.log_level_handle_errors, args.log_level_rust_web_dev, args.log_level_warp
        )
    });
    let store = store::Store::new(&format!(
        "postgres://{}:{}@{}:{}/{}",
        args.database_user,
        args.database_password,
        args.database_host,
        args.database_port,
        args.database_name
    ))
    .await;
    sqlx::migrate!("../db/migrations")
        .run(&store.clone().connection)
        .await
        .expect("Cannot run migration");
    let store_filter = warp::any().map(move || store.clone());
    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(log_filter)
        // Record an event when each span closes.
        // This can be used to time our
        // routes' durations!
        //.with_span_events(FmtSpan::CLOSE)
        .init();

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_answers = warp::get()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(routes::answer::get_answers);

    let get_answers_of_question = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::answer::get_answers_of_question)
        .with(warp::trace(|info| {
            tracing::info_span!(
                  "get_answers_of_question request",
                  method = %info.method(),
                  path = %info.path(),
                  id = %uuid::Uuid::new_v4(),
            )
        }));

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(routes::question::get_questions)
        .with(warp::trace(|info| {
            tracing::info_span!(
                  "get_questions request",
                  method = %info.method(),
                  path = %info.path(),
                  id = %uuid::Uuid::new_v4(),
            )
        }));

    let get_question = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::question::get_question)
        .with(warp::trace(|info| {
            tracing::info_span!(
                  "get_question request",
                  method = %info.method(),
                  path = %info.path(),
                  id = %uuid::Uuid::new_v4(),
            )
        }));

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::add_question);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::update_question);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and_then(routes::question::delete_question);

    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(routes::authentication::auth())
        .and(store_filter.clone())
        .and(warp::body::form())
        .and_then(routes::answer::add_answer);

    let registration = warp::post()
        .and(warp::path("registration"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::authentication::register);

    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::authentication::login);

    let routes = add_answer
        .or(add_question)
        .or(delete_question)
        .or(get_answers)
        .or(get_answers_of_question)
        .or(get_question)
        .or(get_questions)
        .or(login)
        .or(registration)
        .or(update_question)
        .with(cors)
        .with(warp::trace::request())
        .recover(return_error);

    tracing::info!("Q&A service build ID {}", env!("RUST_WEB_DEV_VERSION"));

    warp::serve(routes)
        .run(([127, 0, 0, 1], port))
        .await;
}
