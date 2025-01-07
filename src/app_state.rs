use std::sync::Arc;

use sqlx::MySqlPool;

use crate::{settings::Settings, storage::connect::get_pool};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: MySqlPool,
}

impl AppState {
    pub async fn new(settings: Settings) -> Arc<Self> {
        let db_pool = get_pool(&settings).await.unwrap();
        Arc::new(Self { db_pool })
    }
}
