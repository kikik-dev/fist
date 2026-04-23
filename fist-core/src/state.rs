use std::sync::Arc;
use crate::db::DbPool;

pub struct AppState {
    pub db: DbPool,
}

pub type SharedState = Arc<AppState>;
