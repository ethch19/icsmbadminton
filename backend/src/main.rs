use sqlx::postgres::PgPoolOptions;
use anyhow::{Context};

use backend::database;
use error::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let database_url = dotenvy::var("DATABASE_URL").map_err(Error::Env)?;

    let pool = PgPoolOptions::new().max_connections(5).connect(&database_url).await.map_err(Error::Sqlx)?;
    
    let new_admin_json = r#"
        {
            "username": "ethch19",
            "password": "testPassword123",
        }"#;
    let v: database::Admin = serde_json::from_str(new_admin_json).map_err(Error::Unknown)?;

    let admins = database::Admin::get_all(pool); 
    println!("{:?}", admins);

    Ok(())
}
