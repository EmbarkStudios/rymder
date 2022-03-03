pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    HealthPingConnectionFailure(String),
    TimedOut(tokio::time::error::Elapsed),
    InvalidUri(http::uri::InvalidUri),
    Rpc(tonic::Status),
    Transport(tonic::transport::Error),
    ParseInteger(std::num::ParseIntError),
    UnknownState(String),
    InvalidIp {
        ip_str: String,
        err: std::net::AddrParseError,
    },
    InvalidComponentRange(time::error::ComponentRange),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(match self {
            Self::HealthPingConnectionFailure(_) | Self::UnknownState(_) => return None,
            Self::TimedOut(elapsed) => elapsed,
            Self::InvalidUri(invalid) => invalid,
            Self::Rpc(status) => status,
            Self::Transport(transport) => transport,
            Self::ParseInteger(parse) => parse,
            Self::InvalidIp { err, .. } => err,
            Self::InvalidComponentRange(_err) => return None,
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
            Self::UnknownState(us) => write!(f, "received an unknown state '{}' from agones", us),
            Self::InvalidIp { ip_str, err } => {
                write!(f, "failed to parse ip '{}': {}", ip_str, err)
            }
            Self::InvalidComponentRange(cr) => {
                write!(f, "invalid component range: {}", cr)
            }
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

impl From<time::error::ComponentRange> for Error {
    #[inline]
    fn from(e: time::error::ComponentRange) -> Self {
        Self::InvalidComponentRange(e)
    }
}
