use std::io::{self, Write};
use std::process::Command;

use rust_web_dev::{config, handle_errors, oneshot, setup_store};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    email: String,
    password: String,
}

#[tokio::main]
async fn main() -> Result<(), handle_errors::Error> {
    dotenv::dotenv().ok();
    let config = config::Config::new().expect("Config can't be set");

    let s = Command::new("sqlx")
        .arg("database")
        .arg("drop")
        .arg("--database-url")
        .arg(format!(
            "postgres://{}:{}@{}:{}/{}",
            config.database_user,
            config.database_password,
            config.database_host,
            config.database_port,
            config.database_name
        ))
        .arg("-y")
        // The output function will create the final command, which we can use to execute later.
        .output()
        .expect("sqlx command failed to start");

    // Execute DB commands.
    // Uses the stdout function to write our command to the command line and execute it.
    // Execute the command via the write_all command, and
    // print out errors if they happen via the stderr field.
    io::stdout().write_all(&s.stderr).unwrap();

    let s = Command::new("sqlx")
        .arg("database")
        .arg("create")
        .arg("--database-url")
        .arg(format!(
            "postgres://{}:{}@{}:{}/{}",
            config.database_user,
            config.database_password,
            config.database_host,
            config.database_port,
            config.database_name
        ))
        .output()
        .expect("sqlx command failed to start");
    io::stdout().write_all(&s.stderr).unwrap();

    // Set up a new store instance with a db connection pool.
    let store = setup_store(&config).await?;

    // Start the web server via the oneshot function
    // and listen for a sender signal to shut it down.
    let handler = oneshot(store).await;

    let u = User {
        email: "test@email.com".to_string(),
        password: "password".to_string(),
    };

    register_new_user(&u).await;
    // TODO login_user();
    // TODO post_question();

    // Send any integer to shut down the server.
    let _ = handler.sender.send(1);
    Ok(())
}

async fn register_new_user(user: &User) {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3030/registration")
        .json(&user)
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();
    assert_eq!(res, "Account added".to_string());
}
