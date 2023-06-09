use std::collections::HashMap;

//use tracing::{event, instrument, Level};
use tracing::{event, Level};
use warp::http::StatusCode;

// use crate::profanity::check_profanity;
use crate::store::Store;
use crate::types::account::Session;
use crate::types::pagination::{extract_pagination, Pagination};
use crate::types::question::{NewQuestion, Question};

pub async fn add_question(
    session: Session,
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    //let title = match check_profanity(new_question.title).await {
    //    Ok(res) => res,
    //    Err(e) => return Err(warp::reject::custom(e)),
    //};
    //let content = match check_profanity(new_question.content).await {
    //    Ok(res) => res,
    //    Err(e) => return Err(warp::reject::custom(e)),
    //};
    event!(Level::INFO, "Init add");
    let account_id = session.account_id;
    let question = NewQuestion {
        title: new_question.title,
        content: new_question.content,
        tags: new_question.tags,
    };
    match store.add_question(question, account_id).await {
        Ok(_) => Ok(warp::reply::with_status("Question added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

// TODO check what happen y ID not in db
pub async fn delete_question(
    id: i32,
    session: Session,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    if store.is_question_owner(id, &account_id).await? {
        match store.delete_question(id, account_id).await {
            Ok(_) => Ok(warp::reply::with_status(
                format!("Question {} deleted", id),
                StatusCode::OK,
            )),
            Err(e) => Err(warp::reject::custom(e)),
        }
    } else {
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }
}

// TODO check what happen y ID not in db
pub async fn get_question(id: i32, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    event!(Level::INFO, "Init");
    match store.get_question(id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => return Err(warp::reject::custom(e)),
    }
}

// The instrument macro opens and closes a span
// when the function is called.
// All tracing events inside this function will be
// assigned to this span.
// instrument: genereates more logs with more data.
//#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(Level::INFO, "Init. params: {:?}", params);
    let mut pagination = Pagination::default();
    if !params.is_empty() {
        pagination = extract_pagination(params)?;
    }
    match store
        .get_questions(pagination.limit, pagination.offset)
        .await
    {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => return Err(warp::reject::custom(e)),
    }
}

pub async fn update_question(
    id: i32,
    session: Session,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    //let title = check_profanity(question.title);
    //let content = check_profanity(question.content);
    // Instead of the spawn, we don’t wrap the function calls separately, we call them inside the join! macro without any await.
    //let (title, content) = tokio::join!(title, content);
    //if title.is_err() {
    //    return Err(warp::reject::custom(title.unwrap_err()));
    //}
    //if content.is_err() {
    //    return Err(warp::reject::custom(content.unwrap_err()));
    //}
    //let question = Question {
    //    id: question.id,
    //    title: title.unwrap(),
    //    content: content.unwrap(),
    //    tags: question.tags,
    //};
    event!(Level::INFO, "Init update");
    let account_id = session.account_id;
    if store.is_question_owner(id, &account_id).await? {
        let question = Question {
            id: question.id,
            title: question.title,
            content: question.content,
            tags: question.tags,
        };
        match store.update_question(question, id, account_id).await {
            Ok(res) => Ok(warp::reply::json(&res)),
            Err(e) => Err(warp::reject::custom(e)),
        }
    } else {
        Err(warp::reject::custom(handle_errors::Error::Unauthorized))
    }
}
