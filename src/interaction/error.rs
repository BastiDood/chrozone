use core::fmt;

pub type Result<T> = core::result::Result<T, Error>;

pub enum Error {
    UnsupportedInteractionType,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::UnsupportedInteractionType => "Unsupported interaction type.",
        })
    }
}
