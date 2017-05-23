//! Bindings to Braintree's API.
//!
//! For those unfamiliar with Braintree or payments processing, [Braintree's
//! homepage](https://www.braintreepayments.com/) is a good place to start to
//! learn more, along with the [developer
//! documentation](https://developers.braintreepayments.com/) which provides a
//! good overview of the available tools and API's.
//!
//! Note that this is an unofficial library, with no direct support from
//! Braintree themselves. The goal is to provide a set of reasonably-complete
//! bindings to core functionality, but naturally a lot of it will be incomplete.
//! Pull requests are welcome!
//!
//! The first thing you'll need to do is create a [sandbox
//! account](https://www.braintreepayments.com/sandbox), which you can use
//! to test your integration without needing to go through the full application
//! process. Once you've created an account, follow [these
//! instructions](https://articles.braintreepayments.com/control-panel/important-gateway-credentials#api-credentials)
//! to retrieve your Merchant ID, Public Key, and Private Key. Once you have those,
//! you should be able to create your first transaction! Naturally you'll need to
//! substitute those three values in for the placeholders below, and it bears
//! repeating that you should _never_ commit those credentials to source control:
//!
//! ```rust
//! extern crate braintree;
//! use braintree::{Braintree, CreditCard, Environment, Error, Transaction};
//!
//! fn main() {
//!     // Create a handle to the Braintree API.
//!     let bt = Braintree::new(
//!         Environment::Sandbox,
//!         "<merchant_id>",
//!         "<public_key>",
//!         "<private_key>",
//!     );
//!
//!     // Attempt to charge the provided credit card $10.
//!     let result = bt.transaction().create(Transaction{
//!         amount: String::from("10.00"),
//!         credit_card: Some(CreditCard{
//!             number: Some(String::from("4111111111111111")),
//!             expiration_date: Some(String::from("10/20")),
//!             ..CreditCard::default()
//!         }),
//!         options: Some(braintree::transaction::Options{
//!             submit_for_settlement: Some(true),
//!             ..braintree::transaction::Options::default()
//!         }),
//!         ..Transaction::default()
//!     });
//!
//!     // Check to see if it worked.
//!     match result {
//!         Ok(transaction) => println!("Created transaction: {}", transaction.id),
//!         Err(Error::Http(err)) => panic!("HTTP-level error: {:?}", err),
//!         Err(Error::Api(err)) => println!("API error: {}", err.message),
//!     }
//! }
//! ```
//!
//! Once you've decided that your integration is good to go live, you'll need
//! to get a separate set of production credentials by signing up on
//! Braintree's main site. Remember to also change `Environment::Sandbox` to
//! `Environment::Production` when you make the switch.
//!
//! # Note on API Design
//!
//! This crate is very much in a pre-alpha state, and as such the design of its
//! API is subject to change. In particular, note that nearly every field
//! defined on a model is an `Option` type. This is to be as explicit as
//! possible about which fields get sent in any given API call, but it also
//! adds some extra noise that may or may not be better than the alternative of
//! only sending values that aren't blank.

extern crate elementtree;
#[macro_use] extern crate hyper;
extern crate hyper_native_tls;
extern crate libflate;
extern crate xml;

macro_rules! write_xml {
    ($s:expr, $elem:expr, $value:expr) => {
        if let Some(ref value) = $value {
            write!($s, "<{}>{}</{}>", $elem, &xml::escape(&value.to_string()), $elem).unwrap();
        }
    }
}


header! { (XApiVersion, "X-ApiVersion") => [u8] }

use std::io::Read;
pub mod address;
pub mod credit_card;
pub mod descriptor;
pub mod customer;
pub mod error;
pub mod transaction;

pub use address::Address as Address;
pub use credit_card::CreditCard as CreditCard;
pub use descriptor::Descriptor as Descriptor;
pub use customer::Customer as Customer;
pub use error::Error as Error;
pub use transaction::Transaction as Transaction;

pub struct Braintree {
    creds: Box<Credentials>,
    client: hyper::Client,
    merchant_url: hyper::Url,
    user_agent: String,
}

