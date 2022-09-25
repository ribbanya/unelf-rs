mod process_symbols;
mod db;
pub(crate) mod main_result;
pub(crate) mod parse_map;

use unelf::settings::Settings;
use crate::{
    process_symbols::process_symbols,
    parse_map::parse_map,
    main_result::{MainError::{self, *}, MainResult},
    db::*,
};

use std::{fs, path::PathBuf, time::Instant};
use sha2::{Sha256, Digest};
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;

fn main() -> MainResult {
    let before = Instant::now();

    try_init_logger()?;

    let settings = Settings::new().map_err(SettingsError)?;
    let mut sha256 = Sha256::new();

    {
        let map_text = {
            let path = settings.map.path.ok_or(MissingMapPath)?;
            fs::read_to_string(path).map_err(FileError)?
        };

        sha256.update(&map_text);
        println!("{:256x}", sha256.finalize_reset());
        // parse_map(&map_text)?;
    }

    {
        let elf_data = {
            let path = settings.elf.path.ok_or(MissingElfPath)?;
            fs::read(path).map_err(FileError)?
        };

        sha256.update(&elf_data);
        println!("{:256x}", sha256.finalize_reset());

        let out = {
            let path = get_out_path()?;
            fs::File::create(path).map_err(FileError)?
        };

        // process_symbols(elf_data, out)?;
    }

    info!("Elapsed time: {:.2?}", before.elapsed());

    Ok(())
}

fn get_out_path() -> Result<PathBuf, MainError> {
    let exe_path = std::env::current_exe().map_err(FileError)?;
    let parent = exe_path.parent().ok_or(ExeHasNoParent)?;
    Ok(parent.join("out.s"))
}

fn try_init_logger() -> MainResult {
    SimpleLogger::new()
        .with_level(
            if cfg!(debug_assertions) {
                LevelFilter::Debug
            } else {
                LevelFilter::Warn
            })
        .init().map_err(LoggerError)
}
