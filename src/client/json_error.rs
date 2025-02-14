#[derive(Debug)]
pub(crate) enum JSONError {
    RequestError(super::RequestError),
    SerdeJSONError(serde_json::Error),
}
impl std::fmt::Display for JSONError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JSONError::RequestError(e) => write!(f, "Request or IO error:\n{}", e),
            JSONError::SerdeJSONError(e) => write!(f, "Serde JSON error:\n{}", e),
        }
    }
}
impl From<super::RequestError> for JSONError {
    fn from(err: super::RequestError) -> Self {
        JSONError::RequestError(err)
    }
}
impl From<serde_json::Error> for JSONError {
    fn from(err: serde_json::Error) -> Self {
        JSONError::SerdeJSONError(err)
    }
}
