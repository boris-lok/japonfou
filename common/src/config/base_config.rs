#[derive(Debug, Default, Clone)]
pub struct Config {
    pub debug: bool,
    pub secret_key: String,
}

impl Config {
    pub fn new(debug: Option<bool>, secret_key: String) -> Self {
        Self {
            debug: debug.unwrap_or(true),
            secret_key,
        }
    }
}
