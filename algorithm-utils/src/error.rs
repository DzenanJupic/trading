use std::fmt::Formatter;

#[derive(Clone, Debug)]
pub struct Error {
    msg: String,
    kind: ErrorKind,
}

impl Error {
    pub fn new(msg: String, kind: ErrorKind) -> Self {
        Self {
            msg,
            kind,
        }
    }

    pub fn msg(&self) -> &str {
        &self.msg
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Error({:?}): {}", self.kind, self.msg)
    }
}

impl std::error::Error for Error {}

impl From<libloading::Error> for Error {
    fn from(error: libloading::Error) -> Self {
        Self {
            msg: format!("{:?}", error),
            kind: ErrorKind::LibLoading,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self {
            msg: err.to_string(),
            kind: ErrorKind::IO,
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub enum ErrorKind {
    BrokerConnectionFailed,
    CouldNotLogin,
    CouldNotBuy,
    CouldNotSell,
    IO,
    LibLoading,
    MisMatchedVersion,
    NoNewPositions,
    Other,
    Panic,
    TimeOut,
}