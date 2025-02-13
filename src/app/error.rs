#[derive(Debug)]
pub(crate) enum Error {
    JSONError(crate::client::JSONError),
    RegexError(crate::etuovi::RegexError),
    RequestError(crate::client::RequestError),
    TeloxideError(teloxide::RequestError),
    OpenRouteServiceError(crate::open_route_service::Error),
    TokioTaskJoinError(tokio::task::JoinError),
    IOError(std::io::Error),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::JSONError(e) => write!(f, "Request or IO error:\n{}", e),
            Error::RegexError(e) => write!(f, "Regex error:\n{}", e),
            Error::RequestError(e) => write!(f, "Request error:\n{}", e),
            Error::TeloxideError(e) => write!(f, "Teloxide error:\n{}", e),
            Error::OpenRouteServiceError(e) => write!(f, "OpenRouteService error:\n{}", e),
            Error::TokioTaskJoinError(e) => write!(f, "Tokio task join error:\n{}", e),
            Error::IOError(e) => write!(f, "IO error:\n{}", e),
        }
    }
}
impl From<crate::client::JSONError> for Error {
    fn from(err: crate::client::JSONError) -> Self {
        Error::JSONError(err)
    }
}
impl From<crate::etuovi::RegexError> for Error {
    fn from(err: crate::etuovi::RegexError) -> Self {
        Error::RegexError(err)
    }
}
impl From<crate::client::RequestError> for Error {
    fn from(err: crate::client::RequestError) -> Self {
        Error::RequestError(err)
    }
}
impl From<teloxide::RequestError> for Error {
    fn from(err: teloxide::RequestError) -> Self {
        Error::TeloxideError(err)
    }
}
impl From<crate::open_route_service::Error> for Error {
    fn from(err: crate::open_route_service::Error) -> Self {
        Error::OpenRouteServiceError(err)
    }
}
impl From<tokio::task::JoinError> for Error {
    fn from(error: tokio::task::JoinError) -> Self {
        Error::TokioTaskJoinError(error)
    }
}
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IOError(error)
    }
}
