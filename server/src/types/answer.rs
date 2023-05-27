use serde::{Deserialize, Serialize};

use crate::types::question::QuestionId;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: QuestionId,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AnswerId(pub i32);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewAnswer {
    pub content: String,
    pub question_id: QuestionId,
}
