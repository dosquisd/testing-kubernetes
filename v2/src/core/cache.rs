use std::sync::LazyLock;

use super::config::SETTINGS;
use redis::{self, Commands};

pub struct RedisService {
    client: redis::Client,
}

impl RedisService {
    pub fn new() -> RedisService {
        let params = redis::ConnectionInfo {
            addr: redis::ConnectionAddr::Tcp(
                SETTINGS.redis_host.clone(),
                SETTINGS.redis_port.parse().expect("Invalid Redis port"),
            ),
            redis: redis::RedisConnectionInfo {
                db: 0,
                username: None,
                password: SETTINGS.redis_password.clone().ok(),
                protocol: redis::ProtocolVersion::default(),
            },
        };

        RedisService {
            client: redis::Client::open(params).expect("Failed to create Redis client"),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        Commands::get(&mut self.client.get_connection().unwrap(), key).ok()
    }

    pub fn set(&self, key: &str, value: &str) -> redis::RedisResult<()> {
        Commands::set(&mut self.client.get_connection().unwrap(), key, value)
    }

    pub fn delete(&self, key: &str) -> redis::RedisResult<()> {
        Commands::del(&mut self.client.get_connection().unwrap(), key)
    }

    pub fn delete_pattern(&self, pattern: &str) -> usize {
        let mut con = self.client.get_connection().unwrap();
        let keys: Vec<String> = Commands::keys(&mut con, pattern).unwrap_or(vec![]);
        if keys.is_empty() {
            return 0;
        }

        let results: Vec<redis::RedisResult<()>> = keys
            .iter()
            .map(|key| Commands::del(&mut con, key))
            .collect();
        results.iter().filter(|res| res.is_ok()).count()
    }
}

pub static REDIS_SERVICE: LazyLock<RedisService> = LazyLock::new(|| RedisService::new());
