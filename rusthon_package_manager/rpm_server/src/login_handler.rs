use crate::db::db_handlers;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize)]
pub struct CheckTokenResponse {
    message: String,
}

#[derive(Debug, Deserialize)]
pub struct CheckTokenRequest {
    token: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    message: String,
    token: String,
}

pub async fn login_handler_check_token(
    State(pool): State<PgPool>,
    Json(payload): Json<CheckTokenRequest>,
) -> impl IntoResponse {
    println!("Function check token called");
    match db_handlers::check_token_db_handler(&pool, payload.token).await {
        Ok(valid) => {
            if valid {
                println!("Token check passed");
                (StatusCode::OK, Json(CheckTokenResponse {
                    message: "Token correct, logged in".to_string(),
                }))
                    .into_response()
            } else {
                println!("Token is expired or invalid");
                (StatusCode::UNAUTHORIZED, Json(CheckTokenResponse {
                    message: "Token is expired or invalid".to_string(),
                }))
                    .into_response()
            }
        }
        Err(e) => {
            eprintln!("Error checking token: {}", e);
            (StatusCode::UNAUTHORIZED, Json(CheckTokenResponse {
                message: format!("Error while checking token: {}", e),
            }))
                .into_response()
        }
    }
}

pub async fn login_handler_main(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    println!("Function login_handler_main called");

    match db_handlers::login_db_handler(&pool, payload.email, payload.password).await {
        Ok(token) => {
            (StatusCode::OK, Json(LoginResponse {
                message: "User logged in successfully".to_string(),
                token,
            }))
                .into_response()
        }
        Err(e) => {
            eprintln!("Failed to login: {}", e);
            (StatusCode::UNAUTHORIZED, Json(LoginResponse {
                message: "Invalid email or password".to_string(),
                token: "".to_string(),
            }))
                .into_response()
        }
    }
}

