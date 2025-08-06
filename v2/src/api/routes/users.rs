use crate::core::database::DatabaseService;
use crate::crud::UserService;
use crate::schemas::users::{UserCreate, UserUpdate};
use actix_web::{HttpResponse, Responder, delete, get, post, put, web};

#[get("/")]
async fn get_users(db: web::Data<DatabaseService>) -> Result<impl Responder, actix_web::Error> {
    println!("Fetching all users");
    let user_service = UserService {};
    match user_service.get_users(&db.connection).await {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        Err(e) => {
            log::error!("Error fetching users: {}", e.message);
            Ok(HttpResponse::InternalServerError().json(e))
        }
    }
}

#[get("/id/{id}")]
async fn get_user(
    db: web::Data<DatabaseService>,
    id: web::Path<u16>,
) -> Result<impl Responder, actix_web::Error> {
    println!("Fetching user with ID: {}", id);
    let user_service = UserService {};
    match user_service
        .get_user_by_id(id.into_inner(), &db.connection)
        .await
    {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => {
            log::error!("Error fetching user: {}", e.message);
            Ok(HttpResponse::build(e.get_status_code()).json(e))
        }
    }
}

#[get("/email/{email}")]
async fn get_user_by_email(
    db: web::Data<DatabaseService>,
    email: web::Path<String>,
) -> Result<impl Responder, actix_web::Error> {
    println!("Fetching user with email: {}", email);
    let user_service = UserService {};
    match user_service
        .get_user_by_email(email.into_inner().as_str(), &db.connection)
        .await
    {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => {
            log::error!("Error fetching user by email: {}", e.message);
            Ok(HttpResponse::build(e.get_status_code()).json(e))
        }
    }
}

#[post("/")]
async fn create_user(
    db: web::Data<DatabaseService>,
    user: web::Json<UserCreate>,
) -> Result<impl Responder, actix_web::Error> {
    let user = user.into_inner();
    println!("Creating user: {:?}", user);
    let user_service = UserService {};
    match user_service.create_user(user, &db.connection).await {
        Ok(id) => Ok(HttpResponse::Created().json(id)),
        Err(e) => {
            log::error!("Error creating user: {}", e.message);
            Ok(HttpResponse::InternalServerError().json(e))
        }
    }
}

#[put("/id/{id}")]
async fn update_user(
    db: web::Data<DatabaseService>,
    id: web::Path<u16>,
    user: web::Json<UserUpdate>,
) -> Result<impl Responder, actix_web::Error> {
    let user = user.into_inner();
    let user_id = id.into_inner();
    println!("Updating user with ID: {} - {:?}", user_id, user);
    let user_service = UserService {};
    match user_service
        .update_user(user_id, user, &db.connection)
        .await
    {
        Ok(updated_user) => Ok(HttpResponse::Ok().json(updated_user)),
        Err(e) => {
            log::error!("Error updating user: {}", e.message);
            Ok(HttpResponse::build(e.get_status_code()).json(e))
        }
    }
}

#[delete("/id/{id}")]
async fn delete_user(
    db: web::Data<DatabaseService>,
    id: web::Path<u16>,
) -> Result<impl Responder, actix_web::Error> {
    let user_id = id.into_inner();
    println!("Deleting user with ID: {}", user_id);
    let user_service = UserService {};
    match user_service.delete_user(user_id, &db.connection).await {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => {
            log::error!("Error deleting user: {}", e.message);
            Ok(HttpResponse::build(e.get_status_code()).json(e))
        }
    }
}

pub fn handler_users() -> actix_web::Scope {
    actix_web::web::scope("/users")
        .service(get_users)
        .service(get_user)
        .service(get_user_by_email)
        .service(create_user)
        .service(update_user)
        .service(delete_user)
}
    