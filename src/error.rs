use std::fmt::{Display, Formatter, Debug};
use anyhow::{anyhow};
use std::error::Error;


pub type NpResult<T> = Result<T, NpError>;

pub struct NpError {
    error : anyhow::Error,
}

impl NpError {
    pub fn new(e : impl Error + Send + Sync + 'static) -> Self{ Self{ error : e.into() } }
}

impl Display for NpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.error, f)
    }
}

impl Debug for NpError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.error, f)
    }
}

impl Error for NpError{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.error.source()
    }
}


impl From<std::io::Error> for NpError {
    fn from(e : std::io::Error) -> Self { Self::new(e) }
}

impl From<docchi_json5::MyError> for NpError {
    fn from(e : docchi_json5::MyError) -> Self { Self::new(e) }
}

impl From<&str> for NpError {
    fn from(e : &str) -> Self{ Self::new(anyhow!("{}", e)) }
}

impl From<String> for NpError {
    fn from(e : String) -> Self{ Self::new(anyhow!("{}", e)) }
}
