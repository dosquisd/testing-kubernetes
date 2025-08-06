use super::models::prelude::Users as UserEntity;
use super::models::users;
use super::models::users::Model as UserModel;
use super::schemas::{
    api::ErrorResponse,
    users::{UserCreate, UserUpdate},
};

use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter,
};

pub struct UserService;

impl UserService {
    pub async fn get_user_by_id(
        &self,
        user_id: u16,
        connection: &DatabaseConnection,
    ) -> Result<UserModel, ErrorResponse> {
        let result = UserEntity::find_by_id(user_id as i32).one(connection).await;
        match result {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(ErrorResponse {
                message: format!("User with ID {} not found", user_id),
                status_code: 404,
            }),
            Err(e) => Err(ErrorResponse {
                message: format!("Database error: {}", e),
                status_code: 500,
            }),
        }
    }

    pub async fn get_user_by_email(
        &self,
        email: &str,
        connection: &DatabaseConnection,
    ) -> Result<UserModel, ErrorResponse> {
        let result = UserEntity::find()
            .filter(users::Column::Email.eq(email.to_owned()))
            .one(connection)
            .await;
        match result {
            Ok(Some(user)) => Ok(user),
            Ok(None) => Err(ErrorResponse {
                message: format!("User with email {} not found", email),
                status_code: 404,
            }),
            Err(e) => Err(ErrorResponse {
                message: format!("Database error: {}", e),
                status_code: 500,
            }),
        }
    }

    pub async fn get_users(
        &self,
        connection: &DatabaseConnection,
    ) -> Result<Vec<UserModel>, ErrorResponse> {
        let result = UserEntity::find().all(connection).await;
        match result {
            Ok(users) => Ok(users),
            Err(e) => Err(ErrorResponse {
                message: format!("Database error: {}", e),
                status_code: 500,
            }),
        }
    }

    pub async fn create_user(
        &self,
        user: UserCreate,
        connection: &DatabaseConnection,
    ) -> Result<UserModel, ErrorResponse> {
        let active_model: users::ActiveModel = users::ActiveModel {
            id: NotSet,
            email: Set(user.email),
            name: Set(user.name),
            age: Set(user.age),
            is_active: Set(Some(true)),
            created_at: NotSet,
            updated_at: NotSet,
        };

        // 1.
        let result: Result<UserModel, sea_orm::DbErr> = active_model.insert(connection).await;

        // 2.
        // let result: Result<sea_orm::InsertResult<users::ActiveModel>, sea_orm::DbErr> = UserEntity::insert(active_model).exec(connection).await;

        // Same output
        match result {
            Ok(user_result) => Ok(user_result),
            Err(e) => Err(ErrorResponse {
                message: format!("Database error: {}", e),
                status_code: 500,
            }),
        }
    }

    pub async fn update_user(
        &self,
        user_id: u16,
        update_user: UserUpdate,
        connection: &DatabaseConnection,
    ) -> Result<UserModel, ErrorResponse> {
        let active_model: Result<Option<UserModel>, sea_orm::DbErr> =
            UserEntity::find_by_id(user_id).one(connection).await;

        if active_model.is_err() {
            return Err(ErrorResponse {
                message: format!("Database error: {}", active_model.err().unwrap()),
                status_code: 500,
            });
        }

        let mut active_model: users::ActiveModel = match active_model.unwrap() {
            Some(model) => model.into(),
            None => {
                return Err(ErrorResponse {
                    message: format!("User with ID {} not found", user_id),
                    status_code: 404,
                });
            }
        };

        if let Some(name) = update_user.name {
            active_model.name = Set(name);
        }
        if let Some(email) = update_user.email {
            active_model.email = Set(email);
        }

        active_model.age = Set(update_user.age);
        active_model.is_active = Set(update_user.is_active);

        let active_model = active_model.update(connection).await;
        match active_model {
            Ok(updated_user) => Ok(updated_user),
            Err(e) => Err(ErrorResponse {
                message: format!("Database error: {}", e),
                status_code: 500,
            }),
        }
    }

    pub async fn delete_user(
        &self,
        user_id: u16,
        connection: &DatabaseConnection,
    ) -> Result<UserModel, ErrorResponse> {
        let user = UserEntity::find_by_id(user_id).one(connection).await;

        if let Err(e) = user {
            return Err(ErrorResponse {
                message: format!("Database error: {}", e),
                status_code: 500,
            });
        }

        let user = user.unwrap();
        if user.is_none() {
            return Err(ErrorResponse {
                message: format!("User with ID {} not found", user_id),
                status_code: 404,
            });
        }

        match user.clone().unwrap().delete(connection).await {
            Err(e) => Err(ErrorResponse {
                message: format!("Database error: {}", e),
                status_code: 500,
            }),
            Ok(_) => Ok(user.unwrap()),
        }

        // Shorthand
        // UserEntity::delete_by_id(user_id).exec(connection).await?
    }
}
