
use crate::harvesters::{FsHarvester, TransactionsHarvester};

pub trait Collectable {
  fn collect(&self);
}

pub enum Harvester {
  FsHarvester(FsHarvester),
  TransactionHarvester(TransactionsHarvester),
}



impl Collectable for Harvester {
  fn collect(&self) {
    match self {
      Harvester::FsHarvester(harvester) => harvester.collect(),
      Harvester::TransactionHarvester(harvester) => harvester.collect(),
    }
  }
}
