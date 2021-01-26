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

impl FSTime {
  // setters with verification input date.
  // Date from parameter should be in past.
  // If param in future it will return error in case when parameter in future
  pub fn for_past(time: &SystemTime) -> Result<FSTime, &str> {
    if time > &SystemTime::now() {
      Err("input parameter cannot be in future")
    } else {
      Ok(FSTime { time: *time })
    }
  }

  // Parameter's date should be in future,
  // in case when parameter in past will be returned error 
  pub fn for_future(time: &SystemTime) -> Result<FSTime, &str> {
    if time < &SystemTime::now() {
      Err("input parameter cannot be in past")
    } else {
      Ok(FSTime { time: *time })
    }
  }
}


#[derive(Debug, Clone)]
pub struct FSResource {
  fs_type: FSType,
  path: PathBuf,
  size: u64,
  create_time: FSTime,
  modify_time: FSTime,
}

pub struct FSResourceParams {
  pub fs_type: Option<FSType>,
  pub path: Option<PathBuf>,
  pub size: Option<u64>,
  pub create_time: Option<FSTime>, 
  pub modify_time: Option<FSTime>,
}

impl FSResource {
  pub fn new(params: FSResourceParams) -> FSResource {
    let fs_type = match params.fs_type {
      Some(value) => value,
      None => FSType::Unhandled,
    };

    let path = match params.path {
      Some(value) => value,
      None => PathBuf::from("."),
    };

    let size = match params.size {
      Some(value) => value,
      None => 0,
    };

    let present_or_past_fs_time = |fs_time_suggession: Option<FSTime>| -> FSTime {
      let now = SystemTime::now();
      let time = match fs_time_suggession {
        Some(value) => if value.time >= now {
          now
        } else {
          value.time
        },
        None => now
      };
      FSTime { time }
    }; 

    let create_time = present_or_past_fs_time(params.create_time);

    let modify_time = present_or_past_fs_time(params.modify_time);

    FSResource { fs_type, path, size, create_time, modify_time }
  }

  pub fn fs_type(&self) -> &FSType {
    &self.fs_type
  }

  pub fn set_fs_type(&mut self, fst_value: &FSType) -> &mut FSResource {
    self.fs_type = *fst_value;
    self
  }

  pub fn path(&self) -> &PathBuf {
    &self.path
  }

  pub fn set_path(&mut self, path_str: &str) -> &mut FSResource {
    self.path = PathBuf::from(path_str);
    self
  }

  pub fn size(&self) -> &u64 {
    &self.size
  }

  pub fn set_size(&mut self, size: &u64) -> &mut FSResource {
    self.size = *size;
    self
  }

  pub fn create_time(&self) -> &SystemTime {
    &self.create_time.time
  }

  pub fn set_create_time(&mut self, time: &SystemTime) -> &mut FSResource {
    self.create_time.time = *time;
    self
  }

  pub fn modify_time(&self) -> &SystemTime {
    &self.modify_time.time
  }

  pub fn set_modify_time(&mut self, time: &SystemTime) -> &mut FSResource {
    self.modify_time.time = *time;
    self
  }
}

impl Default for FSResource {
  fn default() -> FSResource {
    let now = SystemTime::now();
    FSResource {
      fs_type: FSType::Unhandled,
      path: PathBuf::from("."),
      size: 0,
      create_time: FSTime { time: now },
      modify_time: FSTime { time: now },
    }
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
  fn create_resource_by_default() {
      let resource = FSResource::default();

      assert_eq!(FSType::Unhandled, *resource.fs_type());
  }

  #[test]
  fn create_resource_with_fstype_file() {
      let mut resource = FSResource::default();
      resource.set_fs_type(&FSType::File);

      assert_eq!(FSType::File, *resource.fs_type());
  }

  #[test]
  fn create_dummy_file_type_of_fs_resource() {
      let mut res = FSResource::default();

      res.set_fs_type(&FSType::File)
        .set_path("/home/user/test.txt")
        .set_size(&0u64)
        .set_create_time(&SystemTime::now())
        .set_modify_time(&SystemTime::now());

      assert_eq!(&FSType::File, res.fs_type());
      assert_eq!(4, res.path.components().count());
      assert_eq!(0u64, *res.size());
  }

  #[test]
  fn create_dummy_directory_type_of_fs_resource() {
      let mut res = FSResource::default();
      res.set_fs_type(&FSType::Directory)
        .set_path("/home/user/test.txt")
        .set_size(&0u64)
        .set_create_time(&SystemTime::now())
        .set_modify_time(&SystemTime::now());

      assert_eq!(FSType::Directory, res.fs_type);
      assert_eq!(4, res.path.components().count());
      assert_eq!(0u64, res.size);
  }

  #[test]
  fn create_dummy_link_type_of_fs_resource() {
      let mut res = FSResource::default();
      res.set_fs_type(&FSType::Link)
        .set_path("/home/user/test.txt")
        .set_size(&0u64)
        .set_create_time(&SystemTime::now())
        .set_modify_time(&SystemTime::now());

      assert_eq!(FSType::Link, res.fs_type);
      assert_eq!(4, res.path.components().count());
      assert_eq!(0u64, res.size);
  }

}