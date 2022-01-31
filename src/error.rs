use lavalink_rs::error::LavalinkError;
use serenity::prelude::SerenityError;
use sqlx::error::Error as SqlxError;
use sqlx::migrate::MigrateError;
use std::{error::Error as RawStdError, fmt, io::Error as StdIOError};

pub type StdError = Box<dyn RawStdError + Send + Sync>;
pub type CommandResult<T = (), E = Error> = Result<T, E>;

#[derive(Debug)]
pub enum Error {
    RequiredArgument(String),
    Join(String),

    Sqlx(SqlxError),
    Migrate(MigrateError),

    Serenity(SerenityError),
    StdIO(StdIOError),
    Lavalink(LavalinkError),
}

impl RawStdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::RequiredArgument(why) => write!(f, "Required Argument Error => {}", why),
            Self::Join(why) => write!(f, "Error while joining voice channel => {}", why),
            Self::Sqlx(err) => write!(f, "Sqlx Error => {:?}", err),
            Self::Migrate(err) => write!(f, "Error while running migrations => {:?}", err),
            Self::Serenity(err) => write!(f, "Serenity Error => {}", err),
            Self::StdIO(err) => write!(f, "std::io Error => {}", err),
            Self::Lavalink(err) => write!(f, "Lavalink Error => {}", err),
        }
    }
}

impl From<SerenityError> for Error {
    fn from(err: SerenityError) -> Self {
        Self::Serenity(err)
    }
}

impl From<StdIOError> for Error {
    fn from(err: StdIOError) -> Self {
        Self::StdIO(err)
    }
}

impl From<LavalinkError> for Error {
    fn from(err: LavalinkError) -> Self {
        Self::Lavalink(err)
    }
}

impl From<SqlxError> for Error {
    fn from(err: SqlxError) -> Self {
        Self::Sqlx(err)
    }
}

impl From<MigrateError> for Error {
    fn from(err: MigrateError) -> Self {
        Self::Migrate(err)
    }
}
