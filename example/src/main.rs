extern crate braintree;
use braintree::{Braintree, CreditCard, Environment, Error, Transaction};

fn main() {
    let bt = Braintree::new(
        Environment::Sandbox,
        std::env::var("MERCHANT_ID").expect("environment variable MERCHANT_ID is not defined"),
        std::env::var("PUBLIC_KEY").expect("environment variable PUBLIC_KEY is not defined"),
        std::env::var("PRIVATE_KEY").expect("environment variable PRIVATE_KEY is not defined"),
    );

    let result = bt.transaction().create(Transaction{
        amount: String::from("13.00"),
        credit_card: Some(CreditCard{
            number: Some(String::from("4111111111111111")),
            expiration_date: Some(String::from("10/20")),
            ..CreditCard::default()
        }),
        options: Some(braintree::transaction::Options{
            submit_for_settlement: Some(true),
            ..braintree::transaction::Options::default()
        }),
        ..Transaction::default()
    });

    match result {
        Ok(response) => println!("{}", response),
        Err(Error::Http(err)) => panic!("HTTP-level error: {:?}", err),
        Err(Error::Api(err)) => println!("API error: {}", err.message),
    }
}
