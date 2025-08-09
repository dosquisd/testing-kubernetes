use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use v2::api::main::handler;

use actix_web::middleware::{Logger, from_fn};
use env_logger::Env;
use v2::api::middlewares::logs::dispatch_logs;

// use v2::core::database::DatabaseService;
// use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};
// use tokio;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let prefix = "/api/v2";
    let db = v2::core::database::DatabaseService::init().await;
    let app_data = web::Data::new(db);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin() // Just for development
            .supports_credentials();
        App::new()
            .app_data(app_data.clone())
            .service(handler(prefix))
            .wrap(cors)
            // Documentation: https://actix.rs/docs/middleware
            // Logger::new("  %a %t "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T")
            .wrap(Logger::default())
            .wrap(from_fn(dispatch_logs))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await  
}
