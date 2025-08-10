use super::routes::handler_users;
use actix_web::{Responder, Scope, get, http::Error, web};
use serde::Serialize;

// For healthchecks
use crate::core::cache::REDIS_SERVICE;
use crate::core::config::SETTINGS;
use crate::core::database::{DatabaseParams, DatabaseService};

#[derive(Serialize)]
struct Root {
    message: String,
    version: String,
    docs: String,
    redoc: String,
}

#[derive(Serialize)]
struct Health {
    status: String,
    database: String,
    cache: String,
    questdb: String,
}

#[get("/")]
pub async fn root() -> Result<impl Responder, Error> {
    Ok(web::Json(Root {
        message: "User Management API".to_owned(),
        version: "2.0.0".to_owned(),
        docs: "/docs".to_owned(),
        redoc: "/redoc".to_owned(),
    }))
}

#[get("/health")]
pub async fn health_check() -> Result<impl Responder, Error> {
    let mut health_status = Health {
        status: "healthy".to_owned(),
        database: "unknown".to_owned(),
        cache: "unknown".to_owned(),
        questdb: "unknown".to_owned(),
    };

    // Check database connection
    /* Another way to check the database connection
    let exec = db
    .connection
    .execute(Statement::from_string(
        DatabaseBackend::Postgres,
        "SELECT 1",
    ))
    .await;
    */
    let db = DatabaseService::init(None).await;
    match db.connection.ping().await {
        Ok(_) => health_status.database = "healthy".to_owned(),
        Err(_) => health_status.database = "unhealthy".to_owned(),
    }

    // Check Redis cache connection
    match REDIS_SERVICE.ping() {
        Ok(_) => health_status.cache = "healthy".to_owned(),
        Err(_) => health_status.cache = "unhealthy".to_owned(),
    }

    // Check QuestDB connection
    let questdb_conn = DatabaseService::init(Some(DatabaseParams {
        protocol: Some("postgres".to_string()),
        host: SETTINGS.questdb_host.clone(),
        port: SETTINGS.questdb_pg_port.clone(),
        db: SETTINGS.questdb_db.clone(),
        user: SETTINGS.questdb_user.clone().ok(),
        password: SETTINGS.questdb_password.clone().ok(),
    }))
    .await;

    match questdb_conn.connection.ping().await {
        Ok(_) => health_status.questdb = "healthy".to_owned(),
        Err(_) => health_status.questdb = "unhealthy".to_owned(),
    };

    Ok(web::Json(health_status))
}

pub fn handler(prefix: &str) -> Scope {
    web::scope(prefix)
        .service(root)
        .service(health_check)
        .service(handler_users())
}
