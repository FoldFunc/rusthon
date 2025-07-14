use crate::db::db_handlers;
use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    message: String,
    token: String,
}

pub async fn register_handler_main(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    println!("Function register_handler_main called");

    match db_handlers::register_db_handler(&pool, payload.email, payload.password).await {
        Ok(token) => Json(RegisterResponse {
            message: "User registered successfully.".to_string(),
            token: token,
        }),
        Err(e) => {
            eprintln!("Failed to add user: {}", e);
            Json(RegisterResponse {
                message: format!("Failed: {}", e),
                token: "".to_string(),
            })
        }
    }
}

