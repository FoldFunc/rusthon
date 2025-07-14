use sqlx::PgPool;
use rand::{distributions::Alphanumeric, Rng};
pub async fn login_db_handler(
    pool: &PgPool,
    email: String,
    password: String,
) -> Result<bool, sqlx::Error> {
    let user = sqlx::query!(
        r#"
        SELECT email FROM users WHERE email = $1 AND password = $2
        "#,
        email,
        password
    )
    .fetch_optional(pool) // Try to get one row, but return None if not found
    .await?;

    Ok(user.is_some()) // true = login success, false = wrong email/pass
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
) -> Result<String, sqlx::Error> {
    let token = generate_random_string(16);
    sqlx::query(
        r#"
        INSERT INTO users (email, password)
        VALUES ($1, $2)
        "#
    )
    .bind(email.clone())
    .bind(password)
    .execute(pool)
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
    Ok(token)
}

