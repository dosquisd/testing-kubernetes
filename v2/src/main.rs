use actix_web::{App, HttpServer};
use v2::api::main::controller;

// use v2::core::database::DatabaseService;
// use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};
// use tokio;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new().service(controller("/api/v2")))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
    /*
    let now = std::time::SystemTime::now();
    let db_service = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let db = DatabaseService::init().await;

            let exec = db
                .connection
                .execute(Statement::from_string(
                    DatabaseBackend::Postgres,
                    "SELECT 1",
                ))
                .await;
            println!(
                "Execute result: {} --- {} seconds",
                exec.is_ok(),
                now.elapsed().unwrap().as_micros() as f32 * 0.000001
            );

            db
        });

    println!(
        "Database service initialized: {:?} --- {} seconds",
        db_service.connection,
        now.elapsed().unwrap().as_micros() as f32 * 0.000001
    );

    let now = std::time::SystemTime::now();
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            // let ping_result = db_service.connection.ping().await;
            // println!(
            //     "Ping result: {:?} --- {} seconds",
            //     ping_result.is_ok(),
            //     now.elapsed().unwrap().as_micros() as f32 * 0.000001
            // );
            let exec = db_service
                .connection
                .execute(Statement::from_string(
                    DatabaseBackend::Postgres,
                    "SELECT 1",
                ))
                .await;
            println!(
                "Execute result: {} --- {} seconds",
                exec.is_ok(),
                now.elapsed().unwrap().as_micros() as f32 * 0.000001
            );
        });
    */
}
