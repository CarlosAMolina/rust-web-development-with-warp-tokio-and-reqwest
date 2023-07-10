use argon2::{self, Config};
use chrono::prelude::Utc;
use paseto;
use rand::Rng;
use std::{env, future};
use tracing::{event, Level};
use warp::{http::StatusCode, Filter};

use crate::store::Store;
use crate::types::account::{Account, AccountId, Session};

pub async fn register(store: Store, account: Account) -> Result<impl warp::Reply, warp::Rejection> {
    event!(Level::INFO, "Init register");
    let hashed_password = hash_password(account.password.as_bytes());
    let account = Account {
        id: account.id,
        email: account.email,
        password: hashed_password,
    };
    match store.add_account(account).await {
        Ok(_) => Ok(warp::reply::with_status("Account added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub fn hash_password(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

pub async fn login(store: Store, login: Account) -> Result<impl warp::Reply, warp::Rejection> {
    event!(Level::INFO, "Init login");
    match store.get_account(login.email).await {
        Ok(account) => match verify_password(&account.password, login.password.as_bytes()) {
            Ok(verified) => {
                if verified {
                    Ok(warp::reply::json(&issue_token(
                        account.id.expect("id not found"),
                    )))
                } else {
                    Err(warp::reject::custom(handle_errors::Error::WrongPassword))
                }
            }
            Err(e) => Err(warp::reject::custom(
                handle_errors::Error::ArgonLibraryError(e),
            )),
        },
        Err(e) => Err(warp::reject::custom(e)),
    }
}

fn verify_password(hash: &str, password: &[u8]) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password)
}

pub fn verify_token(token: String) -> Result<Session, handle_errors::Error> {
    let key = env::var("PASETO_KEY").unwrap();
    let token = paseto::tokens::validate_local_token(
        &token,
        None,
        key.as_bytes(),
        &paseto::tokens::TimeBackend::Chrono,
    )
    .map_err(|_| handle_errors::Error::CannotDecryptToken)?;
    serde_json::from_value::<Session>(token).map_err(|_| handle_errors::Error::CannotDecryptToken)
}

fn issue_token(account_id: AccountId) -> String {
    let key = env::var("PASETO_KEY").unwrap();
    let current_date_time = Utc::now();
    let expiration_date_time = current_date_time + chrono::Duration::days(1);
    // Instead of using the JWT format, we use Paseto, which has a stronger algorithm.
    paseto::tokens::PasetoBuilder::new()
        .set_encryption_key(&Vec::from(key.as_bytes()))
        .set_expiration(&expiration_date_time)
        .set_claim("account_id", serde_json::json!(account_id))
        .build()
        .expect("Failed to construct paseto token w/ builder!")
}

// We return a type that implements the Filter trait that expects the generic type Session, or an Error that implements Warp’s Rejection trait. With `+ Clone` the returned Filter can be clone.
// `future::ready` returns a type Ready with the Result inside it.
pub fn auth() -> impl Filter<Extract = (Session,), Error = warp::Rejection> + Clone {
    warp::header::<String>("Authorization").and_then(|token: String| {
        let token = match verify_token(token) {
            Ok(t) => t,
            Err(_) => {
                return future::ready(Err(warp::reject::custom(
                    handle_errors::Error::Unauthorized,
                )))
            }
        };

        future::ready(Ok(token))
    })
}

#[cfg(test)]
mod authentication_tests {
    use super::{auth, env, issue_token, AccountId};

    #[tokio::test]
    async fn post_questions_auth() {
        // Set the PASETO_KEY env variable; otherwise, the issue_token function,
        // which auth calls in the background, would fail.
        // It's important to set the same value in all tests to not affect
        // other tests.
        env::set_var("PASETO_KEY", "RANDOM WORDS WINTER MACINTOSH PC");
        // Issues a new token that we can pass to your test request in the
        // Authorization header.
        let token = issue_token(AccountId(3));
        let filter = auth();
        // Calls create-a-test request with a header and passes it to the filter,
        // which is our auth function.
        let res = warp::test::request()
            .header("Authorization", token)
            .filter(&filter);
        // Awaits the response and gets a session back, where we compare the account_id
        // from the session with the one we issued the token with.
        assert_eq!(res.await.unwrap().account_id, AccountId(3));
    }
}
