use sqlx::postgres::PgPoolOptions;
use backend::database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = dotenvy::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new().max_connections(5).connect(&database_url).await?;
    
    let admins = sqlx::query_as!(database::Admin,
        r#"
        SELECT * FROM auth.admins;
        "#
    )
        .fetch_all(&pool)
        .await?;

    println!("{:?}", admins);

    Ok(())
}
