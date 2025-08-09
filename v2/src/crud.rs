use super::models::prelude::Users as UserEntity;
use super::models::users::{self, Model as UserModel};
use super::schemas::{
    api::ErrorResponse,
    users::{UserCreate, UserUpdate},
};

use sea_orm::ActiveValue::{NotSet, Set};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter,
};

use crate::core::cache::REDIS_SERVICE;
use serde_json;

pub struct UserService;

impl UserService {
    pub async fn get_user_by_id(
        &self,
        user_id: u16,
        connection: &DatabaseConnection,
    ) -> Result<UserModel, ErrorResponse> {
        // Return cached response
        let cache_key = format!("user:id:{}", user_id);
        let cached_user = REDIS_SERVICE.get(cache_key.as_str());
        if let Some(user) = cached_user {
            match serde_json::from_str::<UserModel>(user.as_str()) {
                Ok(user_model) => return Ok(user_model),
                Err(_) => {
                    log::warn!("Failed to deserialize cached user with ID {}", user_id);
                }
            }
        }

        let result = UserEntity::find_by_id(user_id as i32).one(connection).await;
        match result {
            Ok(Some(user)) => {
                // Try to cache the response
                match serde_json::to_string(&user) {
                    Ok(cached_response) => {
                        let set_result =
                            REDIS_SERVICE.set(&cache_key.as_str(), cached_response.as_str());
                        if let Err(e) = set_result {
                            log::warn!("Failed to cache response with ID {user_id} -- Error: {e} ")
                        }
                    }
                    Err(e) => {
                        log::warn!(
                            "Failed to serialize cached response with ID {user_id} -- Error: {e}"
                        );
                    }
                };
                Ok(user)
            }
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
        let cache_key = format!("user:email:{}", email);
        let cached_user = REDIS_SERVICE.get(cache_key.as_str());
        if let Some(user) = cached_user {
            match serde_json::from_str::<UserModel>(user.as_str()) {
                Ok(user_model) => return Ok(user_model),
                Err(_) => {
                    log::warn!("Failed to deserialize cached user with email {}", email);
                }
            }
        }

        let result = UserEntity::find()
            .filter(users::Column::Email.eq(email.to_owned()))
            .one(connection)
            .await;
        match result {
            Ok(Some(user)) => {
                match serde_json::to_string(&user) {
                    Ok(cached_response) => {
                        let set_result =
                            REDIS_SERVICE.set(&cache_key.as_str(), cached_response.as_str());
                        if let Err(e) = set_result {
                            log::warn!("Failed to cache response with email {email} -- Error: {e} ")
                        }
                    }
                    Err(e) => {
                        log::warn!(
                            "Failed to serialize cached response with email {email} -- Error: {e}"
                        );
                    }
                };
                Ok(user)
            }
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
        page: usize,
        limit: usize,
        search: Option<String>,
    ) -> Result<Vec<UserModel>, ErrorResponse> {
        let cache_key = format!(
            "users:page:{page}:limit:{limit}:search:{}",
            search.clone().unwrap_or("none".to_string())
        );
        let cached_response = REDIS_SERVICE.get(&cache_key.as_str());
        if let Some(response) = cached_response {
            match serde_json::from_str::<Vec<UserModel>>(response.as_str()) {
                Ok(users) => return Ok(users),
                _ => {
                    log::warn!("Failed to deserialize cached users")
                }
            }
        }

        // Before (without pagination)
        // let result = UserEntity::find().all(connection).await;

        // After (with pagination)
        let offset = (page - 1) * limit;
        let mut query = UserEntity::find();
        sea_orm::QueryTrait::query(&mut query)
            .offset(offset as u64)
            .limit(limit as u64);
        if let Some(search_term) = search {
            query = query.filter(
                users::Column::Name
                    .contains(search_term.as_str())
                    .or(users::Column::Email.contains(search_term.as_str())),
            );
        }
        let result = query.all(connection).await;

        match result {
            Ok(users) => {
                match serde_json::to_string(&users) {
                    Ok(cached_response) => {
                        let set_result =
                            REDIS_SERVICE.set(&cache_key.as_str(), cached_response.as_str());
                        if let Err(e) = set_result {
                            log::warn!("Failed to cache response -- Error: {e} ")
                        }
                    }
                    Err(e) => {
                        log::warn!("Failed to serialize cached response -- Error: {e}");
                    }
                };
                Ok(users)
            }
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

        // Invalidate cached response of all users
        REDIS_SERVICE.delete_pattern("users:*");

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

        // Invalidate cache responses
        REDIS_SERVICE.delete_pattern("users:*");
        let redis_result = REDIS_SERVICE.delete(&format!("user:id:{user_id}"));
        if let Err(e) = redis_result {
            log::warn!("Failed to delete cached user with ID {user_id} -- Error: {e}");
        }
        let redis_result =
            REDIS_SERVICE.delete(&format!("user:email:{}", active_model.email.as_ref()));
        if let Err(e) = redis_result {
            log::warn!(
                "Failed to delete cached user with email {} -- Error: {e}",
                active_model.email.as_ref()
            );
        }

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
            Ok(_) => {
                // Invalidate cache responses
                REDIS_SERVICE.delete_pattern("users:*");
                let redis_result = REDIS_SERVICE.delete(&format!("user:id:{user_id}"));
                if let Err(e) = redis_result {
                    log::warn!("Failed to delete cached user with ID {user_id} -- Error: {e}");
                }
                let redis_result =
                    REDIS_SERVICE.delete(&format!("user:email:{}", user.clone().unwrap().email));
                if let Err(e) = redis_result {
                    log::warn!(
                        "Failed to delete cached user with email {} -- Error: {e}",
                        user.clone().unwrap().email
                    );
                }

                Ok(user.unwrap())
            }
        }

        // Shorthand
        // UserEntity::delete_by_id(user_id).exec(connection).await?
    }
}
