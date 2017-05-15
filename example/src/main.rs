extern crate braintree;

fn main() {
    let bt = braintree::Braintree::new(
        braintree::Environment::Sandbox,
        "<merchant_id>",
        "<public_key>",
        "<private_key>",
    );

    println!("creating transaction");
    match bt.transaction().create(braintree::transaction::Transaction{
        typ: String::from("sale"),
        amount: String::from("13.00"),
        credit_card: Some(braintree::credit_card::CreditCard{
            number: String::from("4111111111111111"),
            expiration_date: String::from("10/18"),
        }),
    }) {
        Ok(response) => println!("{}", response),
        Err(err) => panic!("error: {}", err),
    }
}
