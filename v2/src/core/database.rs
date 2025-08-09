use super::config::SETTINGS;
use sea_orm::{ConnectOptions, DatabaseConnection};
use std::time::Duration;

pub struct DatabaseService {
    pub connection: DatabaseConnection,
}

impl DatabaseService {
    fn _create_database_uri(protocol: Option<&str>) -> String {
        let credentials = match(&SETTINGS.postgres_user, &SETTINGS.postgres_password) {
            (Ok(user), Ok(password)) => format!("{}:{}@", user, password),
            (Ok(user), _) => format!("{}@", user),
            (_, Ok(password)) => format!(":{}@", password),
            (_, _) => String::new(),
        };

        format!(
            "{}://{}{}:{}/{}",
            protocol.unwrap_or("postgres"),
            credentials,
            SETTINGS.postgres_host,
            SETTINGS.postgres_port,
            SETTINGS.postgres_db
        )
    }

    pub async fn init() -> Self {
        let uri = DatabaseService::_create_database_uri(Some("postgres"));
        let mut options = ConnectOptions::new(uri);

        // Configure connection timeouts and pool settings
        options
            .connect_timeout(Duration::from_secs(8))   // Connection timeout
            .acquire_timeout(Duration::from_secs(8))   // Pool acquire timeout
            .idle_timeout(Duration::from_secs(600))    // Connection idle timeout (10 minutes)
            .max_lifetime(Duration::from_secs(3600))   // Max connection lifetime (1 hour)
            .max_connections(10)                       // Maximum pool connections
            .min_connections(1)                        // Minimum pool connections
            .sqlx_logging(true)                        // Enable SQL logging for debugging
            .sqlx_logging_level(log::LevelFilter::Debug);

        let connection = sea_orm::Database::connect(options)
            .await
            .expect("Failed to connect to the database");

        DatabaseService { connection }
    }
}
