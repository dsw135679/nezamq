use std::io;

use thiserror::Error;

#[derive(Error,Debug)]
pub enum NezaMQError{
    #[error("{0}")]
    CommonError(String),
    #[error("io error")]
    IOJsonError(#[from] io::Error)
}