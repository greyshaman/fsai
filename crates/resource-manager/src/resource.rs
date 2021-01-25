use uuid::Uuid;
use crate::fs_resource::FSResource;
use crate::transaction_resource::TransactionResource;

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

pub struct Resource {
  id: Uuid,
  info: ResourceInfo,
}

impl Identified for Resource {
  fn id(&self) -> Uuid {
    self.id
  }

  fn generate_id() -> Uuid {
    Uuid::new_v4()
  }
}

impl Named for Resource {
  fn name(&self) -> &str {
    self.info.name()
  }
}


impl Resource {
  pub fn new(info: ResourceInfo) -> Resource {
    let id = Self::generate_id();
    Resource { id, info }
  }
}

pub enum ResourceInfo {
  FSResource(FSResource),
  TransactionResource(TransactionResource),
}

impl Named for ResourceInfo {
  fn name(&self) -> &str {
    match self {
      ResourceInfo::FSResource(data) => match data.path().to_str() {
        Some(fs_path) => fs_path,
        None => ""
      }
      ResourceInfo::TransactionResource(trans_data) => trans_data.description()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::fs_resource::{FSTime, FSType, FSResourceBuilder};
  use crate::transaction_resource::{TransactionResource, TransactionResourceParams};
  use iso_currency::Currency;
  use rust_decimal::prelude::*;

  #[test]
  fn create_fs_resource_unit() {
    let unit = Resource::new(
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

  #[test]
  fn resource_name_should_return_description_of_transaction_resource() {
      let transaction_resource_params = TransactionResourceParams {
        amount: Some(Decimal::new(123, 2)),
        currency: Some(Currency::RUB),
        description: Some(String::from("test transaction"))
      };

      let resource = Resource::new(ResourceInfo::TransactionResource(
        TransactionResource::new(transaction_resource_params)
      ));

      assert_eq!(resource.name(), "test transaction");
      assert!(!resource.id().is_nil());
  }
}
