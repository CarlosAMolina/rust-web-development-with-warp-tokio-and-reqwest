use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

use tracing::{event, instrument, Level};
use warp::http::StatusCode;

use crate::store::Store;
use crate::types::pagination::{extract_pagination, Pagination};
use crate::types::question::{NewQuestion, Question, QuestionId};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct APIResponse {
    message: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BadWord {
    original: String,
    word: String,
    deviations: i64,
    info: i64,
    #[serde(rename = "replacedLen")]
    replaced_len: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct BadWordsResponse {
    content: String,
    bad_words_total: i64,
    bad_words_list: Vec<BadWord>,
    censored_content: String,
}

pub async fn add_question(
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    const ENV_VARIABLE: &str = "BAD_WORDS_API_KEY";
    let api_key = env::var(ENV_VARIABLE).expect(&format!("env variable {} not set", ENV_VARIABLE));
    let client = reqwest::Client::new();
    let res = client
        .post("https://api.apilayer.com/bad_words?censor_character=*")
        .header("apikey", api_key)
        .body(new_question.content.clone()) // TODO try removing clone()
        .send()
        .await
        .map_err(|e| handle_errors::Error::ExternalAPIError(e))?;
    if !res.status().is_success() {
        if res.status().is_client_error() {
            let err = transform_error(res).await;
            return Err(handle_errors::Error::ServerError(err).into());
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

    let res = res
        .json::<BadWordsResponse>()
        .await
        .map_err(|e| handle_errors::Error::ExternalAPIError(e))?;
    let content = res.censored_content;
    let question = NewQuestion {
        title: new_question.title,
        content,
        tags: new_question.tags,
    };
    match store.add_question(question).await {
        Ok(question) => Ok(warp::reply::json(&question)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

async fn transform_error(res: reqwest::Response) -> handle_errors::APILayerError {
    handle_errors::APILayerError {
        status: res.status().as_u16(),
        message: res.json::<APIResponse>().await.unwrap().message,
    }
}

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
