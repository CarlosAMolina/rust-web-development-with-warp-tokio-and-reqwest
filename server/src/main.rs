use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use serde::{Deserialize, Serialize};
use warp::{
    filters::{body::BodyDeserializeError, cors::CorsForbidden},
    http::Method,
    http::StatusCode,
    reject::Reject,
    Filter, Rejection, Reply,
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


#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
struct AnswerId(String);

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Answer {
    id: AnswerId,
    content: String,
    question_id: QuestionId,
}


fn extract_pagination(
    params: HashMap<String, String>,
    response_length: usize,
) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        let start = params
            .get("start")
            .unwrap()
            .parse::<usize>()
            .map_err(Error::ParseError)?;
        let mut end = params
            .get("end")
            .unwrap()
            .parse::<usize>()
            .map_err(Error::ParseError)?;
        if start > response_length {
            return Err(Error::StartGreaterThanEnd);
        }
        if end > response_length {
            end = response_length;
        }
        return Ok(Pagination { start, end });
    }
    Err(Error::MissingParameters)
}

async fn get_answers(
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

async fn get_answers_of_question(
    id: String,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.read().await.get(&QuestionId(id)) {
        Some(question) =>  {
            let answers_all: Vec<Answer> = store.answers.read().await.values().cloned().collect();
            let mut answers: Vec<Answer> = vec![];
            for answer in answers_all.iter() {
                if answer.question_id == question.id {
                    answers.push(answer.clone());
                }
            }
            Ok(warp::reply::json(&answers))
        },
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}


async fn get_questions(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}", params);
    if params.is_empty() {
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        Ok(warp::reply::json(&res))
    } else {
        let res: Vec<Question> = store.questions.read().await.values().cloned().collect();
        let pagination = extract_pagination(params, res.len())?;
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    }
}

async fn get_question(
    id: String,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.read().await.get(&QuestionId(id)) {
        Some(question) =>  Ok(warp::reply::json(&question)),
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}

async fn add_question(
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

async fn update_question(
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

async fn delete_question(id: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.questions.write().await.remove(&QuestionId(id)) {
        Some(_) => return Ok(warp::reply::with_status("Question deleted", StatusCode::OK)),
        None => return Err(warp::reject::custom(Error::QuestionNotFound)),
    }
}

async fn add_answer(
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
        },
        None => return Err(warp::reject::custom(Error::MissingParameters))
    }
    match params.get("questionId") {
        Some(question_id) => {
            if question_id.to_string().is_empty() {
                return Ok(warp::reply::with_status(
                    "Empty parameter: questionId",
                    StatusCode::RANGE_NOT_SATISFIABLE,
                ));
            }
        },
        None => return Err(warp::reject::custom(Error::MissingParameters))
    }
    let answer_id = {
        let answers: Vec<Answer> = store.answers.read().await.values().cloned().collect();
        if answers.is_empty() {
            0
        }
        else {
            let mut max_answer_id = 0;
            for answer in answers.iter() {
                let answer_id = answer.id.0.parse::<usize>().unwrap();
                if answer_id > max_answer_id {
                    max_answer_id = answer_id;
                }
            }
            max_answer_id + 1
        }
    };
    let exists_question_id = {
        let question_id = QuestionId(params["questionId"].to_string());
        let questions: Vec<Question> = store.questions.read().await.values().cloned().collect();
        let mut question_ids: Vec<QuestionId> = vec![];
        for question in questions.iter() {
            question_ids.push(question.id.clone());
        }
        question_ids.contains(&question_id)
    };
    if !exists_question_id {
        return Err(warp::reject::custom(Error::QuestionNotFound))
    }
    let answer = Answer {
        id: AnswerId(answer_id.to_string()),
        content: params["content"].to_string(),
        question_id: QuestionId(params["questionId"].to_string()),
    };
    store
        .answers
        .write()
        .await
        .insert(answer.id.clone(), answer);
    return Ok(warp::reply::with_status("Answer added", StatusCode::OK))
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    println!("{:?}", r);
    if let Some(error) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}

#[derive(Debug)]
struct Pagination {
    start: usize,
    end: usize,
}

#[derive(Clone)]
struct Store {
    questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
    answers: Arc<RwLock<HashMap<AnswerId, Answer>>>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Self::init())),
            answers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }
}

#[derive(Debug)]
enum Error {
    MissingParameters,
    ParseError(std::num::ParseIntError),
    QuestionNotFound,
    StartGreaterThanEnd,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::QuestionNotFound => write!(f, "Question not found"),
            Error::MissingParameters => write!(f, "Missing parameter"),
            Error::ParseError(ref err) => {
                write!(f, "Cannot parse parameter: {}", err)
            },
            Error::StartGreaterThanEnd => write!(f, "The start is greater than the end"),
        }
    }
}

impl Reject for Error {}

#[tokio::main]
async fn main() {
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_answers = warp::get()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(get_answers);

    let get_answers_of_question = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_answers_of_question);

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(get_questions);

    let get_question = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_question);

    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(add_question);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(update_question);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(delete_question);

    let add_answer = warp::post()
        .and(warp::path("comments"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::form())
        .and_then(add_answer);

    let routes = get_answers
        .or(get_answers_of_question)
        .or(get_questions)
        .or(get_question)
        .or(add_question)
        .or(add_answer)
        .or(update_question)
        .or(delete_question)
        .with(cors)
        .recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
