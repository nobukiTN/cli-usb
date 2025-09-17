use thiserror::{Error};

#[derive(Debug,Error)]
pub enum UsbMpError{
#[error("any error")]
AnyError(String),
}