use super::config::SETTINGS;
use sea_orm::{ConnectOptions, DatabaseConnection};
use std::time::Duration;

pub struct DatabaseService {
    pub connection: DatabaseConnection,
}

pub struct DatabaseParams {
    pub protocol: Option<String>,
    pub host: String,
    pub port: String,
    pub db: String,
    pub user: Option<String>,
    pub password: Option<String>,
}

impl DatabaseService {
    pub fn create_database_uri(params: DatabaseParams) -> String {
        let credentials = match (params.user, params.password) {
            (Some(user), Some(password)) => format!("{}:{}@", user, password),
            (Some(user), _) => format!("{}@", user),
            (_, Some(password)) => format!(":{}@", password),
            (_, _) => String::new(),
        };

        format!(
            "{}://{}{}:{}/{}",
            params.protocol.unwrap_or("postgres".to_string()),
            credentials,
            params.host,
            params.port,
            params.db
        )
    }

    pub async fn init(params: Option<DatabaseParams>) -> Self {
        let params = params.unwrap_or(DatabaseParams {
            protocol: Some("postgres".to_string()),
            host: SETTINGS.postgres_host.clone(),
            port: SETTINGS.postgres_port.clone(),
            db: SETTINGS.postgres_db.clone(),
            user: SETTINGS.postgres_user.clone().ok(),
            password: SETTINGS.postgres_password.clone().ok(),
        });
        let uri = DatabaseService::create_database_uri(params);
        let mut options = ConnectOptions::new(uri);

        // Configure connection timeouts and pool settings
        options
            .connect_timeout(Duration::from_secs(8)) // Connection timeout
            .acquire_timeout(Duration::from_secs(8)) // Pool acquire timeout
            .idle_timeout(Duration::from_secs(600)) // Connection idle timeout (10 minutes)
            .max_lifetime(Duration::from_secs(3600)) // Max connection lifetime (1 hour)
            .max_connections(10) // Maximum pool connections
            .min_connections(1) // Minimum pool connections
            .sqlx_logging(true) // Enable SQL logging for debugging
            .sqlx_logging_level(log::LevelFilter::Debug);

        let connection = sea_orm::Database::connect(options)
            .await
            .expect("Failed to connect to the database");

        DatabaseService { connection }
    }
}
