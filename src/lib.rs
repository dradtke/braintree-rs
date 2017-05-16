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

use std::io::Read;

header! { (XApiVersion, "X-ApiVersion") => [u8] }

pub mod address;
pub mod credit_card;
pub mod error;
pub mod transaction;

pub use address::Address as Address;
pub use credit_card::CreditCard as CreditCard;
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

    /// Reads the response body into a string, decoding it if necessary based on the Content-Encoding header.
    fn read_response(&self, response: &mut hyper::client::response::Response) -> hyper::error::Result<String> {
        let mut body = match response.headers.get::<hyper::header::ContentLength>() {
            Some(content_length) => Vec::with_capacity(content_length.0 as usize),
            None => vec![],
        };
        response.read_to_end(&mut body)?;
        match response.headers.get::<hyper::header::ContentEncoding>() {
            None => Ok(String::from_utf8(body)?),
            Some(content_encoding) => {
                match content_encoding[0] {
                    hyper::header::Encoding::Gzip => {
                        let mut decoded = vec![];
                        let mut decoder = libflate::gzip::Decoder::new(std::io::Cursor::new(body))?;
                        decoder.read_to_end(&mut decoded)?;
                        Ok(String::from_utf8(decoded)?)
                    },
                    _ => panic!("unsupported content encoding: {}", content_encoding[0]),
                }
            }
        }
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
    pub fn create(&self, transaction: transaction::Transaction) -> error::Result<String> {
        let mut response = self.0.execute(hyper::method::Method::Post, "transactions", Some(transaction.to_xml(None).as_bytes()))?;
        let body = self.0.read_response(&mut response)?;
        match response.status {
            hyper::status::StatusCode::Created => Ok(body),
            _ => Err(Error::Application(body)),
        }
    }
}

trait ToXml {
    fn to_xml(&self, name: Option<&str>) -> String;
}
