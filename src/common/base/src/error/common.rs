use std::io;

use thiserror::Error;
use tonic::Status;
use valico::json_schema::SchemaError;

#[derive(Debug, Error)]
pub enum CommonError {
  #[error("{0}")]
  TokioBroacastSendErrorBool(#[from] tokio::sync::broadcast::error::SendError<bool>),
  #[error("{0}")]
  FromTonicTransport(#[from] tonic::transport::Error),
  #[error("{0}")]
  FromErrorKind(#[from] Box<bincode::ErrorKind>),
  #[error("{0}")]
  FromDecodeError(#[from] prost::DecodeError),
  #[error("{0}")]
  FromSerdeJsonError(#[from] serde_json::Error),
  #[error("{0}")]
  FromRocksdbError(#[from] rocksdb::Error),
  #[error("{0}")]
  SchemaError(#[from] SchemaError),
  #[error("{0}")]
  FromIoError(#[from] io::Error),
  #[error("{0}")]
  FromUtf8Error(#[from] std::string::FromUtf8Error),
  #[error("{0}")]
  FromAddrParseError(#[from] std::net::AddrParseError),
  #[error("{0}")]
  ApacheAvroError(#[from] apache_avro::Error),
  #[error("{0}")]
  FromParseIntError(#[from] std::num::ParseIntError),
  #[error("{0}")]
  CommonError(String),
  #[error("{0}")]
  GrpcServerStatus(#[from] tonic::Status),
  #[error("{0} connection pool has no connection information available. {1}")]
  NoAvailableGrpcConnection(String, String),
}

impl From<CommonError> for Status {
  fn from(value: CommonError) -> Self {
    Status::cancelled(value.to_string())
  }
}
