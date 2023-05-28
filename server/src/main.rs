#![warn(clippy::all)]

use handle_errors::return_error;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, Filter};

mod routes;
mod store;
mod types;

#[tokio::main]
async fn main() {
    // Set log level for the application.
    // We pass three:
    // - One for the server implementation: indicated by the
    // application name (server) set in Cargo.toml.
    // - One for Warp.
    let log_filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "handle_errors=warn,server=warn,warp=warn".to_owned());
    // "postgres://username:password@localhost:5432/rustwebdev"
    let store = store::Store::new("postgres://postgres:pw@localhost:5432/rustwebdev").await;
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
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    // TODO let get_answers = warp::get()
    // TODO     .and(warp::path("answers"))
    // TODO     .and(warp::path::end())
    // TODO     .and(warp::query())
    // TODO     .and(store_filter.clone())
    // TODO     .and_then(routes::answer::get_answers);

    // TODO let get_answers_of_question = warp::get()
    // TODO     .and(warp::path("questions"))
    // TODO     .and(warp::path::param::<i32>())
    // TODO     .and(warp::path("answers"))
    // TODO     .and(warp::path::end())
    // TODO     .and(store_filter.clone())
    // TODO     .and_then(routes::answer::get_answers_of_question);

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

    //TODO let get_question = warp::get()
    //TODO     .and(warp::path("questions"))
    //TODO     .and(warp::path::param::<i32>())
    //TODO     .and(warp::path::end())
    //TODO     .and(store_filter.clone())
    //TODO     .and_then(routes::question::get_question);

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::add_question);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::update_question);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::question::delete_question);

    let add_answer = warp::post()
        .and(warp::path("comments"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::form())
        .and_then(routes::answer::add_answer);

    let routes = add_question
        //TODO let routes = get_answers
        // TODO .or(get_answers_of_question)
        .or(get_questions)
        //TODO .or(get_question)
        //TODO .or(add_question)
        .or(add_answer)
        .or(update_question)
        .or(delete_question)
        .with(cors)
        .with(warp::trace::request())
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
