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
    new_answer: NewAnswer,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.add_answer(new_answer).await {
        Ok(_) => Ok(warp::reply::with_status("Answer added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

// TODO pub async fn get_answers(
// TODO     params: HashMap<String, String>,
// TODO     store: Store,
// TODO ) -> Result<impl warp::Reply, warp::Rejection> {
// TODO     if params.is_empty() {
// TODO         let res: Vec<Answer> = store.answers.read().await.values().cloned().collect();
// TODO         Ok(warp::reply::json(&res))
// TODO     } else {
// TODO         let res: Vec<Answer> = store.answers.read().await.values().cloned().collect();
// TODO         let pagination = extract_pagination(params, res.len())?;
// TODO         let res = &res[pagination.offset..pagination.limit];
// TODO         Ok(warp::reply::json(&res))
// TODO     }
// TODO }
// TODO
// TODO pub async fn get_answers_of_question(
// TODO     id: i32,
// TODO     store: Store,
// TODO ) -> Result<impl warp::Reply, warp::Rejection> {
// TODO     match store.questions.read().await.get(&QuestionId(id)) {
// TODO         Some(question) => {
// TODO             let answers_all: Vec<Answer> = store.answers.read().await.values().cloned().collect();
// TODO             let answers: Vec<Answer> = answers_all
// TODO                 .iter()
// TODO                 .filter(|answer| answer.question_id == question.id)
// TODO                 .cloned()
// TODO                 .collect();
// TODO             Ok(warp::reply::json(&answers))
// TODO         }
// TODO         None => Err(warp::reject::custom(Error::QuestionNotFound)),
// TODO     }
// TODO }
