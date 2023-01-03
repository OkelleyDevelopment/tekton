//! Various Error enums for the tekton program
use core::fmt;
use std::io;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TektonError {
    /// An error with a custom message as a String
    Reason(String),
    /// An 'error state' that indicates the file needs to process with the `multi_prefix` support
    SwitchModes(bool),
}

impl fmt::Display for TektonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            TektonError::Reason(r) => r.clone(),
            TektonError::SwitchModes(b) => b.to_string(),
        };
        write!(f, "{}", string)
    }
}

impl std::error::Error for TektonError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl std::convert::From<io::Error> for TektonError {
    fn from(io_err: io::Error) -> Self {
        TektonError::Reason(io_err.to_string())
    }
}

#[test]
fn test_tekton_error_enum() {
    let reason = "test error".to_string();
    let err = TektonError::Reason(reason.clone());
    let string_err = err.to_string();
    assert_eq!(string_err, reason);
}

#[test]
fn test_tekton_switch() {
    let switch = TektonError::SwitchModes(true);
    assert_eq!(switch, TektonError::SwitchModes(true));
}
