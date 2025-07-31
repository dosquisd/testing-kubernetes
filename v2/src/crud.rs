use super::core::database::DatabaseService;

use super::models::prelude::Users as UserEntity;
use super::models::users;
use super::models::users::Model as UserModel;
use super::schemas::users::{UserCreate, UserUpdate};

use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter};

pub struct UserService {
    db: DatabaseService,
}

impl UserService {
    pub async fn get_user_by_id(&self, user_id: u16) -> Result<UserModel, String> {
        let result = UserEntity::find_by_id(user_id as i32)
            .one(&self.db.connection)
            .await;
        match result {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(format!("User with ID {} not found", user_id)),
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<UserModel, String> {
        let result = UserEntity::find()
            .filter(users::Column::Email.eq(email.to_owned()))
            .one(&self.db.connection)
            .await;
        match result {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(format!("User with email {} not found", email)),
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }

    pub async fn get_users(&self) -> Result<Vec<UserModel>, String> {
        let result = UserEntity::find().all(&self.db.connection).await;
        match result {
            Ok(users) => Ok(users),
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }

    pub async fn create_user(&self, user: UserCreate) -> Result<i32, String> {
        let active_model: users::ActiveModel = users::ActiveModel {
            id: Set(1),
            email: Set(user.email),
            name: Set(user.name),
            age: Set(user.age),
            is_active: Set(Some(true)),
            created_at: NotSet,
            updated_at: NotSet,
        };

        // 1.
        // let result = active_model.insert(&self.db.connection).await;

        // 2.
        let result = UserEntity::insert(active_model)
            .exec(&self.db.connection)
            .await;

        // Same output
        match result {
            Ok(user_result) => Ok(user_result.last_insert_id),
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }

    pub async fn update_user(
        &self,
        user_id: i32,
        update_user: UserUpdate,
    ) -> Result<UserModel, String> {
        let active_model: Result<Option<UserModel>, sea_orm::DbErr> =
            UserEntity::find_by_id(user_id)
                .one(&self.db.connection)
                .await;

        if active_model.is_err() {
            return Err(format!("Database error: {}", active_model.err().unwrap()));
        }

        let mut active_model: users::ActiveModel = match active_model.unwrap() {
            Some(model) => model.into(),
            None => return Err(format!("User with ID {} not found", user_id)),
        };

        if let Some(name) = update_user.name {
            active_model.name = Set(name);
        }
        if let Some(email) = update_user.email {
            active_model.email = Set(email);
        }

        active_model.age = Set(update_user.age);
        active_model.is_active = Set(update_user.is_active);

        let active_model = active_model.update(&self.db.connection).await;
        match active_model {
            Ok(updated_user) => Ok(updated_user),
            Err(e) => Err(format!("Database error: {}", e)),
        }
    }

    pub async fn delete_user(&self, user_id: i32) -> Result<UserModel, String> {
        let user = UserEntity::find_by_id(user_id)
            .one(&self.db.connection)
            .await;

        if let Err(e) = user {
            return Err(format!("Database error: {}", e));
        }

        let user = user.unwrap();
        if user.is_none() {
            return Err(format!("User with ID {} not found", user_id));
        }

        match user.clone().unwrap().delete(&self.db.connection).await {
            Err(e) => Err(format!("Database error: {}", e)),
            Ok(_) => Ok(user.unwrap())
        }

        // Shorthand
        // UserEntity::delete_by_id(user_id).exec(&self.db.connection).await?
    }
}
