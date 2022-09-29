use crate::types::ErrorInfo;

#[derive(thiserror::Error, serde::Serialize, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// 401
    #[error("unauthorized")]
    Unauthorized,

    /// 403
    #[error("forbidden")]
    Forbidden,

    /// 404
    #[error("not found")]
    NotFound,

    /// 422
    #[error("unprocess")]
    UnprocessableEntity(ErrorInfo),

    /// 500
    #[error("internal server error")]
    InternalServerError,

    /// serde deserialize error
    #[error("deserialize error")]
    DeserializeError,

    /// request error
    #[error("http request error")]
    RequestError,
}