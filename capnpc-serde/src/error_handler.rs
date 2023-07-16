use std::fmt;

pub type CapSerResult<T> = std::result::Result<T, CapSerError>;

#[derive(Debug, Clone)]
pub struct CapSerError {
    message: String,
}

impl CapSerError {
    pub fn new(message: &str) -> Self {
        let message = String::from(message);
        CapSerError { message }
    }
}

impl fmt::Display for CapSerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message.as_str())
    }
}

impl From<capnp::Error> for CapSerError {
    fn from(err: capnp::Error) -> CapSerError {
        let message = format!("{}", err);
        CapSerError { message }
    }
}

impl From<serde_json::Error> for CapSerError {
    fn from(err: serde_json::Error) -> CapSerError {
        let message = format!("{}", err);
        CapSerError { message }
    }
}

impl From<capnp::NotInSchema> for CapSerError {
    fn from(err: capnp::NotInSchema) -> CapSerError {
        let message = format!("{}", err);
        CapSerError { message }
    }
}

impl From<std::io::Error> for CapSerError {
    fn from(err: std::io::Error) -> CapSerError {
        let message = format!("{}", err);
        CapSerError { message }
    }
}
