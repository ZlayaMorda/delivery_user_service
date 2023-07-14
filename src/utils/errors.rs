use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthorizationError {
    #[error("Could not insert or get result user")]
    FailedDieselResult(#[from] diesel::result::Error),
    #[error("Could not encode JWT")]
    FailedToGenerateJWT (String),
    #[error("Invalid password")]
    InvalidPassword(String),
    #[error("Could not find user")]
    UserDoesNotExist(String),
}