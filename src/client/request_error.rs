#[derive(Debug)]
pub(crate) enum RequestError {
    ReqwestError(reqwest::Error),
    IOError(std::io::Error),
    URLParseError(url::ParseError),
}
impl std::fmt::Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestError::ReqwestError(e) => write!(f, "Reqwest error:\n{}", e),
            RequestError::IOError(e) => write!(f, "IO error:\n{}", e),
            RequestError::URLParseError(e) => write!(f, "URL parse error:\n{}", e),
        }
    }
}
impl From<reqwest::Error> for RequestError {
    fn from(err: reqwest::Error) -> Self {
        RequestError::ReqwestError(err)
    }
}
impl From<std::io::Error> for RequestError {
    fn from(err: std::io::Error) -> Self {
        RequestError::IOError(err)
    }
}
impl From<url::ParseError> for RequestError {
    fn from(err: url::ParseError) -> Self {
        RequestError::URLParseError(err)
    }
}
