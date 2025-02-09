#[derive(Debug)]
pub(crate) enum Error {
    JSONError(crate::client::JSONError),
    InvalidHeaderValue(reqwest::header::InvalidHeaderValue),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::JSONError(e) => write!(f, "Request or IO error:\n{}", e),
            Error::InvalidHeaderValue(e) => write!(f, "Invalid header value:\n{}", e),
        }
    }
}
impl From<crate::client::JSONError> for Error {
    fn from(err: crate::client::JSONError) -> Self {
        Error::JSONError(err)
    }
}
impl From<reqwest::header::InvalidHeaderValue> for Error {
    fn from(err: reqwest::header::InvalidHeaderValue) -> Self {
        Error::InvalidHeaderValue(err)
    }
}
