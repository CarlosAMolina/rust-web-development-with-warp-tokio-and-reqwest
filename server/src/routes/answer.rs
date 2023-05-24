use handle_errors::Error;
use std::collections::HashMap;
use warp::http::StatusCode;

use crate::store::Store;
use crate::types::pagination::extract_pagination;
use crate::types::{
    answer::{Answer, AnswerId},
    question::{Question, QuestionId},
};

pub async fn add_answer(
    store: Store,
    params: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    match params.get("content") {
        Some(content) => {
            if content.to_string().is_empty() {
                return Ok(warp::reply::with_status(
                    "Empty parameter: content",
                    StatusCode::RANGE_NOT_SATISFIABLE,
                ));
            }
        }
        None => return Err(warp::reject::custom(Error::MissingParameters)),
    }
    match params.get("questionId") {
        Some(question_id) => {
            if question_id.to_string().is_empty() {
                return Ok(warp::reply::with_status(
                    "Empty parameter: questionId",
                    StatusCode::RANGE_NOT_SATISFIABLE,
                ));
            }
        }
        None => return Err(warp::reject::custom(Error::MissingParameters)),
    }
    let answer_id = {
        let answer_ids: Vec<AnswerId> = store.answers.read().await.keys().cloned().collect();
        if answer_ids.is_empty() {
            0
        } else {
            let mut answer_ids_usize: Vec<i32> =
                answer_ids.iter().map(|answer_id| answer_id.0).collect();
            answer_ids_usize.sort_unstable();
            let max_answer_id = answer_ids_usize[answer_ids_usize.len() - 1];
            max_answer_id + 1
        }
    };
    let exists_question_id = {
        let question_id = QuestionId(params["questionId"].parse::<i32>().unwrap());
        let questions: Vec<Question> = store.questions.read().await.values().cloned().collect();
        let mut question_ids: Vec<QuestionId> = vec![];
        for question in questions.iter() {
            question_ids.push(question.id.clone());
        }
        question_ids.contains(&question_id)
    };
    if !exists_question_id {
        return Err(warp::reject::custom(Error::QuestionNotFound));
    }
    let answer = Answer {
        id: AnswerId(answer_id.try_into().unwrap()),
        content: params["content"].to_string(),
        question_id: QuestionId(params["questionId"].parse::<i32>().unwrap()),
    };
    store
        .answers
        .write()
        .await
        .insert(answer.id.clone(), answer);
    Ok(warp::reply::with_status("Answer added", StatusCode::OK))
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
