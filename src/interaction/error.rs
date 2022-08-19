use core::fmt;

pub type Result<T> = core::result::Result<T, Error>;

pub enum Error {
    Fatal,
    MissingPayload,
    UnsupportedInteractionType,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Fatal => "Unrecoverable error. This is unexpected behavior. Please file a bug report.",
            Self::MissingPayload => "No interaction data present.",
            Self::UnsupportedInteractionType => "Unsupported interaction type.",
        })
    }
}
