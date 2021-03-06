use {RpcError, ClientError};
use core::fmt;
use std::fmt::Formatter;
use std::error::Error;
use std::io;
use std::num::ParseIntError;
use bitcoin::util::hash::HexError;

#[derive(Debug)]
pub enum ApiError {
    RPC(RpcError),
    Client(ClientError),
    Config(String),
    IO(io::Error),
    ParseInt(ParseIntError),
    Hex(bitcoin::util::hash::HexError),
    Other(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            ApiError::RPC(ref cause) => write!(f, "RPC error: {}", cause.message ),
            ApiError::Client(ref _cause) =>
                match _cause {
                    ClientError::Json(json_err) => fmt::Display::fmt(json_err, f),
                    ClientError::Transport(transport_error) => fmt::Display::fmt(transport_error, f),
                },
            ApiError::Config(ref err) => write!(f, "{}", err),
            ApiError::IO(ref cause) => write!(f, "IO error: {:?}", cause.kind()),
            ApiError::ParseInt(ref err) => write!(f, "Parse error: {:?}", err.description()),
            ApiError::Hex(ref err) => write!(f, "Parse error: {:?}", err.description()),
            ApiError::Other(ref err) => write!(f, "{}", err)
        }
    }
}

impl Error for ApiError {
    fn description(&self) -> &str {
        match *self {
            ApiError::RPC(ref cause) => cause.description(),
            ApiError::Client(ref cause) => cause.description(),
            ApiError::Config(ref err) => err,
            ApiError::IO(ref cause) => cause.description(),
            ApiError::ParseInt(ref cause) => cause.description(),
            ApiError::Hex(ref cause) => cause.description(),
            ApiError::Other(ref err) => err,
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ApiError::RPC(ref cause) => Some(cause),
            ApiError::Client(ref cause) => Some(cause),
            ApiError::Config(_) => None,
            ApiError::IO(ref cause) => Some(cause),
            ApiError::ParseInt(ref err) => Some(err),
            ApiError::Hex(ref err) => Some(err),
            ApiError::Other(_) => None,
        }
    }
}

impl From<io::Error> for ApiError {
    fn from(e: io::Error) -> ApiError {
        ApiError::IO(e)
    }
}

impl From<RpcError> for ApiError {
    fn from(cause: RpcError) -> ApiError {
        ApiError::RPC(cause)
    }
}

impl From<ClientError> for ApiError {
    fn from(cause: ClientError) -> ApiError {
        ApiError::Client(cause)
    }
}

impl From<ParseIntError> for ApiError {
    fn from(e: ParseIntError) -> ApiError {
        ApiError::ParseInt(e)
    }
}

impl From<HexError> for ApiError {
    fn from(e: HexError) -> ApiError {
        ApiError::Hex(e)
    }
}