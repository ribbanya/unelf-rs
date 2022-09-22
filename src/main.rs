mod process_symbols;
pub(crate) mod main_result;
pub(crate) mod parse_map;

use unwrap_elf::settings::Settings;
use crate::{
    process_symbols::process_symbols,
    main_result::{MainError::{self, *}, MainResult},
};

use std::{fs, path::PathBuf, time::Instant};
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;

fn main() {
    let before = Instant::now();

    if let Err(error) = try_main() {
        handle_error(error);
    }

    info!("Elapsed time: {:.2?}", before.elapsed());
}

fn try_main() -> MainResult {
    try_init_logger()?;

    let settings = Settings::new().map_err(SettingsError)?;

    let elf_data = {
        let path = settings.elf.path.ok_or(MissingElfPath)?;
        fs::read(path).map_err(FileError)?
    };

    let out = {
        let path = get_out_path()?;
        fs::File::create(path).map_err(FileError)?
    };

    process_symbols(elf_data, out)?;

    Ok(())
}

fn handle_error(error: MainError) {
    match error {
        LoggerError(inner) => todo!("{}", inner.to_string()),
        SettingsError(inner) => todo!("{inner}"),
        MissingElfPath => todo!("Missing elf path"),
        MissingSymbolTable => todo!("Missing symbol table"),
        FileError(inner) => todo!("{inner}"),
        ElfError(inner) => todo!("{inner}"),
        ExeHasNoParent => todo!("This shouldn't happen...")
    };
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
