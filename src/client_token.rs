use elementtree;
use std::convert::From;
use std::io::Read;
use std::fmt::Write;
use xml;

/// A request to retrieve a new client token.
#[derive(Debug)]
pub struct Request {
    /// The identification value for an existing customer. This value only
    /// applies to the Drop-in UI, and is used to display the customer's
    /// saved payment methods.
    pub customer_id: Option<String>,
    /// The merchant account ID that you want to use to create the transactions.
    /// If not specified, your account's default merchant account will be used.
    pub merchant_account_id: Option<String>,
    pub options: Option<Options>,
    /// The version of the client token to generate. The default value is 2,
    /// which is what most of the client SDK's currently use. Verify your
    /// client SDK's supported versions before specifying a different value.
    pub version: u8,
}

impl Default for Request {
    fn default() -> Request {
        Request{
            customer_id: None,
            merchant_account_id: None,
            options: None,
            version: 2,
        }
    }
}

impl ::ToXml for Request {
    fn to_xml(&self, name: Option<&str>) -> String {
        let name = xml::escape(&name.unwrap_or("client-token"));
        let mut s = String::new();
        write!(s, "<{}>", name).unwrap();

        write!(s, "<version type=\"integer\">{}</version>", self.version).unwrap();
        write_xml!(s, "customer-id", self.customer_id);
        write_xml!(s, "merchant-account-id", self.merchant_account_id);
        if let Some(ref options) = self.options { write!(s, "{}", options.to_xml(None)).unwrap(); }

        write!(s, "</{}>", name).unwrap();
        s
    }
}

#[derive(Debug, Default)]
pub struct Options {
    /// Only for use with non-PayPal payment methods and the Drop-in UI. If this
    /// option is passed and the payment method has already been added to the Vault,
    /// the request will fail. This requires that a `customer_id` be specified as well.
    pub fail_on_duplicate_payment_method: Option<bool>,
    /// Make this payment method the customer's default. This requires that a `customer_id`
    /// be specified as well.
    pub make_default: Option<bool>,
    /// Prompt the gateway to verify the card's AVS and CVV information; this behavior
    /// can also be enabled for your entire account from the Control Panel. This requires
    /// that a `customer_id` be specified as well.
    pub verify_card: Option<bool>,
}

impl ::ToXml for Options {
    fn to_xml(&self, name: Option<&str>) -> String {
        let name = xml::escape(&name.unwrap_or("options"));
        let mut s = String::new();
        write!(s, "<{}>", name).unwrap();

        write_xml_type!(s, "fail-on-duplicate-payment-method", "boolean", self.fail_on_duplicate_payment_method);
        write_xml_type!(s, "make-default", "boolean", self.make_default);
        write_xml_type!(s, "verify-card", "boolean", self.verify_card);

        write!(s, "</{}>", name).unwrap();
        s
    }
}

pub struct ClientToken {
    /// The value of the client token.
    pub value: String,
}

impl From<Box<Read>> for ClientToken {
    fn from(xml: Box<Read>) -> ClientToken {
        let root = elementtree::Element::from_reader(xml).unwrap();
        ClientToken{
            value: String::from(root.find("value").unwrap().text()),
        }
    }
}
