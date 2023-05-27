use handle_errors::Error;
use std::collections::HashMap;
use warp::http::StatusCode;

use crate::store::Store;
use crate::types::pagination::extract_pagination;
use crate::types::{
    answer::{Answer, AnswerId, NewAnswer},
    question::{Question, QuestionId},
};

pub async fn add_answer(
    store: Store,
    new_anser: NewAnswer,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.add_answer(new_answer).await {
        Ok(_) => Ok(warp::reply::with_status("Answer added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn get_answers(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    if params.is_empty() {
        let res: Vec<Answer> = store.answers.read().await.values().cloned().collect();
        Ok(warp::reply::json(&res))
    } else {
        let res: Vec<Answer> = store.answers.read().await.values().cloned().collect();
        let pagination = extract_pagination(params, res.len())?;
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    }
}

pub async fn get_answers_of_question(
    id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.read().await.get(&QuestionId(id)) {
        Some(question) => {
            let answers_all: Vec<Answer> = store.answers.read().await.values().cloned().collect();
            let answers: Vec<Answer> = answers_all
                .iter()
                .filter(|answer| answer.question_id == question.id)
                .cloned()
                .collect();
            Ok(warp::reply::json(&answers))
        }
        None => Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}
