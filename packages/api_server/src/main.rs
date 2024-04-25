use auth::auth_config::AuthConfig;
use axum::Router;
use dotenv::dotenv;
use rabbit_trading_core::utils::error::env_var_error_to_anyhow_error;
use simple_logger::SimpleLogger;
use std::{env, str::FromStr, sync::Arc};

use crate::{
    handler::{pod::router::initialize_pod_router, state::AppState},
    utils::id_generator::auto_increment::AutoIncrementIdGenerator,
};

mod auth;
mod handler;
mod model;
mod utils;

const LOG_LEVEL: log::LevelFilter = ::log::LevelFilter::Info;
const DEFAULT_HOST: &'static str = "127.0.0.1";
const DEFAULT_PORT: &'static str = "7000";
const DEFAULT_AUTH: AuthConfig = AuthConfig::NoAuth;

#[tokio::main]
async fn main() {
    dotenv().unwrap();
    SimpleLogger::new()
        .env()
        .with_level(LOG_LEVEL)
        .init()
        .unwrap();

    let server_host = env::var("API_SERVER_HOST").unwrap_or(DEFAULT_HOST.to_owned());
    let server_port = env::var("API_SERVER_PORT").unwrap_or(DEFAULT_PORT.to_owned());
    let auth_kind = env::var("API_SERVER_AUTH")
        .map_err(env_var_error_to_anyhow_error)
        .and_then(|auth_kind| AuthConfig::from_str(&auth_kind))
        .unwrap_or(DEFAULT_AUTH);
    let bind_address = format!("{}:{}", server_host, server_port);

    if auth_kind == AuthConfig::NoAuth {
        log::warn!("NoAuth, might be risky if port was exposed to public network.");
    }
    log::warn!("bind_address = {}", bind_address);

    let app = Router::new();
    let app_state = AppState::new(Arc::new(Box::new(AutoIncrementIdGenerator::new(1i64))));
    let app = initialize_pod_router(app).with_state(app_state);
    let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
