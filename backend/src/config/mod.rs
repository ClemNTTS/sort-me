pub mod env;

#[derive(Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub app_port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            database_url: env::get_database_url(),
            app_port: env::get_app_port(),
        }
    }
}
