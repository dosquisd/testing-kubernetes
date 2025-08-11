use crate::core::database::DatabaseService;
use crate::crud::UserService;
use crate::models::users::Model as UserModel;
use crate::schemas::users::{UserCreate, UserUpdate};
use crate::tests::utils::users::create_random_user;
use crate::tests::utils::utils::{random_email, random_int, random_string};

// Helper to init DB + service
async fn setup() -> (DatabaseService, UserService) {
    let db = DatabaseService::init(None).await;
    (db, UserService {})
}

#[tokio::test]
async fn test_create_user() {
    let (db, user_service) = setup().await;
    let user_create = UserCreate {
        email: random_email(),
        name: random_string(16),
        age: Some(random_int(18, 65)),
    };
    let user = user_service
        .create_user(user_create.clone(), &db.connection)
        .await
        .ok()
        .expect("failed to create user");

    assert_eq!(user.email, user_create.email);
    assert_eq!(user.name, user_create.name);
    assert_eq!(user.age, user_create.age);
    assert_eq!(user.is_active, Some(true));
}

#[tokio::test]
async fn test_get_user() {
    let (db, user_service) = setup().await;
    let user = create_random_user(true, Some(&db.connection))
        .await
        .as_model()
        .unwrap();
    let retrieved = user_service
        .get_user_by_id(user.id as u16, &db.connection)
        .await
        .ok()
        .expect("user should exist");

    assert_eq!(retrieved.id, user.id);
    assert_eq!(retrieved.email, user.email);
    assert_eq!(retrieved.name, user.name);
    assert_eq!(retrieved.age, user.age);
    assert_eq!(retrieved.is_active, user.is_active);
}

#[tokio::test]
async fn test_get_user_not_found() {
    let (db, user_service) = setup().await;
    let missing_id: u16 = 65535; // High id unlikely to exist in test DB
    let result = user_service
        .get_user_by_id(missing_id, &db.connection)
        .await;
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.status_code, 404);
    assert!(err.message.contains("not found"));
}

#[tokio::test]
async fn test_get_user_by_email() {
    let (db, user_service) = setup().await;
    let user = create_random_user(true, Some(&db.connection))
        .await
        .as_model()
        .unwrap();
    let retrieved = user_service
        .get_user_by_email(user.email.as_str(), &db.connection)
        .await
        .ok()
        .expect("user should exist");
    assert_eq!(retrieved.id, user.id);
    assert_eq!(retrieved.email, user.email);
}

#[tokio::test]
async fn test_get_user_by_email_not_found() {
    let (db, user_service) = setup().await;
    let email = random_email();
    let result = user_service
        .get_user_by_email(email.as_str(), &db.connection)
        .await;
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.status_code, 404);
    assert!(err.message.contains("not found"));
}

#[tokio::test]
async fn test_get_users_and_search() {
    let (db, user_service) = setup().await;
    // Create multiple users
    let mut created: Vec<UserModel> = Vec::new();
    for _ in 0..5 {
        let model = create_random_user(true, Some(&db.connection))
            .await
            .as_model()
            .unwrap();
        created.push(model);
    }

    // List users
    let users = user_service
        .get_users(&db.connection, 1, 10, None)
        .await
        .ok()
        .expect("should list users");
    assert!(users.len() >= 5);

    // Search using substring of first user's name
    let needle = created[0].name.chars().take(5).collect::<String>();
    let searched = user_service
        .get_users(&db.connection, 1, 10, Some(needle.clone()))
        .await
        .ok()
        .expect("search should work");
    assert!(!searched.is_empty());
    assert!(
        searched
            .iter()
            .all(|u| u.name.contains(&needle) || u.email.contains(&needle))
    );
}

#[tokio::test]
async fn test_update_user() {
    let (db, user_service) = setup().await;
    let user = create_random_user(true, Some(&db.connection))
        .await
        .as_model()
        .unwrap();
    let new_name = format!("updated_{}", random_string(8));
    let new_email = random_email();
    let update = UserUpdate {
        name: Some(new_name.clone()),
        email: Some(new_email.clone()),
        age: Some(30),
        is_active: Some(true),
    };
    let updated = user_service
        .update_user(user.id as u16, update, &db.connection)
        .await
        .ok()
        .expect("update should succeed");
    assert_eq!(updated.id, user.id);
    assert_eq!(updated.name, new_name);
    assert_eq!(updated.email, new_email);
    assert_eq!(updated.age, Some(30));
}

#[tokio::test]
async fn test_update_user_not_found() {
    let (db, user_service) = setup().await;
    let update = UserUpdate {
        name: Some(random_string(10)),
        email: Some(random_email()),
        age: Some(40),
        is_active: Some(true),
    };
    let result = user_service
        .update_user(65535, update, &db.connection)
        .await;
    assert!(result.is_err());
    let err = result.err().unwrap();
    assert_eq!(err.status_code, 404);
}
