use std::str::FromStr;
use thiserror::Error;
use tonic::{service::Interceptor, Status};

#[derive(Debug, Error)]
#[error("FAILURE_MODE of '{0}' not supported. Try 'INTERCEPTOR', 'METHOD', or 'NONE'")]
pub struct Error(String);

/// Configurable failure modes for response comparison
#[derive(Debug)]
pub enum FailureMode {
    None,
    Interceptor,
    Method,
}

impl FailureMode {
    /// Create a tonic Interceptor from the failure configuration
    pub fn to_interceptor(&self) -> impl Interceptor + Clone {
        match self {
            Self::None | Self::Method => Result::Ok,
            Self::Interceptor => |_| Err(Status::permission_denied("Super secure")),
        }
    }
}

impl Default for FailureMode {
    fn default() -> Self {
        Self::None
    }
}

impl FromStr for FailureMode {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lowercase = input.to_lowercase();
        let mode = match lowercase.as_str() {
            "interceptor" => Self::Interceptor,
            "method" => Self::Method,
            "none" => Self::None,
            _ => return Err(Error(input.to_string())),
        };

        Ok(mode)
    }
}
