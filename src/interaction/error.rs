use core::fmt;

pub type Result<T> = core::result::Result<T, Error>;

pub enum Error {
    Fatal,
    InvalidArgs,
    MissingPayload,
    UnknownCommand,
    UnknownTimezone,
    UnsupportedInteractionType,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Fatal => "Unrecoverable error. This is unexpected behavior. Please file a bug report.",
            Self::InvalidArgs => "Invalid command arguments.",
            Self::MissingPayload => "No interaction data present.",
            Self::UnknownCommand => "Unknown command name.",
            Self::UnknownTimezone => {
                "Unknown timezone. Please ensure that it is registered in the IANA time zone database."
            }
            Self::UnsupportedInteractionType => "Unsupported interaction type.",
        })
    }
}
