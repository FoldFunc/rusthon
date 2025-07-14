use sqlx::PgPool;
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

