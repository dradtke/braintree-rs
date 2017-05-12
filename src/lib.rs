#[macro_use] extern crate hyper;
extern crate hyper_native_tls;
extern crate xml;

use std::io::Read;

header! { (XApiVersion, "X-ApiVersion") => [u8] }

pub mod credit_card;
pub mod transaction;

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
            .header(XApiVersion(3));

        if let Some(data) = body {
            req = req.body(hyper::client::Body::BufBody(data, data.len()));
        }

        req.send()
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
    pub fn create(&self, transaction: transaction::Transaction) -> hyper::error::Result<Vec<u8>> {
        let mut raw = String::new();
        raw.push_str("<transaction>");
        raw.push_str("<type>"); raw.push_str(&xml::escape(&transaction.typ)); raw.push_str("</type>");
        raw.push_str("<amount>"); raw.push_str(&xml::escape(&transaction.amount)); raw.push_str("</amount>");
        if let Some(credit_card) = transaction.credit_card {
            raw.push_str("<credit_card>");
            raw.push_str("<number>"); raw.push_str(&xml::escape(&credit_card.number)); raw.push_str("</number>");
            raw.push_str("<expiration-date>"); raw.push_str(&xml::escape(&credit_card.expiration_date)); raw.push_str("</expiration-date>");
            raw.push_str("</credit_card>");
        }
        raw.push_str("</transaction>");

        let mut response = self.0.execute(hyper::method::Method::Post, "transactions", Some(raw.as_bytes()))?;
        match response.headers.get::<hyper::header::ContentType>() {
            Some(content_type) => println!("content type: {}", content_type),
            None => println!("no content type"),
        }
        let mut buffer = match response.headers.get::<hyper::header::ContentLength>() {
            Some(content_length) => Vec::with_capacity(content_length.0 as usize),
            None => vec![],
        };
        response.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}