impl Braintree {
    pub fn new<S>(env: Environment, merchant_id: S, public_key: S, private_key: S) -> Braintree
        where S: Into<String>
    {
        let ssl = hyper_native_tls::NativeTlsClient::new().unwrap();
        let connector = hyper::net::HttpsConnector::new(ssl);

        let merchant_id = merchant_id.into();
        let public_key = public_key.into();
        let private_key = private_key.into();
        // Calculate some things in advance.
        let merchant_url = hyper::Url::parse(&format!("{}/merchants/{}/", env.base_url(), merchant_id)).unwrap();
        Braintree{
            creds: Box::new(ApiKey{
                       env: env,
                       merchant_id: merchant_id,
                       auth_header: hyper::header::Basic{username: public_key.clone(), password: Some(private_key.clone())},
                       public_key: public_key,
                       private_key: private_key,
                   }),
            client: hyper::Client::with_connector(connector),
            merchant_url: merchant_url,
            user_agent: format!("Braintree Rust {}", env!("CARGO_PKG_VERSION")),
        }
    }

    pub fn transaction(&self) -> TransactionGateway {
        TransactionGateway(self)
    }

    fn execute(&self, method: hyper::method::Method, path: &str, body: Option<&[u8]>) -> hyper::error::Result<hyper::client::response::Response> {
        use hyper::header::{self, Quality, QualityItem};
        use hyper::mime::{Mime, TopLevel, SubLevel};

        let url = self.merchant_url.join(&path).unwrap();

        let mut req = self.client.request(method, url)
            .header(header::ContentType(Mime(TopLevel::Application, SubLevel::Xml, vec![])))
            .header(header::Accept(vec![QualityItem::new(Mime(TopLevel::Application, SubLevel::Xml, vec![]), Quality(1000))]))
            .header(header::AcceptEncoding(vec![QualityItem::new(header::Encoding::Gzip, Quality(1000))]))
            .header(header::UserAgent(self.user_agent.clone()))
            .header(header::Authorization(self.creds.authorization_header()))
            .header(XApiVersion(4));

        if let Some(data) = body {
            req = req.body(hyper::client::Body::BufBody(data, data.len()));
        }

        req.send()
    }

    /// Returns a reader that will correctly decode the response body's data based on its Content-Encoding header.
    fn response_reader(&self, response: hyper::client::response::Response) -> hyper::error::Result<Box<Read>> {
        // TODO: This is written this way in order to appease the borrow checker, but there's probably a better way to do this.
        let headers = response.headers.clone();
        let content_encoding = headers.get::<hyper::header::ContentEncoding>();
        let mut r: Box<Read> = Box::new(response);
        // ???: Use Content-Length somehow to provide a hint to the consumer?
        if let Some(content_encoding) = content_encoding {
            match content_encoding[0] {
                hyper::header::Encoding::Gzip => {
                    r = Box::new(libflate::gzip::Decoder::new(r)?);
                },
                _ => panic!("unsupported content encoding: {}", content_encoding[0]),
            }
        }
        Ok(r)
    }
}

#[derive(Clone, Copy)]
pub enum Environment {
    Sandbox,
    Production,
}

impl Environment {
    fn base_url(&self) -> &str {
        match *self {
            Environment::Sandbox => "https://sandbox.braintreegateway.com",
            Environment::Production => "https://www.braintreegateway.com",
        }
    }
}

trait Credentials {
    fn environment(&self) -> Environment;
    fn merchant_id(&self) -> &str;
    fn authorization_header(&self) -> hyper::header::Basic;
}

#[allow(dead_code)]
struct ApiKey {
    env: Environment,
    merchant_id: String,
    public_key: String,
    private_key: String,
    auth_header: hyper::header::Basic,
}

impl Credentials for ApiKey {
    fn environment(&self) -> Environment { self.env }
    fn merchant_id(&self) -> &str { &self.merchant_id }
    fn authorization_header(&self) -> hyper::header::Basic { self.auth_header.clone() }
}

pub struct TransactionGateway<'a>(&'a Braintree);

impl<'a> TransactionGateway<'a> {
    pub fn create(&self, transaction: transaction::Transaction) -> error::Result<Transaction> {
        let response = self.0.execute(hyper::method::Method::Post, "transactions", Some(transaction.to_xml(None).as_bytes()))?;
        match response.status {
            hyper::status::StatusCode::Created => Ok(Transaction::from(self.0.response_reader(response)?)),
            _ => Err(Error::from(self.0.response_reader(response)?)),
        }
    }
}

trait ToXml {
    fn to_xml(&self, name: Option<&str>) -> String;
}
