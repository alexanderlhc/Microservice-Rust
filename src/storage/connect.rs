use sqlx::MySqlPool;
use thiserror::Error;

use crate::settings::Settings;

pub async fn get_pool(settings: &Settings) -> Result<MySqlPool, DbError> {
    let pool = MySqlPool::connect(&settings.db.connection_url).await?;
    Ok(pool)
}

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Database error: {0}")]
    SqlxError(#[from] sqlx::Error),
}
