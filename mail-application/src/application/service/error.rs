use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Email `{0}` is already taken")]
    UserExist(String),
}
