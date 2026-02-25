use config::{Config, ConfigError, File, FileFormat};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
    pub pool_size: u8,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
    pub lease_recovery: u8,
    pub cleanup: u8,
}

#[derive(Debug, Deserialize)]
pub struct Worker {
    pub heartbeat: u8,
    pub lease_duration: u8,
}

#[derive(Debug, Deserialize)]
pub struct MailServer {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub database: Database,
    pub server: Server,
}

#[derive(Debug, Deserialize)]
pub struct WorkerConfig {
    pub database: Database,
    pub worker: Worker,
    pub mail_server: MailServer,
}

pub fn load_server_config(path: &str) -> Result<ServerConfig, ConfigError> {
    let config = Config::builder()
        .add_source(File::new(path, FileFormat::Yaml))
        .build()?
        .try_deserialize()?;

    Ok(config)
}
pub fn load_worker_config(path: &str) -> Result<WorkerConfig, ConfigError> {
    let config = Config::builder()
        .add_source(File::new(path, FileFormat::Yaml))
        .build()?
        .try_deserialize()?;

    Ok(config)
}
