use crate::harvester::Collectable;

pub struct FsHarvester {
  entry_point: String,
}

impl Collectable for FsHarvester {
  fn collect(&self) {
    unimplemented!("Please implement FsHarvester.collect!!!");
  }
}