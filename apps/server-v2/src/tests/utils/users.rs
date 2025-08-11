use super::utils::{random_email, random_int, random_string};
use crate::crud::UserService;
use crate::models::users::Model as UserModel;
use crate::schemas::users::UserCreate;
use sea_orm::DatabaseConnection;

// Unify the two possible return types in an enum.
pub enum RandomUser {
    Model(UserModel),
    Create(UserCreate),
}

impl RandomUser {
    pub fn as_model(&self) -> Option<UserModel> {
        if let RandomUser::Model(m) = self { Some(m.to_owned()) } else { None }
    }
    pub fn as_create(&self) -> Option<UserCreate> {
        if let RandomUser::Create(c) = self { Some(c.to_owned()) } else { None }
    }
}

/// Creates a random user. If submit_db is true, persists and returns the UserModel,
/// otherwise returns the in-memory UserCreate.
pub async fn create_random_user(
    submit_db: bool,
    db: Option<&DatabaseConnection>,
) -> RandomUser {
    let user_create = UserCreate {
        email: random_email(),
        name: random_string(32),
        age: Some(random_int(18, 65)),
    };

    if submit_db {
        let db = db.expect("Database connection is required when submitting to DB");
        let user_service = UserService {};
        let model = user_service
            .create_user(user_create, db)
            .await
            .ok()
            .expect("Failed to create user in DB");
        RandomUser::Model(model)
    } else {
        RandomUser::Create(user_create)
    }
}
