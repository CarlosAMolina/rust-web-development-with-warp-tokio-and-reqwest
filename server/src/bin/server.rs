use rust_web_dev::config; // rust_web_dev is the project name.
use rust_web_dev::{run, setup_store};

use dotenv;

#[tokio::main]
async fn main() -> Result<(), handle_errors::Error> {
    // Initialize the .env file via the dotenv crate.
    dotenv::dotenv().ok();
    let config = config::Config::new().expect("Config can't be set");
    let store = setup_store(&config).await?;
    tracing::info!("Q&A service build ID {}", env!("RUST_WEB_DEV_VERSION"));
    run(config, store).await;
    Ok(())
}
