use sea_orm::{Database, DatabaseConnection};

#[derive(Clone)]
pub struct DbPool {
    pub conn: DatabaseConnection,
}

impl DbPool {
    pub async fn connect(url: &str) -> Result<Self, sea_orm::DbErr> {
        let conn = Database::connect(url).await?;
        Ok(Self { conn })
    }
}
