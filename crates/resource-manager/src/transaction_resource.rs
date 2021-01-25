use rust_decimal::prelude::*;
use iso_currency::Currency;

#[derive(Builder, Debug)]
#[builder(setter(into))]
pub struct TransactionResource {
  amount: Decimal,
  currency: Currency,
  description: String,
}


impl TransactionResource {
  pub fn amount(&self) -> &Decimal {
    &self.amount
  }

  pub fn set_amount(&mut self, amount: &Decimal) {
    self.amount = *amount;
  }

  pub fn currency(&self) -> &Currency {
    &self.currency
  }

  pub fn set_currency(&mut self, currency: &Currency) {
    self.currency = *currency;
  }

  pub fn description(&self) -> &str {
    &self.description
  }

  pub fn set_description(&mut self, desc: &str) {
    self.description = String::from(desc);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn build_simple_transaction() {
    let mut builder = TransactionResourceBuilder::default();
    builder.amount(Decimal::new(123, 2));
    builder.currency(Currency::RUB);
    builder.description(String::from("test"));
    
    let resource = builder.build().unwrap();

    assert_eq!(resource.amount(), &Decimal::new(123, 2));
    assert_eq!(resource.currency(), &Currency::RUB);
    assert_eq!(resource.description(), &String::from("test"));
  }
}