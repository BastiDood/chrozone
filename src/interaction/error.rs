use core::fmt;

pub type Result<T> = core::result::Result<T, Error>;

pub enum Error {
    AmbiguousTime,
    Fatal,
    InvalidArgs,
    MissingPayload,
    MissingRequired,
    UnknownCommand,
    UnknownTimezone,
    UnsupportedInteractionType,
    OutOfRange,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::AmbiguousTime => "Provided date and time are ambiguous (i.e. more than one possible interpretation).",
            Self::Fatal => "Unrecoverable error. This is unexpected behavior. Please file a bug report.",
            Self::InvalidArgs => "Invalid command arguments.",
            Self::MissingPayload => "No interaction data present.",
            Self::MissingRequired => "Required arguments not provided.",
            Self::UnknownCommand => "Unknown command name.",
            Self::UnknownTimezone => "Unknown timezone. Please ensure that it is in the IANA Time Zone Database.",
            Self::UnsupportedInteractionType => "Unsupported interaction type.",
            Self::OutOfRange => "A value is out of range. It is either too large or too small.",
        })
    }
}
