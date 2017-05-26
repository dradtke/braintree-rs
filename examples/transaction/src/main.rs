extern crate braintree;
extern crate clap;

use braintree::{Braintree, CreditCard, Environment};
use braintree::transaction;

use std::error::Error;

fn print_transaction(transaction: transaction::Transaction, merchant_id: String) {
    println!("        ID: {}", transaction.id);
    println!("      Type: {:?}", transaction.typ);
    println!("    Amount: {}", transaction.amount);
    println!("  Currency: {}", transaction.currency_iso_code);
    println!("    Status: {:?}", transaction.status);
    println!("       URL: https://sandbox.braintreegateway.com/merchants/{}/transactions/{}", merchant_id, transaction.id);
}

fn main() {
    let merchant_id = std::env::var("MERCHANT_ID").expect("environment variable MERCHANT_ID is not defined");
    let bt = Braintree::new(
        Environment::Sandbox,
        merchant_id.clone(),
        std::env::var("PUBLIC_KEY").expect("environment variable PUBLIC_KEY is not defined"),
        std::env::var("PRIVATE_KEY").expect("environment variable PRIVATE_KEY is not defined"),
    );

    let app_m = clap::App::new("Braintree Example")
        .subcommand(
            clap::SubCommand::with_name("create")
                .arg(clap::Arg::with_name("amount").required(true))
        )
        .subcommand(
            clap::SubCommand::with_name("find")
                .arg(clap::Arg::with_name("transaction_id").required(true))
        )
        .subcommand(
            clap::SubCommand::with_name("void")
                .arg(clap::Arg::with_name("transaction_id").required(true))
        )
        .subcommand(
            clap::SubCommand::with_name("refund")
                .arg(clap::Arg::with_name("transaction_id").required(true))
        )
        .subcommand(
            clap::SubCommand::with_name("settle")
                .arg(clap::Arg::with_name("transaction_id").required(true))
        )
        .get_matches();

    match app_m.subcommand() {
        ("create", Some(sub_m)) => {
            let amount = String::from(sub_m.value_of("amount").unwrap());
            let result = bt.transaction().create(transaction::Request{
                amount: amount,
                credit_card: Some(CreditCard{
                    number: Some(String::from("4111111111111111")),
                    expiration_date: Some(String::from("10/20")),
                    ..Default::default()
                }),
                options: Some(braintree::transaction::Options{
                    submit_for_settlement: Some(true),
                    ..Default::default()
                }),
                ..Default::default()
            });

            match result {
                Ok(transaction) => {
                    println!("\n  Successfully created a transaction! Details to follow:\n");
                    print_transaction(transaction, merchant_id);
                    println!("");
                },
                Err(err) => println!("\nError: {}\n", err.description()),
            }
        },

        ("find", Some(sub_m)) => {
            let transaction_id = String::from(sub_m.value_of("transaction_id").unwrap());
            let result = bt.transaction().find(transaction_id);

            match result {
                Ok(transaction) => {
                    println!("\n  Found a transaction! Details to follow:\n");
                    print_transaction(transaction, merchant_id);
                    println!("");
                },
                Err(err) => println!("\nError: {}\n", err.description()),
            }
        },

        ("void", Some(sub_m)) => {
            let transaction_id = String::from(sub_m.value_of("transaction_id").unwrap());
            let result = bt.transaction().void(transaction_id);

            match result {
                Ok(transaction) => println!("\nSuccessfully voided {}\n", transaction.id),
                Err(err) => println!("\nError: {}\n", err.description()),
            }
        },

        ("refund", Some(sub_m)) => {
            let transaction_id = String::from(sub_m.value_of("transaction_id").unwrap());
            let result = bt.transaction().refund(transaction_id);

            match result {
                Ok(transaction) => {
                    println!("\nSuccessfully refunded {}, details to follow:\n", transaction.id);
                    print_transaction(transaction, merchant_id);
                    println!("");
                },
                Err(err) => println!("\nError: {}\n", err.description()),
            }
        },

        ("settle", Some(sub_m)) => {
            let transaction_id = String::from(sub_m.value_of("transaction_id").unwrap());
            let result = bt.testing().settle(transaction_id);

            match result {
                Ok(transaction) => println!("\nSuccessfully settled {}\n", transaction.id),
                Err(err) => println!("\nError: {}\n", err.description()),
            }
        },

        _ => println!("unknown command"),
    }
}
