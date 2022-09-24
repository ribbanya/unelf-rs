use std::io::Error as IoError;
use object::Error as ObjectError;
use config::ConfigError;
use log::SetLoggerError;
use regex::Error as RegexError;

#[derive(Debug)]
pub(crate) enum MainError {
    LoggerError(SetLoggerError),
    SettingsError(ConfigError),
    MissingElfPath,
    MissingMapPath,
    MissingSymbolTable,
    ExeHasNoParent,
    FileError(IoError),
    ElfError(ObjectError),
    RegexError(RegexError),
}

pub(crate) type MainResult = Result<(), MainError>;
