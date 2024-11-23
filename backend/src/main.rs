use sqlx::postgres::PgPoolOptions;
use tokio::time::{self, Duration};
use backend::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let database_url = dotenvy::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new().max_connections(5).connect(&database_url).await?;

    sqlx::migrate!().run(&pool).await?;

    //let cloned_pool = pool.clone();
    //tokio::spawn(async move {
    //    let mut interval = time::interval(Duration::from_secs(3600));
    //    loop {
    //        interval.tick().await;
    //        backend::http::get_members(&cloned_pool).await;
    //    }
    //});

    backend::http::serve(pool).await
}
