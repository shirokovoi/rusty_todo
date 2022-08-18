use serde::Deserialize;

#[derive(Deserialize)]
pub struct DB {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct ApplicationConfig {
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(rename = "DB")]
    pub db: DB,
}

fn default_log_level() -> String {
    "info".to_owned()
}
