
use std::fmt::{self, Debug, Display};

pub use arango::ErrorCode;

#[derive(Clone, PartialEq, Eq, Deserialize)]
pub struct Error {
    #[serde(rename = "code")]
    status_code: u16,
    #[serde(rename = "errorNum")]
    error_code: ErrorCode,
    #[serde(rename = "errorMessage")]
    message: String,
}

impl Error {
    pub fn new<M>(status_code: u16, error_code: ErrorCode, message: M) -> Self
        where M: Into<String>
    {
        Error {
            status_code,
            error_code,
            message: message.into(),
        }
    }

    pub fn status_code(&self) -> u16 {
        self.status_code
    }

    pub fn error_code(&self) -> ErrorCode {
        self.error_code
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("Error {}: {} (Status: {})",
            &self.error_code.as_u16(), &self.message, &self.status_code))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&format!("Error {}: {} (Status: {})",
            &self.error_code.as_u16(), &self.message, &self.status_code))
    }
}
