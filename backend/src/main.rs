use backend::Result;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::fmt;

#[tokio::main]
async fn main() -> Result<()> {
    let chrono_fmter = tracing_subscriber::fmt::time::ChronoUtc::new("%F %T%.3f".to_string());
    let format_e = fmt::format().with_timer(chrono_fmter).with_thread_ids(true);
    tracing_subscriber::fmt().event_format(format_e).init();
    let database_url = dotenvy::var("DATABASE_URL")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    //let cloned_pool = pool.clone();
    //tokio::spawn(async move {
    //    let mut interval = time::interval(Duration::from_secs(3600));
    //    loop {
    //        interval.tick().await;
    //        let _ = backend::http::get_members(&cloned_pool).await;
    //    }
    //});

    backend::http::serve(pool).await
}
