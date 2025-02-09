#[derive(Debug)]
pub(crate) enum RegexError {
    RequestError(crate::client::RequestError),
    RegexError(regex::Error),
}
impl std::fmt::Display for RegexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegexError::RequestError(e) => write!(f, "Request or IO error:\n{}", e),
            RegexError::RegexError(e) => write!(f, "Regex error:\n{}", e),
        }
    }
}
impl From<crate::client::RequestError> for RegexError {
    fn from(err: crate::client::RequestError) -> Self {
        RegexError::RequestError(err)
    }
}
impl From<regex::Error> for RegexError {
    fn from(err: regex::Error) -> Self {
        RegexError::RegexError(err)
    }
}
