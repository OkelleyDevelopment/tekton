//pub mod tekton_error;
use core::fmt;

#[derive(Debug, Clone)]
pub enum TektonError {
    Reason(String),
    VimSnippetError(String),
    FriendlySnippetError(String),
}

impl fmt::Display for TektonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            TektonError::Reason(r) => r.clone(),
            TektonError::VimSnippetError(r) => r.clone(),
            TektonError::FriendlySnippetError(r) => r.clone(),
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
