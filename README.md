# braintree-rs
A Rust client library for Braintree

## Running the Example

If you clone the repository somewhere, you'll find a program in
`examples/transaction` that can be used to perform various operations on
transactions. Example invocations are:

```sh
$ cargo run -- create <amount> # Create a transaction
$ cargo run -- find <transaction_id> # Find a transaction
$ cargo run -- void <transaction_id> # Void a transaction
$ cargo run -- settle <transaction_id> # Force a transaction into a settled state
$ cargo run -- refund <transaction_id> # Refund a settled transaction
```

## TODO

1. Replace the `ToXml` trait with a proper XML serializer. Serde currently
   [does not](https://github.com/RReverser/serde-xml-rs/issues/7) support this, but
   once it does we should switch to using that.
