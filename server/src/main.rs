use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use warp::{
    filters::cors::CorsForbidden, http::Method, http::StatusCode, Filter, Rejection, Reply,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
struct QuestionId(String);

async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    // TODO only the 1st param is detected:
    // curl localhost:3030/questions?start=1&end=10
    println!("{:?}", params);
    let mut start = 0;
    if let Some(n) = params.get("start") {
        start = n.parse::<usize>().expect("Could not parse start");
    }
    println!("{}", start);
    let res: Vec<Question> = store.questions.values().cloned().collect();
    Ok(warp::reply::json(&res))
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    println!("{:?}", r);
    // Example request to call this function:
    // ```bash
    // curl -X OPTIONS localhost:3030/questions \
    //      -H "Access-Control-Request-Method: PUT" \
    //      -H "Access-Control-Request-Headers: invalid-header" \
    //      -H "Origin: https://not-origin.io" -verbose
    // ```
    if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}

#[derive(Clone)]
struct Store {
    questions: HashMap<QuestionId, Question>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: Self::init(),
        }
    }

    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../question.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }
}

#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter)
        .and_then(get_questions);

    let routes = get_questions.with(cors).recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
