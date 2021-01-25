use rust_decimal::prelude::*;
use iso_currency::Currency;

pub struct TransactionResource {
  amount: Decimal,
  currency: Currency,
  description: String,
}

pub struct TransactionResourceParams {
  pub amount: Option<Decimal>,
  pub currency: Option<Currency>,
  pub description: Option<String>,
}


impl TransactionResource {
  pub fn new(tr_args: TransactionResourceParams) -> TransactionResource {
    let amount = match tr_args.amount {
      Some(value) => value,
      None => Decimal::new(0, 2)
    };
    let currency = match tr_args.currency {
      Some(value) => value,
      None => Currency::RUB
    };
    let description = match tr_args.description {
      Some(value) => value,
      None => String::from("")
    };

    TransactionResource { amount, currency, description }
  }

  pub fn amount(&self) -> &Decimal {
    &self.amount
  }

  pub fn set_amount(&mut self, amount: &Decimal) -> &mut TransactionResource {
    self.amount = *amount;
    self
  }

  pub fn currency(&self) -> &Currency {
    &self.currency
  }

  pub fn set_currency(&mut self, currency: &Currency) -> &mut TransactionResource {
    self.currency = *currency;
    self
  }

  pub fn description(&self) -> &str {
    &self.description
  }

  pub fn set_description(&mut self, desc: &str) -> &mut TransactionResource {
    self.description = String::from(desc);
    self
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn build_simple_transaction() {
    let resource = TransactionResource::new(
      TransactionResourceParams {
        amount: Some(Decimal::new(123, 2)),
        currency: None,
        description: Some(String::from("test transaction resource")),
      }
    );

    assert_eq!(resource.amount(), &Decimal::new(123, 2));
    assert_eq!(resource.currency(), &Currency::RUB);
    assert_eq!(resource.description(), &String::from("test transaction resource"));
  }

  #[test]
  fn amount_setter_should_modify_resource() {
    let mut resource = TransactionResource::new(
      TransactionResourceParams {
        amount: Some(Decimal::new(123, 2)),
        currency: None,
        description: Some(String::from("test transaction resource")),
      }
    );

    assert_eq!(resource.amount(), &Decimal::new(123, 2));

    resource.set_amount(&Decimal::new(12345678987654321, 4));

    assert_eq!(resource.amount(), &Decimal::new(12345678987654321, 4));

    assert_eq!(resource.set_amount(
      &Decimal::new(112233445566778899, 2)).amount(), 
      &Decimal::new(112233445566778899, 2)
    );
  }
}