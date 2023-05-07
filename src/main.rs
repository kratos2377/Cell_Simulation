use std::{env, time::Duration, path::PathBuf};

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::post,
};

use mongodb::{options::{ClientOptions, Compressor}, Client};

use tower_http::cors::{Any, CorsLayer};

use axum_server::tls_rustls::RustlsConfig;

mod controllers;
use controllers::{auth_controller, fs_controller};

#[tokio::main]
async fn main() -> Result<() , Box<dyn std::error::Error> {
    println!("Hello, world!");
}
