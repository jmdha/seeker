use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Unfinished,
    Unsolvable,
    OutOfTime,
    OutOfMemory,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Unfinished => write!(f, "Unfinished"),
            Error::Unsolvable => write!(f, "Unsolvable"),
            Error::OutOfTime => write!(f, "Out of time"),
            Error::OutOfMemory => write!(f, "Out of memory"),
        }
    }
}
