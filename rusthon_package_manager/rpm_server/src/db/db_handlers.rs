use sqlx::PgPool;
use rand::{distributions::Alphanumeric, Rng};
use chrono::{DateTime, Duration, Utc, TimeZone};
pub async fn logout_db_handler(
    pool: &PgPool,
    token: String,
) -> Result<(), sqlx::Error> {
    let token_str = token.trim();
    sqlx::query!(
        r#"
        DELETE FROM tokens WHERE token = $1
        "#,
        token_str
    )
    .execute(pool)
    .await?;
    Ok(())
}
pub async fn check_token_db_handler(
    pool: &PgPool,
    token: String,
) -> Result<bool, sqlx::Error> {
    let token_str = token.trim();
    let result = sqlx::query!(
        r#"
        SELECT time as "time: chrono::NaiveDateTime"
        FROM tokens
        WHERE token = $1
        "#,
        token_str
    )
    .fetch_optional(pool)
    .await?;
    if let Some(row) = result {
        if let Some(token_time) = row.time {
            let token_time_utc = Utc.from_utc_datetime(&token_time);
            let now = Utc::now();
            let age = now.signed_duration_since(token_time_utc);
            println!("age: {:?}", age);
            println!("day: {:?}", Duration::days(1));
            return Ok(age < Duration::days(1));
        }
    }

    Err(sqlx::Error::RowNotFound)
}
pub async fn login_db_handler(
    pool: &PgPool,
    email: String,
    password: String,
) -> Result<String, sqlx::Error> {
    let token = generate_random_string(64);
    sqlx::query!(
        r#"
        SELECT email FROM users WHERE email = $1 AND password = $2
        "#,
        email,
        password
    )
    .fetch_optional(pool) // Try to get one row, but return None if not found
    .await?;
    sqlx::query(
        r#"
        INSERT INTO tokens (email, token)
        VALUES ($1, $2)
        "#
    )
    .bind(email.clone())
    .bind(&token)
    .execute(pool)
    .await?;
    Ok(token) // true = login success, false = wrong email/pass
}
fn generate_random_string(len: usize) -> String {
    let rand_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();
    return  rand_string;
}
pub async fn register_db_handler(
    pool: &PgPool,
    email: String,
    password: String,
) -> Result<bool, sqlx::Error> {
    let user = sqlx::query!(
        r#"
        INSERT INTO users (email, password)
        VALUES ($1, $2)
        "#,
        email,
        password
    )
    .fetch_optional(pool) // Try to get one row, but return None if not found
    .await?;
    Ok(user.is_some())
}

