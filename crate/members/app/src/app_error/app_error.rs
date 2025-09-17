use thiserror::{Error};

#[derive(Debug,Error)]
pub enum AppError{
    #[error("any error debubg")]
    AnyErrorDebug(String),
}