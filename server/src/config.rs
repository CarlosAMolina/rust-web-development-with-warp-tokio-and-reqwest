#![warn(clippy::all)]

use clap::Parser;
use std::env;

/// Q&A web service API
#[derive(Parser, Debug, PartialEq)]
#[clap(author, version, about, long_about = None)]
pub struct Config {
    /// URL for the postgres database
    #[clap(long, default_value = "localhost")]
    pub database_host: String,
    /// Database name
    #[clap(long, default_value = "rustwebdev")]
    pub database_name: String,
    /// Database password
    #[clap(long, default_value = "pw")]
    pub database_password: String,
    /// PORT number for the database connection
    #[clap(long, default_value = "5432")]
    pub database_port: u16,
    /// Database user
    #[clap(long, default_value = "postgres")]
    pub database_user: String,
    /// Which errors we want to log (info, warn or error)
    /// Log level handle errors
    #[clap(long, default_value = "warn")]
    pub log_level_handle_errors: String,
    /// Log level rust-web-dev
    #[clap(long, default_value = "info")]
    pub log_level_rust_web_dev: String,
    /// Log level warp
    #[clap(long, default_value = "error")]
    pub log_level_warp: String,
    /// Which PORT the web server is listening to
    #[clap(long, default_value = "3030")]
    pub web_server_port: u16,
}

impl Config {
    pub fn new() -> Result<Config, handle_errors::Error> {
        let config = Config::parse();
        if let Err(_) = env::var("BAD_WORDS_API_KEY") {
            panic!("BadWords API key not set");
        }
        if let Err(_) = env::var("PASETO_KEY") {
            panic!("PASETO_KEY not set");
        }
        let web_server_port = std::env::var("PORT")
            .ok()
            .map(|val| val.parse::<u16>())
            .unwrap_or(Ok(config.web_server_port))
            .map_err(|e| handle_errors::Error::ParseError(e))?;
        // TODO .map_err(|e| handle_errors::Error::ParseError(e))
        // TODO .expect("Cannot parse port");

        let database_user = env::var("POSTGRES_USER").unwrap_or(config.database_user.to_owned());
        // Not put credentials directly in the codebase.
        let database_password = env::var("POSTGRES_PASSWORD").unwrap();
        let database_host = env::var("POSTGRES_HOST").unwrap_or(config.database_host.to_owned());
        let database_port = env::var("POSTGRES_PORT").unwrap_or(config.database_port.to_string());
        let database_name = env::var("POSTGRES_DB").unwrap_or(config.database_name.to_owned());
        Ok(Config {
            web_server_port,
            database_user,
            database_password,
            database_host,
            database_port: database_port
                .parse::<u16>()
                .map_err(|e| handle_errors::Error::ParseError(e))?,
            database_name,
            log_level_handle_errors: config.log_level_handle_errors,
            log_level_rust_web_dev: config.log_level_rust_web_dev,
            log_level_warp: config.log_level_warp,
        })
    }
}

#[cfg(test)]
mod config_tests {
    use super::*;

    fn set_env() {
        env::set_var("BAD_WORDS_API_KEY", "yes");
        env::set_var("PASETO_KEY", "yes");
        env::set_var("POSTGRES_USER", "user");
        env::set_var("POSTGRES_PASSWORD", "pass");
        env::set_var("POSTGRES_HOST", "localhost");
        env::set_var("POSTGRES_PORT", "5432");
        env::set_var("POSTGRES_DB", "rustwebdev");
    }

    // As Rust runs test in parallel, we run two tests in the same function
    // in order to not affect each test when env variables are modified.
    #[test]
    fn unset_and_set_api_key() {
        // The env variables are not set.
        // catch_unwind: captures panics without bringing down the program.
        let result = std::panic::catch_unwind(|| Config::new());
        assert!(result.is_err());

        // Now we set the env variables.
        set_env();

        let expected = Config {
            database_host: "localhost".to_string(),
            database_name: "rustwebdev".to_string(),
            database_password: "pass".to_string(),
            database_port: 5432,
            database_user: "user".to_string(),
            log_level_handle_errors: "warn".to_string(),
            log_level_rust_web_dev: "info".to_string(),
            log_level_warp: "error".to_string(),
            web_server_port: 3030,
        };

        let config = Config::new().unwrap();

        assert_eq!(config, expected);
        // Unset all the environment variables
        // env::remove_var("");
    }
}
