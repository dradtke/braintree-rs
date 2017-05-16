# braintree-rs
A Rust client library for Braintree

## TODO

1. Replace the `ToXml` trait with a proper XML serializer. Serde currently
   [does not](https://github.com/RReverser/serde-xml-rs/issues/7) support this, but
   once it does we should switch to using that.
