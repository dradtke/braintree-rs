use std::fmt::Write;
use xml;

#[derive(Debug)]
pub struct Customer {
    pub company: Option<String>,
    pub email: Option<String>,
    pub fax: Option<String>,
    pub first_name: Option<String>,
    pub id: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub website: Option<String>,
}

impl Default for Customer {
    fn default() -> Customer {
        Customer{
            company: None,
            email: None,
            fax: None,
            first_name: None,
            id: None,
            last_name: None,
            phone: None,
            website: None,
        }
    }
}

impl ::ToXml for Customer {
    fn to_xml(&self, name: Option<&str>) -> String {
        let name = xml::escape(&name.unwrap_or("customer"));
        let mut s = String::new();
        write!(s, "<{}>", name).unwrap();
        write_xml!(s, "company", self.company);
        write_xml!(s, "email", self.email);
        write_xml!(s, "fax", self.fax);
        write_xml!(s, "first-name", self.first_name);
        write_xml!(s, "id", self.id);
        write_xml!(s, "last-name", self.last_name);
        write_xml!(s, "phone", self.phone);
        write_xml!(s, "website", self.website);
        write!(s, "</{}>", name).unwrap();
        s
    }
}
