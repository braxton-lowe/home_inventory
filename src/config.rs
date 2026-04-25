use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub auth_username: Option<String>,
    pub auth_password: Option<String>,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://grocery_user:dev_password@localhost:5432/home_food_inventory".to_string());

        let host = env::var("HOST")
            .unwrap_or_else(|_| "0.0.0.0".to_string());

        let port = env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()?;

        let auth_username = env::var("AUTH_USERNAME").ok();
        let auth_password = env::var("AUTH_PASSWORD").ok();

        Ok(Config {
            database_url,
            host,
            port,
            auth_username,
            auth_password,
        })
    }

    pub fn addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
