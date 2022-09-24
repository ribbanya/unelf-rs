use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Elf {
    pub path: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
pub struct Map {
    pub path: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub elf: Elf,
    pub map: Map,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("config/local")
                .required(false))
            .build()?;
        s.try_deserialize()
    }
}
