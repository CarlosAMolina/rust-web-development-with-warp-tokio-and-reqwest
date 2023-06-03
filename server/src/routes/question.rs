use std::collections::HashMap;

use tracing::{event, instrument, Level};
use warp::http::StatusCode;

use crate::profanity::check_profanity;
use crate::store::Store;
use crate::types::pagination::{extract_pagination, Pagination};
use crate::types::question::{NewQuestion, Question, QuestionId};

pub async fn add_question(
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    let title = match check_profanity(new_question.title).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    let content = match check_profanity(new_question.content).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    let question = NewQuestion {
        title,
        content,
        tags: new_question.tags,
    };
    match store.add_question(question).await {
        Ok(_) => Ok(warp::reply::with_status("Question added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

// TODO RM // The raised exceptions do not cover all cases:
// TODO RM // https://docs.rs/reqwest/latest/reqwest/struct.RequestBuilder.html#method.send
// TODO RM // Add raise exception with nonsucess status code in the response.
// TODO RM // Example, if we set the apiKey header wrong.
// TODO RM match res.error_for_status() {
// TODO RM     Ok(res) => {
// TODO RM         let res = res
// TODO RM             .text()
// TODO RM             .await
// TODO RM             .map_err(|e| handle_errors::Error::ExternalAPIError(e))?;
// TODO RM         println!("{}", res); // TODO rm
// TODO RM         match store.add_question(new_question).await {
// TODO RM             Ok(_) => Ok(warp::reply::with_status("Question added", StatusCode::OK)),
// TODO RM             Err(e) => Err(warp::reject::custom(e)),
// TODO RM         }
// TODO RM     }
// TODO RM     Err(err) => Err(warp::reject::custom(
// TODO RM         handle_errors::Error::ExternalAPIError(err),
// TODO RM     )),
// TODO RM }

// TODO rm     let res = res
// TODO rm         .json::<BadWordsResponse>()
// TODO rm         .await
// TODO rm         .map_err(|e| handle_errors::Error::ExternalAPIError(e))?;
// TODO rm     let content = res.censored_content;
// TODO rm     let question = NewQuestion {
// TODO rm         title: new_question.title,
// TODO rm         content,
// TODO rm         tags: new_question.tags,
// TODO rm     };
// TODO rm     match store.add_question(question).await {
// TODO rm         Ok(question) => Ok(warp::reply::json(&question)),
// TODO rm         Err(e) => Err(warp::reject::custom(e)),
// TODO rm     }
// TODO rm }

// TODO check what happen y ID not in db
pub async fn delete_question(id: i32, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.delete_question(id).await {
        Ok(_) => Ok(warp::reply::with_status(
            format!("Question {} deleted", id),
            StatusCode::OK,
        )),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

// The instrument macro opens and closes a span
// when the function is called.
// All tracing events inside this function will be
// assigned to this span.
// This genereates more logs with more data.
#[instrument]
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
        Err(e) => return Err(warp::reject::custom(e)),
    }
}

pub async fn update_question(
    id: i32,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.update_question(question, id).await {
        Ok(res) => return Ok(warp::reply::json(&res)),
        Err(e) => return Err(warp::reject::custom(e)),
    };
}
