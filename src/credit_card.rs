use std::fmt::Write;
use xml;

#[derive(Debug)]
pub struct CreditCard {
    pub cardholder_name: Option<String>,
    pub cvv: Option<String>,
    pub expiration_date: Option<String>,
    pub expiration_month: Option<String>,
    pub expiration_year: Option<String>,
    pub number: Option<String>,
    pub token: Option<String>,
}

impl Default for CreditCard {
    fn default() -> CreditCard {
        CreditCard{
            cardholder_name: None,
            cvv: None,
            expiration_date: None,
            expiration_month: None,
            expiration_year: None,
            number: None,
            token: None,
        }
    }
}

impl ::ToXml for CreditCard {
    fn to_xml(&self, name: Option<&str>) -> String {
        let name = xml::escape(&name.unwrap_or("credit-card"));
        let mut s = String::new();
        write!(s, "<{}>", name).unwrap();
        write_xml!(s, "cardholder-name", self.cardholder_name);
        write_xml!(s, "cvv", self.cvv);
        write_xml!(s, "expiration-date", self.expiration_date);
        write_xml!(s, "expiration-month", self.expiration_month);
        write_xml!(s, "expiration-year", self.expiration_year);
        write_xml!(s, "number", self.number);
        write_xml!(s, "token", self.token);
        write!(s, "</{}>", name).unwrap();
        s
    }
}
