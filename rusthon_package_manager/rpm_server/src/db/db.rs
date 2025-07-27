// src/db.rs

use once_cell::sync::OnceCell;
use sqlx::PgPool;
use std::env;
use dotenvy::dotenv;

static DB_POOL: OnceCell<PgPool> = OnceCell::new();

pub async fn init_db() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&db_url)
        .await
        .expect("Failed to connect to the database");

    // Create tables on startup
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            email TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL
        );
        "#
    )
    .execute(&pool)
    .await
    .expect("Failed to create users table");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tokens (
            id SERIAL PRIMARY KEY,
            email TEXT NOT NULL UNIQUE,
            token TEXT NOT NULL UNIQUE,
            time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        "#
    )
    .execute(&pool)
    .await
    .expect("Failed to create users table");

    DB_POOL.set(pool).expect("Database pool already initialized");
}

pub fn get_db() -> &'static PgPool {
    DB_POOL.get().expect("Database not initialized. Call init_db() first.")
}

