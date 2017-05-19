use std::fmt::Write;
use xml;

pub struct Descriptor {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub url: Option<String>,
}

impl Default for Descriptor {
    fn default() -> Descriptor {
        Descriptor{
            name: None,
            phone: None,
            url: None,
        }
    }
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
