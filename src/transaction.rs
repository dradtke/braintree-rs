pub struct Transaction {
    pub typ: String,
    pub amount: String, // change to a decmial?
    pub credit_card: Option<::credit_card::CreditCard>,
}

impl Default for Transaction {
    fn default() -> Transaction {
        Transaction{
            typ: String::from("sale"),
            amount: String::from("0"),
            credit_card: None,
        }
    }
}
