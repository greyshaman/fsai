use std::time::SystemTime;
// use std::fs::{Permissions, FileType}; // TODO: Add this letter on functionality implementation 
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FSType {
  File,
  Directory,
  Link,
  Unhandled,
}

impl Default for FSType {
  fn default() -> FSType {
    FSType::Unhandled
  }
}

#[derive(Debug, Clone)]
pub struct FSTime {
  pub time: SystemTime,
}

impl Default for FSTime {
  fn default() -> FSTime {
    FSTime {
      time: SystemTime::now()
    }
  }
}

#[derive(Builder, Debug)]
#[builder(setter(into))]
pub struct FSResource {
  fs_type: FSType,
  path: PathBuf,
  size: u64,
  create_time: FSTime,
  modify_time: FSTime,
}

impl FSResource {
  pub fn fs_type(&self) -> &FSType {
    &self.fs_type
  }

  pub fn set_fs_type(&mut self, fst_value: &FSType) {
    self.fs_type = *fst_value;
  }

  pub fn path(&self) -> &PathBuf {
    &self.path
  }

  pub fn size(&self) -> &u64 {
    &self.size
  }

  pub fn create_time(&self) -> &SystemTime {
    &self.create_time.time
  }

  pub fn set_create_time(&mut self, time: &SystemTime) {
    self.create_time.time = *time;
  }

  pub fn modify_time(&self) -> &SystemTime {
    &self.modify_time.time
  }

  pub fn set_modify_time(&mut self, time: &SystemTime) {
    self.modify_time.time = *time;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // fn create_fs_resource() -> FSResource {
  //   FSResource {
  //     fs_type: FSType::File,
  //     path: PathBuf::from("/path/to/test.file"),
  //     size: 1000u64,
  //     create_time: FSTime::default(),
  //     modify_time: FSTime::default(),
  //   }
  // }

  #[test]
  fn create_builder_by_default() {
      let mut builder = FSResourceBuilder::default();
      builder.fs_type(FSType::default());

      assert_eq!(Some(FSType::Unhandled), builder.fs_type);
  }

  #[test]
  fn create_builder_with_fstype_file() {
      let mut builder = FSResourceBuilder::default();
      builder.fs_type(FSType::File);

      assert_eq!(Some(FSType::File), builder.fs_type);
  }

  #[test]
  fn create_dummy_file_type_of_fs_resource() {
      let res = FSResourceBuilder::default()
        .fs_type(FSType::File)
        .path(PathBuf::from("/home/user/test.txt"))
        .size(0u64)
        .create_time(FSTime::default())
        .modify_time(FSTime::default())
        .build()
        .unwrap();

      assert_eq!(FSType::File, res.fs_type);
      assert_eq!(4, res.path.components().count());
      assert_eq!(0u64, res.size);
  }

  #[test]
  fn create_dummy_directory_type_of_fs_resource() {
      let res = FSResourceBuilder::default()
        .fs_type(FSType::Directory)
        .path(PathBuf::from("/home/user/test.txt"))
        .size(0u64)
        .create_time(FSTime::default())
        .modify_time(FSTime::default())
        .build()
        .unwrap();

      assert_eq!(FSType::Directory, res.fs_type);
      assert_eq!(4, res.path.components().count());
      assert_eq!(0u64, res.size);
  }

  #[test]
  fn create_dummy_link_type_of_fs_resource() {
      let res = FSResourceBuilder::default()
        .fs_type(FSType::Link)
        .path(PathBuf::from("/home/user/test.txt"))
        .size(0u64)
        .create_time(FSTime::default())
        .modify_time(FSTime::default())
        .build()
        .unwrap();

      assert_eq!(FSType::Link, res.fs_type);
      assert_eq!(4, res.path.components().count());
      assert_eq!(0u64, res.size);
  }

}