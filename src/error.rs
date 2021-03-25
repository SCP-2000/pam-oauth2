use pam_sys::types::PamReturnCode;
use std::error;
use std::fmt;

#[derive(Debug)]
pub struct PamError {
    value: PamReturnCode,
}

impl PamError {
    pub fn new(value: PamReturnCode) -> Self {
        PamError { value }
    }
}

impl fmt::Display for PamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl error::Error for PamError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
