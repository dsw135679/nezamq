use std::{
  fs,
  path::Path,
  time::{SystemTime, UNIX_EPOCH},
};

use crate::error::CommonError;

pub fn read_file(path: &String) -> Result<String, CommonError> {
  if !Path::new(path).exists() {
    return Err(CommonError::CommonError(format!(
      "File {} does not exist",
      path
    )));
  }

  return Ok(fs::read_to_string(&path)?);
}

pub fn file_exists(path: &String) -> bool {
  return Path::new(path).exists();
}

pub fn create_fold(fold: &String) -> Result<(), CommonError> {
  if !Path::new(fold).exists() {
    fs::create_dir_all(fold)?
  }

  return Ok(());
}

pub fn now_second() -> u64 {
  return SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs();
}
