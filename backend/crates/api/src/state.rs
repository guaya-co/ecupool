use ecupool_config::Config;
use ecupool_db::{DbPool, connect_pool};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: DbPool,
}

pub async fn init_app_state(config: Config) -> AppState {
    let db_pool = connect_pool(config.database)
        .await
        .expect("Could not connect to database!");

    AppState { db_pool }
}
