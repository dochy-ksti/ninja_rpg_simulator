use std::fmt::{Display, Formatter, Debug};
use std::error::Error;
use std::time::SystemTimeError;

pub struct NlError {
    error : anyhow::Error,
}

impl NlError {
    pub fn new(e : impl Error + Send + Sync + 'static) -> Self{ Self{ error : e.into() } }
}

impl Display for NlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.error, f)
    }
}

impl Debug for NlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.error, f)
    }
}

impl Error for NlError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.error.source()
    }
}


impl From<std::io::Error> for NlError {
    fn from(e : std::io::Error) -> Self { Self::new(e) }
}

impl From<SystemTimeError> for NlError {
    fn from(e : SystemTimeError) -> Self { Self::new(e) }
}

impl From<docchi_json5::MyError> for NlError {
    fn from(e : docchi_json5::MyError) -> Self { Self::new(e) }
}

impl From<serde_json::Error> for NlError {
    fn from(e : serde_json::Error) -> Self { Self::new(e) }
}

impl From<anyhow::Error> for NlError{
    fn from(e: anyhow::Error) -> Self {
        Self{ error: e }
    }
}

impl From<&str> for NlError {
    fn from(e : &str) ->Self{ anyhow::Error::msg(e.to_string()).into() }
}

impl From<String> for NlError {
    fn from(e : String) -> Self{ anyhow::Error::msg(e).into() }
}
