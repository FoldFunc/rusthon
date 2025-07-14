use sqlx::PgPool;
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
pub async fn register_db_handler(
    pool: &PgPool,
    email: String,
    password: String,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO users (email, password)
        VALUES ($1, $2)
        "#,
    )
    .bind(email)
    .bind(password)
    .execute(pool)
    .await?;

    Ok(())
}

