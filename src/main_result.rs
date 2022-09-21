use std::io::Error as IoError;
use object::Error as ObjectError;
use config::ConfigError;
use log::SetLoggerError;

pub(crate) enum MainError {
    LoggerError(SetLoggerError),
    SettingsError(ConfigError),
    MissingElfPath,
    MissingSymbolTable,
    ExeHasNoParent,
    FileError(IoError),
    ElfError(ObjectError),
}

pub(crate) type MainResult = Result<(), MainError>;
