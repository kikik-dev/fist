use sea_orm::{Set, EntityTrait};
use crate::model::user::{ActiveModel, Entity as User};
use fist_core::db::DbPool;
use fist_core::error::FistError;

pub struct UserService { pub db: DbPool }

impl UserService {
    pub async fn create_user(&self, username: &str, email: &str) -> Result<(), FistError> {
        let new_user = ActiveModel {
            username: Set(username.to_owned()),
            email: Set(email.to_owned()),
            ..Default::default()
        };
        
        User::insert(new_user)
            .exec(&self.db.conn)
            .await
            .map_err(|e| FistError::Internal(e.to_string()))?;
            
        Ok(())
    }
}
