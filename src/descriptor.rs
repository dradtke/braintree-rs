use std::fmt::Write;
use xml;

/// A record that describes what your customers will see on their statement
/// when they make a purchase through your application.
///
/// For more information, refer to [Braintree's
/// articles](https://articles.braintreepayments.com/control-panel/transactions/descriptors)
/// .
#[derive(Debug, Default)]
pub struct Descriptor {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub url: Option<String>,
}

impl ::ToXml for Descriptor {
    fn to_xml(&self, name: Option<&str>) -> String {
        let name = xml::escape(&name.unwrap_or("descriptor"));
        let mut s = String::new();
        write!(s, "<{}>", name).unwrap();
        write_xml!(s, "name", self.name);
        write_xml!(s, "phone", self.phone);
        write_xml!(s, "url", self.url);
        write!(s, "</{}>", name).unwrap();
        s
    }
}
