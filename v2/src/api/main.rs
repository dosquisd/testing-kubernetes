use actix_web::{Responder, Scope, get, http::Error, web};
use serde::Serialize;

#[derive(Serialize)]
struct Root {
    message: String,
    version: String,
    docs: String,
    redoc: String,
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

pub fn controller(prefix: &str) -> Scope {
    web::scope(prefix).service(root)
}
