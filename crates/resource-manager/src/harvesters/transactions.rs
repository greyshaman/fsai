use crate::harvester::Collectable;

pub struct TransactionsHarvester {
  entry_point: String,
}

impl Collectable for TransactionsHarvester {
  fn collect(&self) {
    unimplemented!("Please implement TransactionHarvester.collect()!!!!");
  }
}

