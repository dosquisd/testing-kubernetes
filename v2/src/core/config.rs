use rand::RngCore;
use secp256k1::SecretKey;
use std::env::VarError;
use std::sync::LazyLock;

use dotenv::dotenv;

#[derive(Debug)]
pub struct Settings {
    // Postgres Configuration
    pub postgres_user: Result<String, VarError>,
    pub postgres_password: Result<String, VarError>,
    pub postgres_host: String,
    pub postgres_port: String,
    pub postgres_db: String,

    // Redis Configuration
    pub redis_host: String,
    pub redis_port: String,
    pub redis_password: Result<String, VarError>,

    // QuestDB Configuration
    pub questdb_host: String,
    pub questdb_port: String,
    pub questdb_user: Result<String, VarError>,
    pub questdb_password: Result<String, VarError>,
    pub questdb_pg_port: String,
    pub questdb_db: String,

    // Server configuration
    pub secret_key: String,
    pub debug: bool,
}

impl Settings {
    pub fn load_env() {
        dotenv().ok();
    }

    fn generate_secret_key() -> String {
        let mut data = [0u8; 32]; // Generate a 32-byte array
        rand::rng().fill_bytes(&mut data);
        let secret_key = SecretKey::from_byte_array(data);
        secret_key.unwrap().display_secret().to_string()
    }

    pub fn load_settings() -> Self {
        Settings::load_env();

        Settings {
            postgres_user: std::env::var("POSTGRES_USER"),
            postgres_password: std::env::var("POSTGRES_PASSWORD"),
            postgres_db: std::env::var("POSTGRES_DB").unwrap_or(String::from("api_test")),
            postgres_host: std::env::var("POSTGRES_HOST").unwrap_or(String::from("localhost")),
            postgres_port: std::env::var("POSTGRES_PORT").unwrap_or(String::from("5432")),

            redis_host: std::env::var("REDIS_HOST").unwrap_or(String::from("localhost")),
            redis_port: std::env::var("REDIS_PORT").unwrap_or(String::from("6379")),
            redis_password: std::env::var("REDIS_PASSWORD"),

            questdb_host: std::env::var("QUESTDB_HOST").unwrap_or(String::from("localhost")),
            questdb_port: std::env::var("QUESTDB_PORT").unwrap_or(String::from("9000")),
            questdb_user: std::env::var("QUESTDB_USER"),
            questdb_password: std::env::var("QUESTDB_PASSWORD"),
            questdb_pg_port: std::env::var("QUESTDB_PG_PORT").unwrap_or(String::from("8812")),
            questdb_db: std::env::var("QUESTDB_DB").unwrap_or(String::from("logs")),

            secret_key: std::env::var("SECRET_KEY").unwrap_or(Settings::generate_secret_key()),
            debug: std::env::var("DEBUG")
                .unwrap_or(String::from("false"))
                .to_lowercase()
                .trim()
                == "true",
        }
    }
}

pub static SETTINGS: LazyLock<Settings> = LazyLock::new(|| Settings::load_settings());
