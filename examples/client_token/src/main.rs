extern crate braintree;

use braintree::{Braintree, Environment};
use braintree::client_token;

use std::error::Error;

fn main() {
    let merchant_id = std::env::var("MERCHANT_ID").expect("environment variable MERCHANT_ID is not defined");
    let bt = Braintree::new(
        Environment::Sandbox,
        merchant_id.clone(),
        std::env::var("PUBLIC_KEY").expect("environment variable PUBLIC_KEY is not defined"),
        std::env::var("PRIVATE_KEY").expect("environment variable PRIVATE_KEY is not defined"),
    );

    let result = bt.client_token().generate(client_token::Request{
        // Uncomment the following line with a valid Braintree customer id to generate a customer-specific client token.
        // customer_id: Some(String::from("...")),
        ..Default::default()
    });
    match result {
        Ok(client_token) => println!("Client Token: {}", client_token.value),
        Err(err) => println!("\nError: {}\n", err.description()),
    }
}
