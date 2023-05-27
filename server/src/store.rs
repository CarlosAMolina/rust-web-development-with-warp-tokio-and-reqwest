use sqlx::postgres::{PgPool, PgPoolOptions, PgRow};
use sqlx::Row;

use crate::types::question::{Question, QuestionId};
use handle_errors::Error;

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish DB connection:[]", e),
        };

        Store {
            connection: db_pool,
        }
    }

    pub async fn get_questions(
        &self,
        limit: Option<u32>,
        offset: u32,
    ) -> Result<Vec<Question>, sqlx::Error> {
        match sqlx::query("SELECT * from questions LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(questions) => Ok(questions),
            Err(e) => Err(e),
        }
    }
}
