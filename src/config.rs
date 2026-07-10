use std::{fs::File, path::Path};

use anyhow::Context;
use serde::Deserialize;

#[derive(Deserialize)]
struct FileConfig {
    server: ServerConfig,
}

#[derive(Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
}

pub struct Config {
    server: ServerConfig,
    database_url: String,
}

impl Config {
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }

    pub(crate) fn database_url(&self) -> &str {
        &self.database_url
    }
}

pub fn load() -> anyhow::Result<Config> {
    if Path::new(".env").exists() {
        dotenvy::dotenv().context("load .env")?;
    }

    let config_file = File::open("config.yml").context("open config.yml")?;
    let file_config: FileConfig =
        yaml_serde::from_reader(config_file).context("parse config.yml")?;
    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL is required")?;

    Ok(Config {
        server: file_config.server,
        database_url,
    })
}
