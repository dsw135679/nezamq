use std::error::Error;

use common_base::errors::NezaMQError;
use openraft::error::{RPCError, Unreachable};

use super::typeconfig::TypeConfig;

pub fn to_error<E: Error + 'static + Clone>(e: NezaMQError) -> RPCError<TypeConfig, E> {
  RPCError::Unreachable(Unreachable::new(&e))
}
