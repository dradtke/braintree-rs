extern crate braintree;

fn main() {
    let bt = braintree::Braintree::new(
        braintree::Environment::Sandbox,
        std::env::var("MERCHANT_ID").expect("environment variable MERCHANT_ID is not defined"),
        std::env::var("PUBLIC_KEY").expect("environment variable PUBLIC_KEY is not defined"),
        std::env::var("PRIVATE_KEY").expect("environment variable PRIVATE_KEY is not defined"),
    );

    println!("creating transaction");
    match bt.transaction().create(braintree::Transaction{
        amount: String::from("13.00"),
        credit_card: Some(braintree::CreditCard{
            number: Some(String::from("4111111111111111")),
            expiration_date: Some(String::from("10/18")),
            ..braintree::CreditCard::default()
        }),
        ..braintree::Transaction::default()
    }) {
        Ok(response) => println!("{}", response),
        Err(err) => println!("error: {:?}", err),
    }
}
