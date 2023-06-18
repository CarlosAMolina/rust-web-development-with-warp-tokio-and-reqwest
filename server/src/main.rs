#![warn(clippy::all)]

use clap::Parser;
use config::Config;
use handle_errors::return_error;
// use tracing_subscriber::fmt::format::FmtSpan;
use std::env;
use warp::{http::Method, Filter};

mod profanity;
mod routes;
mod store;
mod types;

#[derive(Parser, Debug, Default, serde::Deserialize, PartialEq)]
struct Args {
    /// URL for the postgres database
    database_host: String,
    /// Database name
    database_name: String,
    /// Database password
    database_password: String,
    /// PORT number for the database connection
    database_port: u16,
    /// Database user
    database_user: String,
    /// Log level handle errors
    log_level_handle_errors: String,
    /// Log level rust-web-dev
    log_level_rust_web_dev: String,
    /// Log level warp
    log_level_warp: String,
    /// Web server port
    web_server_port: u16,
}

#[tokio::main]
async fn main() {
    // The `.tom` file extension in the file name is not required.
    let config = Config::builder()
        .add_source(config::File::with_name("setup"))
        .build()
        .unwrap();
    let config = config.try_deserialize::<Args>().unwrap();
    // Set log level for the application.
    // We pass three:
    // - One for the server implementation: indicated by the
    // application name (rust-web-dev) set in Cargo.toml.
    // - One for Warp.
    let log_filter = std::env::var("RUST_LOG").unwrap_or_else(|_| {
        format!(
            "handle_errors={},rust_web_dev={},warp={}",
            config.log_level_handle_errors, config.log_level_rust_web_dev, config.log_level_warp
        )
    });
    let store = store::Store::new(&format!(
        "postgres://{}:{}@{}:{}/{}",
        config.database_user,
        config.database_password,
        config.database_host,
        config.database_port,
        config.database_name
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

    warp::serve(routes)
        .run(([127, 0, 0, 1], config.web_server_port))
        .await;
}
