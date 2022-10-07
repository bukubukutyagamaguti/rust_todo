use anyhow::Context;
use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json,
    Router
};
use serde::{Deserialize, Serialize};
use std::{
    net::SocketAddr,
    collections::HashMap,
    env,
    sync::{Arc, RwLock}
};
use thiserror::Error;

#[derive(Debug, Error)]
enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound,
}

fn main() {
    
}