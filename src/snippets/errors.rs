use core::fmt;

#[derive(Debug, Clone)]
pub enum SnippetError {
    Reason(String),
}

impl fmt::Display for SnippetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            SnippetError::Reason(r) => r.clone(),
        };

        write!(f, "{}", string)
    }
}
