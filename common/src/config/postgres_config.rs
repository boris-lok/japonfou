#[derive(Debug, Default)]
pub struct PostgresConfig {
    pub host: String,
    pub database: String,
    pub username: String,
    pub password: String,
    pub port: u16,
    pub max_connection: u8,
}

impl PostgresConfig {
    pub fn new(
        host: String,
        database: String,
        username: String,
        password: String,
        port: Option<u16>,
        max_connection: Option<u8>,
    ) -> Self {
        Self {
            host,
            database,
            username,
            password,
            max_connection: max_connection.unwrap_or(10),
            port: port.unwrap_or(5432),
        }
    }
}
