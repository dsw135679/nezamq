pub mod index;
pub mod server;

pub use index::*;

pub(crate) fn v1_path(path: &str) -> String {
  return format!("/v1{}", path);
}

pub(crate) fn path_list(path: &str) -> String {
  return format!("{}/list", path);
}

pub(crate) fn path_delete(path: &str) -> String {
  return format!("{}/delete", path);
}

pub(crate) fn path_create(path: &str) -> String {
  return format!("{}/create", path);
}
pub(crate) fn path_update(path: &str) -> String {
  return format!("{}/update", path);
}
