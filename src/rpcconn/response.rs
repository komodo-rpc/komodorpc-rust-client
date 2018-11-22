use std::result::Result as StdResult;
use std::{error::Error as StdError, fmt};

#[derive(Debug, Deserialize, PartialEq)]
pub struct RpcError {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct RpcResponse<R> {
    pub id: String,
    pub result: Option<R>,
    pub error: Option<RpcError>,
}

impl<R> Into<StdResult<R, RpcError>> for RpcResponse<R> {
    fn into(self) -> Result<R, RpcError> {
        match self {
            RpcResponse {
                result: Some(result),
                error: None,
                ..
            } => Ok(result),
            RpcResponse {
                result: None,
                error: Some(rpc_error),
                ..
            } => Err(rpc_error),
            _ => panic!("Response must contain either result or error."),
        }
    }
}

impl<R> RpcResponse<R> {
    pub fn into_result(self) -> StdResult<R, RpcError> {
        self.into()
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

impl StdError for RpcError {
    fn description(&self) -> &str {
        "error in rpc"
    }
}

impl fmt::Display for RpcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "code {}\nmessage {}", self.code, self.message)
    }
}
