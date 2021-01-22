#[macro_use]
extern crate derive_builder;

mod fs_resource;

use fs_resource::FSResource;
use uuid::Uuid;
use std::time::SystemTime;

/// The Indetified trait collected functionality regarding unique idetificator 
/// aka ID. Actualy to rich uniqueness we need some BIG number and we already have it
/// It is Uuid
pub trait Identified {
  fn id(&self) -> Uuid;

  fn generate_id() -> Uuid;
}

pub trait Named {
  fn name(&self) -> &str {
    ""
  }

  // fn set_name(&mut self, name: &str); // Not yet needed
}

pub trait Versioned {
  fn version(&self) -> &u64;

  fn modification_time(&self, version: &u64) -> &SystemTime;

  // fn modify(&mut self, info: &ResourceInfo) -> &Version; // TODO: change info to serializable diff object
}

pub struct Version {
  pub version: u64,
  pub modification_time: SystemTime,
}

pub struct ResourceUnit {
  id: Uuid,
  info: ResourceInfo,
}

impl Identified for ResourceUnit {
  fn id(&self) -> Uuid {
    self.id
  }

  fn generate_id() -> Uuid {
    Uuid::new_v4()
  }
}

impl Named for ResourceUnit {
  fn name(&self) -> &str {
    self.info.name()
  }
}

impl ResourceUnit {
  pub fn new(info: ResourceInfo) -> ResourceUnit {
    let id = Self::generate_id();
    ResourceUnit { id, info }
  }
}

pub enum ResourceInfo {
  FSResource(FSResource),
}

impl Named for ResourceInfo {
  fn name(&self) -> &str {
    match self {
      ResourceInfo::FSResource(data) => match data.path().to_str() {
        Some(fs_path) => fs_path,
        None => ""
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use fs_resource::{FSTime, FSType, FSResourceBuilder};

  #[test]
  fn create_fs_resource_unit() {
    let unit = ResourceUnit::new(
      ResourceInfo::FSResource(
        FSResourceBuilder::default()
          .fs_type(FSType::File)
          .path("/path/to/test.file")
          .size(1u64)
          .create_time(FSTime::default())
          .modify_time(FSTime::default())
          .build()
          .unwrap()
      )
    );

    assert_eq!("/path/to/test.file", unit.name());
    assert!(!unit.id().is_nil());
  }
}
