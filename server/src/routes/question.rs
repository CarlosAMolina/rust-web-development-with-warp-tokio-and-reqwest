use std::collections::HashMap;

use tracing::{info, instrument};
use warp::http::StatusCode;

use crate::store::Store;
use crate::types::pagination::extract_pagination;
use crate::types::question::{Question, QuestionId};
use handle_errors::Error;

pub async fn add_question(
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    store
        .questions
        .write()
        .await
        .insert(question.id.clone(), question);
    Ok(warp::reply::with_status("Question added", StatusCode::OK))
}

pub async fn delete_question(
    id: String,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.remove(&QuestionId(id)) {
        Some(_) => Ok(warp::reply::with_status("Question deleted", StatusCode::OK)),
        None => Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}

pub async fn get_question(id: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.read().await.get(&QuestionId(id)) {
        Some(question) => Ok(warp::reply::json(&question)),
        None => Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}

// The instrument macro opens and closes a span
// when the function is called.
// All tracing events inside this function will be
// assigned to this span.
#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    info!("querying questions");
    println!("{:?}", params);
    if params.is_empty() {
        info!("no pagination used");
        info!(pagination = false);
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        Ok(warp::reply::json(&res))
    } else {
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        let pagination = extract_pagination(params, res.len())?;
        info!("Pagination set {:?}", &pagination);
        info!(pagination = true);
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    }
}

pub async fn update_question(
    id: String,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.get_mut(&QuestionId(id)) {
        Some(q) => *q = question,
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }
    Ok(warp::reply::with_status("Question updated", StatusCode::OK))
}
