use ecupool_config::DatabaseConfig;
use eyre::WrapErr;
pub use sqlx::postgres::PgPool as DbPool;
use sqlx::postgres::PgPoolOptions;

pub async fn connect_pool(config: DatabaseConfig) -> eyre::Result<DbPool> {
    // TODO: fix this
    let pool = PgPoolOptions::new()
        .connect(config.url.as_str())
        .await
        .wrap_err("Failed to connect to database")?;

    Ok(pool)
}
