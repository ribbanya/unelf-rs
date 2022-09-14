use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Elf {
    pub path: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub elf: Elf,
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
