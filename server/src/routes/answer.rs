use handle_errors::Error;
use std::collections::HashMap;
use warp::http::StatusCode;

// use crate::profanity::check_profanity;
use crate::store::Store;
use crate::types::pagination::{extract_pagination, Pagination};
use crate::types::{
    answer::{Answer, AnswerId, NewAnswer},
    question::{Question, QuestionId},
};
use tracing::{event, instrument, Level};

pub async fn add_answer(
    store: Store,
    new_answer: NewAnswer,
) -> Result<impl warp::Reply, warp::Rejection> {
    let content = new_answer.content;
    // let content = match
    //     check_profanity(new_answer.content).await {
    //     Ok(res) => res,
    //     Err(e) => return Err(warp::reject::custom(e)),
    // };
    let answer = NewAnswer {
        content,
        question_id: new_answer.question_id,
    };
    match store.add_answer(answer).await {
        Ok(_) => Ok(warp::reply::with_status("Answer added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

#[instrument]
pub async fn get_answers(
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
    match store.get_answers(pagination.limit, pagination.offset).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => return Err(warp::reject::custom(e)),
    }
}

pub async fn get_answers_of_question(
    question_id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.get_answers_of_question(question_id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => return Err(warp::reject::custom(e)),
    }
    //None => Err(warp::reject::custom(Error::QuestionNotFound)), // TODO create this error
}
