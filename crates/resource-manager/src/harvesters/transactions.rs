use crate::harvester::Collectable;

pub struct TransactionsHarvester {
  entry_point: String,
}

impl Collectable for TransactionsHarvester {
  fn collect(&self) {
    unimplemented!("Please implement TransactionHarvester.collect()!!!!");
  }

  fn base(&self) -> &str {
    &self.entry_point
  }
}

impl TransactionsHarvester {
  pub fn new(entry_point: &str) -> TransactionsHarvester {
    TransactionsHarvester {
      entry_point: String::from(entry_point),
    }
  }

  pub fn entry_point(&self) -> &str {
    &self.entry_point
  }

  pub fn set_entry_point(&mut self, entry_point: &str) -> &TransactionsHarvester {
    self.entry_point = String::from(entry_point);
    self
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn create_mock_files(timestemp: &str) {
    // 1. create directory

    // 2. create regular file with size by 5 bytes
    // 3. create link to previously created file
  }

  fn remove_mock_files(timestemp: &str) {}

  #[test]
  fn create_transaction_harvester_by_new() {
      let trh = TransactionsHarvester::new("test entry point");

      assert_eq!("test entry point", trh.entry_point());
  }

  #[test]
  fn setter_should_change_entry_point() {
      let mut trh = TransactionsHarvester::new("one");

      trh.set_entry_point("two");
      assert_eq!("two", trh.entry_point());
  }
}
