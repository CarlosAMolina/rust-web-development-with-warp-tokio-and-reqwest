use std::collections::HashMap;

use tracing::{event, instrument, Level};
use warp::http::StatusCode;

use crate::store::Store;
use crate::types::pagination::{extract_pagination, Pagination};
use crate::types::question::{NewQuestion, Question, QuestionId};
use handle_errors::Error;

pub async fn add_question(
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = store.add_question(new_question).await {
        return Err(warp::reject::custom(Error::DatabaseQueryError(e)));
    }
    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}

// TODO check what happen y ID not in db
pub async fn delete_question(id: i32, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = store.delete_question(id).await {
        return Err(warp::reject::custom(Error::DatabaseQueryError(e)));
    }
    Ok(warp::reply::with_status(
        format!("Question {} deleted", id),
        StatusCode::OK,
    ))
}

// The instrument macro opens and closes a span
// when the function is called.
// All tracing events inside this function will be
// assigned to this span.
// This genereates more logs with more data.
//#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}", params); // TODO rm
                              // TODO use server instead practical_rust_book?
    event!(target: "practical_rust_book", Level::INFO, "querying questions");
    let mut pagination = Pagination::default();
    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        //info!("Pagination set {:?}", &pagination); // TODO set pagintaiton values in log
        pagination = extract_pagination(params)?;
    }
    match store
        .get_questions(pagination.limit, pagination.offset)
        .await
    {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => return Err(warp::reject::custom(Error::DatabaseQueryError(e))),
    }
}

pub async fn update_question(
    id: i32,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res = match store.update_question(question, id).await {
        Ok(res) => return Ok(warp::reply::json(&res)),
        Err(e) => return Err(warp::reject::custom(Error::DatabaseQueryError(e))),
    };
}
