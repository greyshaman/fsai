mod resource;
mod fs_resource;
mod transaction_resource;
mod harvester;
mod harvesters;

use crate::resource::Resource;
use std::sync::{Arc, Mutex};

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
  use crate::transaction_resource::{TransactionResource, TransactionResourceParams};
  use rust_decimal::prelude::*;

  #[test]
  fn create_manager() {
    let manager = ResourceManager::new();

    assert_eq!(manager.storage.len(), 0);
  }

  #[test]
  fn add_resource() {
    let mut manager = ResourceManager::new();
    let transaction_resource_params = TransactionResourceParams {
      amount: Some(Decimal::new(123, 2)),
      currency: Some(Currency::RUB),
      description: Some(String::from("test transaction"))
    };
    manager.add(Resource::new(
      ResourceInfo::TransactionResource(
        TransactionResource::new(transaction_resource_params)
      )
    ));

    assert_eq!(manager.storage.len(), 1);
  }
}
