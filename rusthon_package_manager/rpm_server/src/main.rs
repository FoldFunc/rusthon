mod register_handler;
mod logout_handler;
mod login_handler;
mod db;
use axum::{
    routing::post,
    Router,
};
use std::net::SocketAddr;
#[tokio::main]
async fn main() {
    db::db::init_db().await;
    let pool = db::db::get_db().clone();
    let server = Router::new()
        .route("/api/register", post(register_handler::register_handler_main))
        .route("/api/login", post(login_handler::login_handler_main))
        .route("/api/valid_token", post(login_handler::login_handler_check_token))
        .route("/api/logout", post(logout_handler::logout_handler_main))
        .with_state(pool);
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server runnin on: {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), server)
        .await
        .unwrap();
}
