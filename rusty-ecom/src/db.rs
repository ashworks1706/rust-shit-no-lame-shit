use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    // create connection pool 
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://ecom.db")
        .await?;
    // run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;
    Ok(pool)
    // return the pool 
}
