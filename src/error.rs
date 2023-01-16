use restson::Error as RestsonError;
/// Errors which can be returned from the Transdirect API
/// 
/// 
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    UnreadableResponse,
    UnknownStatus,
    HTTPError(String),
}

impl From<RestsonError> for Error {
    fn from(err: RestsonError) -> Error {
        Error::HTTPError(err.to_string())
    }
}
