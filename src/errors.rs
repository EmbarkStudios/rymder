pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    HealthPingConnectionFailure(String),
    TimedOut(tokio::time::error::Elapsed),
    InvalidUri(http::uri::InvalidUri),
    Rpc(tonic::Status),
    Transport(tonic::transport::Error),
    ParseInteger(std::num::ParseIntError),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(match self {
            Self::HealthPingConnectionFailure(_) => return None,
            Self::TimedOut(elapsed) => elapsed,
            Self::InvalidUri(invalid) => invalid,
            Self::Rpc(status) => status,
            Self::Transport(transport) => transport,
            Self::ParseInteger(parse) => parse,
        })
    }
}

use std::fmt;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HealthPingConnectionFailure(inner) => {
                write!(f, "health ping connection failure: `{}`", inner)
            }
            Self::TimedOut(elapsed) => write!(f, "{}", elapsed),
            Self::InvalidUri(invalid) => write!(f, "failed to parse connection uri: `{}`", invalid),
            Self::Rpc(status) => write!(f, "rpc failure: `{}`", status),
            Self::Transport(transport) => write!(f, "transport failure: `{}`", transport),
            Self::ParseInteger(parse) => write!(f, "failed to parse integer: `{}`", parse),
        }
    }
}

impl From<tokio::time::error::Elapsed> for Error {
    #[inline]
    fn from(e: tokio::time::error::Elapsed) -> Self {
        Self::TimedOut(e)
    }
}

impl From<http::uri::InvalidUri> for Error {
    #[inline]
    fn from(e: http::uri::InvalidUri) -> Self {
        Self::InvalidUri(e)
    }
}

impl From<tonic::Status> for Error {
    #[inline]
    fn from(e: tonic::Status) -> Self {
        Self::Rpc(e)
    }
}

impl From<tonic::transport::Error> for Error {
    #[inline]
    fn from(e: tonic::transport::Error) -> Self {
        Self::Transport(e)
    }
}
