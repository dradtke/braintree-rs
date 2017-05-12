pub struct CreditCard {
    pub number: String,
    pub expiration_date: String,
}

impl Default for CreditCard {
    fn default() -> CreditCard {
        CreditCard{
            number: String::from(""),
            expiration_date: String::from(""),
        }
    }
}
