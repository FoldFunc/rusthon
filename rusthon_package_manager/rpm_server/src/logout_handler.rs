use crate::{db::db_handlers, login_handler::CheckTokenRequest};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize)]
pub struct LogutResponse {
    message: String,
}
#[derive(Debug, Deserialize)]
pub struct LogutRequest {
    token: String,
}
pub async fn logout_handler_main(
    State(pool): State<PgPool>,
    Json(payload): Json<LogutRequest>,
) -> impl IntoResponse {
    println!("Function logout called");
    match db_handlers::logout_db_handler(&pool, payload.token).await {
        Ok(_) => {
            println!("Logged out");
            (StatusCode::OK, Json(LogutResponse{
                message: "Logged out succesfully.".to_string(),
            }))
                .into_response()
        }
        Err(e) => {
            eprintln!("Error while logging out: {}", e); 
            (StatusCode::NOT_FOUND, Json(LogutResponse{
                message: "Error while processing".to_string(),
            }))
            .into_response()
        }
    }
}
