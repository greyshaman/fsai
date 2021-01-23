pub trait Versioned {
  fn version(&self) -> &u64;

  fn modification_time(&self, version: &u64) -> &SystemTime;

  // fn modify(&mut self, info: &ResourceInfo) -> &Version; // TODO: change info to serializable diff object
}

pub struct Version {
  pub version: u64,
  pub modification_time: SystemTime,
}