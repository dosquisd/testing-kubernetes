use crate::core::database::DatabaseService;
use actix_web::{self, web};

pub struct TestAPIParameters {
    pub prefix: String,
    pub db: DatabaseService,
    pub app_data: web::Data<DatabaseService>,
}

impl TestAPIParameters {
    pub async fn new() -> TestAPIParameters {
        let db = DatabaseService::init(None).await;
        TestAPIParameters {
            prefix: "/api/v2".to_string(),
            db: db.clone(),
            app_data: web::Data::new(db),
        }
    }
}
