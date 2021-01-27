
use crate::harvesters::{FsHarvester, TransactionsHarvester};

pub trait Collectable {
  fn collect(&self);
  fn base(&self) -> &str;
}

pub enum Harvester {
  FsHarvester(FsHarvester),
  TransactionsHarvester(TransactionsHarvester),
}

pub struct HarvesterFactory {}

impl Collectable for Harvester {
  fn collect(&self) {
    match self {
      Harvester::FsHarvester(harvester) => harvester.collect(),
      Harvester::TransactionsHarvester(harvester) => harvester.collect(),
    }
  }

  fn base(&self) -> &str {
    match self {
      Harvester::FsHarvester(harvester) => harvester.base(),
      Harvester::TransactionsHarvester(harvester) => harvester.base(),
    }
  } 
}

impl HarvesterFactory {
  pub fn create(harvester_type: &str, entry_point: &str) -> Harvester {
    match harvester_type {
      "FS" => Harvester::FsHarvester(FsHarvester::new(entry_point)),
      "Transaction" => Harvester::TransactionsHarvester(TransactionsHarvester::new(entry_point)),
      &_ => unimplemented!("HarvesterFactory is unimplemented for {} type", harvester_type),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_FsHarvester() {
      let fsh = HarvesterFactory::create("FS", "test");

    assert_eq!("test", fsh.base());
  }
}
