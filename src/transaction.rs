use std::fmt::Write;
use xml;

pub struct Transaction {
    pub typ: TransactionType,
    pub amount: String, // change to a decmial?
    // TODO: billing, etc.
    pub billing: Option<::address::Address>,
    pub credit_card: Option<::credit_card::CreditCard>,
}

pub enum TransactionType {
    Sale,
    Refund,
}

impl Default for Transaction {
    fn default() -> Transaction {
        Transaction{
            typ: TransactionType::Sale,
            amount: String::from("0"),
            billing: None,
            credit_card: None,
        }
    }
}

impl ::ToXml for Transaction {
    fn to_xml(&self, name: Option<&str>) -> String {
        let name = xml::escape(&name.unwrap_or("transaction"));
        let mut s = String::new();
        write!(s, "<{}>", name).unwrap();
        write!(s, "<{}>{}</{}>", "type", match self.typ {
            TransactionType::Sale => "sale",
            TransactionType::Refund => "refund",
        }, "type").unwrap();
        write!(s, "<amount>{}</amount>", xml::escape(&self.amount)).unwrap();
        if let Some(ref billing) = self.billing { write!(s, "{}", billing.to_xml(Some("billing"))).unwrap(); }
        if let Some(ref credit_card) = self.credit_card { write!(s, "{}", credit_card.to_xml(None)).unwrap(); }
        write!(s, "</{}>", name).unwrap();
        s
    }
}
