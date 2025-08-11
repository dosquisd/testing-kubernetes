use actix_web::http::StatusCode;
use actix_web::{self, App, test};

use crate::api::main::handler;
use crate::models::users::Model as UserModel;
use crate::schemas::users::{UserCreate, UserUpdate};

use crate::tests::utils::api::TestAPIParameters;
use crate::tests::utils::users::create_random_user;
use crate::tests::utils::utils::{random_email, random_int, random_string};

#[actix_web::test]
async fn test_create_user() {
    let api_params = TestAPIParameters::new().await;
    let app = test::init_service(
        App::new()
            .app_data(api_params.app_data.clone())
            .service(handler(api_params.prefix.as_str())),
    )
    .await;

    let user_create = UserCreate {
        email: random_email(),
        name: random_string(32),
        age: Some(random_int(18, 65)),
    };

    let req = test::TestRequest::post()
        .uri(&format!("{}/users/", api_params.prefix))
        .set_json(user_create.clone())
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);
    let resp: UserModel = test::read_body_json(resp).await;

    assert_eq!(resp.email, user_create.email);
    assert_eq!(resp.name, user_create.name);
    assert_eq!(resp.age, user_create.age);
    assert_eq!(resp.is_active, Some(true));
}

#[actix_web::test]
async fn test_read_user() {
    let api_params = TestAPIParameters::new().await;
    let app = test::init_service(
        App::new()
            .app_data(api_params.app_data.clone())
            .service(handler(api_params.prefix.as_str())),
    )
    .await;

    let user = create_random_user(true, Some(&api_params.db.clone().connection))
        .await
        .as_model()
        .expect("Failed to create user");

    let req = test::TestRequest::get()
        .uri(&format!("{}/users/id/{}", api_params.prefix, user.id))
        .to_request();

    let resp: UserModel = test::call_and_read_body_json(&app, req).await;

    assert_eq!(resp.id, user.id);
    assert_eq!(resp.email, user.email);
    assert_eq!(resp.name, user.name);
    assert_eq!(resp.age, user.age);
    assert_eq!(resp.is_active, Some(true));
}

#[actix_web::test]
async fn test_read_user_not_found() {
    let api_params = TestAPIParameters::new().await;
    let app = test::init_service(
        App::new()
            .app_data(api_params.app_data.clone())
            .service(handler(api_params.prefix.as_str())),
    )
    .await;

    // Assuming 999999 is an ID that does not exist
    let req = test::TestRequest::get()
        .uri(&format!("{}/users/id/999999", api_params.prefix))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn test_get_users() {
    let api_params = TestAPIParameters::new().await;
    let app = test::init_service(
        App::new()
            .app_data(api_params.app_data.clone())
            .service(handler(api_params.prefix.as_str())),
    )
    .await;

    // Create 5 users directly in DB
    for _ in 0..5 {
        let _ = create_random_user(true, Some(&api_params.db.connection)).await;
    }

    let req = test::TestRequest::get()
        .uri(&format!("{}/users/", api_params.prefix))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let users: Vec<UserModel> = test::read_body_json(resp).await;
    assert!(users.len() >= 5);
}

#[actix_web::test]
async fn test_update_user() {
    let api_params = TestAPIParameters::new().await;
    let app = test::init_service(
        App::new()
            .app_data(api_params.app_data.clone())
            .service(handler(api_params.prefix.as_str())),
    )
    .await;

    let user = create_random_user(true, Some(&api_params.db.connection))
        .await
        .as_model()
        .unwrap();

    let update = UserUpdate {
        name: Some("updated name".to_string()),
        age: Some(30),
        email: None,
        is_active: None,
    };

    let req = test::TestRequest::put()
        .uri(&format!("{}/users/id/{}", api_params.prefix, user.id))
        .set_json(update.clone())
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
    let updated: UserModel = test::read_body_json(resp).await;
    assert_eq!(updated.id, user.id);
    assert_eq!(updated.name, update.name.unwrap());
    assert_eq!(updated.age, update.age);
    // Email unchanged
    assert_eq!(updated.email, user.email);
}

#[actix_web::test]
async fn test_update_user_not_found() {
    let api_params = TestAPIParameters::new().await;
    let app = test::init_service(
        App::new()
            .app_data(api_params.app_data.clone())
            .service(handler(api_params.prefix.as_str())),
    )
    .await;

    let update = UserUpdate {
        name: Some("updated name".to_string()),
        age: Some(30),
        email: None,
        is_active: None,
    };

    let req = test::TestRequest::put()
        .uri(&format!("{}/users/id/999999", api_params.prefix))
        .set_json(update)
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn test_delete_user() {
    let api_params = TestAPIParameters::new().await;
    let app = test::init_service(
        App::new()
            .app_data(api_params.app_data.clone())
            .service(handler(api_params.prefix.as_str())),
    )
    .await;

    let user = create_random_user(true, Some(&api_params.db.connection))
        .await
        .as_model()
        .unwrap();

    let req = test::TestRequest::delete()
        .uri(&format!("{}/users/id/{}", api_params.prefix, user.id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    // Verify deleted
    let req = test::TestRequest::get()
        .uri(&format!("{}/users/id/{}", api_params.prefix, user.id))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn test_delete_user_not_found() {
    let api_params = TestAPIParameters::new().await;
    let app = test::init_service(
        App::new()
            .app_data(api_params.app_data.clone())
            .service(handler(api_params.prefix.as_str())),
    )
    .await;

    let req = test::TestRequest::delete()
        .uri(&format!("{}/users/id/999999", api_params.prefix))
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
