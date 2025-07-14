use crate::db::db_handlers;
use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    message: String,
}
pub async fn login_handler_main(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    println!("Function login_handler_main called");
    match db_handlers::login_db_handler(&pool, payload.email, payload.password).await {
        Ok(_) => Json(LoginResponse {
            message: "User logged in succesfully".to_string(),
        }),
        Err(e) => {
            eprintln!("Failed to login: {}", e);
            Json(LoginResponse { message: format!("Failed: {}", e).to_string() })
        }
    }
}
