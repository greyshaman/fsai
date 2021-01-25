#[macro_use]
extern crate derive_builder;

mod resource;
mod fs_resource;
mod transaction_resource;

use crate::resource::Resource;
use crate::transaction_resource::{TransactionResource, TransactionResourceBuilder};
use std::sync::{Arc, Mutex};
use rust_decimal::prelude::*;
use iso_currency::Currency;

pub struct ResourceManager {
  // TODO: channel to communicate with  other components
  storage: Vec<Arc<Mutex<Resource>>>,
}

impl ResourceManager {
  pub fn new() -> ResourceManager {
    ResourceManager {
      storage: Vec::new()
    }
  }

  pub fn add(&mut self, resource: Resource) {
    self.storage.push(Arc::new(Mutex::new(resource)));
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use iso_currency::Currency;
  use crate::resource::ResourceInfo;

  #[test]
  fn create_manager() {
    let manager = ResourceManager::new();

    assert_eq!(manager.storage.len(), 0);
  }

  #[test]
  fn add_resource() {
    let mut manager = ResourceManager::new();
    manager.add(Resource::new(
      ResourceInfo::TransactionResource(
        TransactionResourceBuilder::default()
          .amount(Decimal::new(123, 2))
          .currency(Currency::RUB)
          .description(String::from("test transaction"))
          .build()
          .unwrap()
      )
    ));

    assert_eq!(manager.storage.len(), 1);
  }
}
