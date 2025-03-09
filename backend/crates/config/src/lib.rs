use eyre::eyre;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Clone, Copy)]
pub enum Environment {
    Development,
    Production,
    Test,
}

pub fn get_env() -> eyre::Result<Environment> {
    match std::env::var("APP_ENVIRONMENT") {
        Ok(val) => {
            tracing::info!(APP_ENVIRONMENT = val, "Setting environment");
            parse_env(&val)
        }
        Err(_) => {
            tracing::info!("Defaulting to environment: development");
            Ok(Environment::Development)
        }
    }
}

pub fn parse_env(env: &str) -> eyre::Result<Environment> {
    let env = env.to_ascii_lowercase();
    match env.as_str() {
        "dev" => Ok(Environment::Development),
        "development" => Ok(Environment::Development),
        "test" => Ok(Environment::Test),
        "prod" => Ok(Environment::Production),
        "production" => Ok(Environment::Production),
        unknown => Err(eyre!(r#"Unknown environment: "{unknown}"!"#)),
    }
}

pub fn load_config(_env: Environment) -> eyre::Result<Config> {
    // TODO: don't hardcode this
    Ok(Config {
        server: ServerConfig { port: 8080 },
        database: DatabaseConfig {
            url: "postgres://postgres:postgres@localhost:5432/ecupool".to_string(),
        },
    })
}
