#[derive(Debug, Default)]
pub struct RedisConfig {
    pub host: String,
    pub username: String,
    pub password: String,
    pub port: u16,
}

impl RedisConfig {
    pub fn new(host: String, username: String, password: String, port: Option<u16>) -> Self {
        Self {
            host,
            username,
            password,
            port: port.unwrap_or(6379),
        }
    }
}
