//! Various Error enums for the tekton program
use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TektonError {
    /// An error with a custom message as a String
    Reason(String),
}

impl fmt::Display for TektonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            TektonError::Reason(r) => r.clone(),
        };
        write!(f, "{}", string)
    }
}

#[test]
fn test_tekton_error_enum() {
    let reason = "test error".to_string();
    let err = TektonError::Reason(reason.clone());
    let string_err = err.to_string();
    assert_eq!(string_err, reason);
}
