use elementtree;
use std::convert::From;
use std::collections::HashMap;
use std::io::Read;
use std::fmt::Write;
use xml;

#[derive(Debug, Default)]
pub struct Transaction {
    pub id: String,
    pub typ: TransactionType,
    pub amount: String, // change to a decmial?
    pub billing_address_id: Option<String>,
    pub billing: Option<::address::Address>,
    pub credit_card: Option<::credit_card::CreditCard>,
    pub custom_fields: HashMap<String, String>,
    pub customer: Option<::customer::Customer>,
    pub customer_id: Option<String>,
    pub descriptor: Option<::descriptor::Descriptor>,
    pub options: Option<Options>,
    pub payment_method_nonce: Option<String>,
    pub payment_method_token: Option<String>,
    pub purchase_order_number: Option<String>,
    pub recurring: Option<bool>,
    pub service_fee_amount: Option<String>,
    pub shipping: Option<::address::Address>,
    pub shipping_address_id: Option<String>,
    pub tax_amount: Option<String>,
    pub tax_exempt: Option<bool>,
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

        write_xml!(s, "billing-address-id", self.billing_address_id);

        if let Some(ref billing) = self.billing { write!(s, "{}", billing.to_xml(Some("billing"))).unwrap(); }

        if let Some(ref credit_card) = self.credit_card { write!(s, "{}", credit_card.to_xml(None)).unwrap(); }

        if !self.custom_fields.is_empty() {
            write!(s, "<custom-fields>").unwrap();
            for (k, v) in &self.custom_fields {
                let k = xml::escape(k);
                let v = xml::escape(v);
                write!(s, "<{}>{}</{}>", k, v, k).unwrap();
            }
            write!(s, "</custom-fields>").unwrap();
        }

        if let Some(ref customer) = self.customer { write!(s, "{}", customer.to_xml(None)).unwrap(); }

        write_xml!(s, "customer-id", self.customer_id);

        if let Some(ref descriptor) = self.descriptor { write!(s, "{}", descriptor.to_xml(None)).unwrap(); }

        if let Some(ref options) = self.options { write!(s, "{}", options.to_xml(None)).unwrap(); }

        write_xml!(s, "payment-method-nonce", self.payment_method_nonce);
        write_xml!(s, "payment-method-token", self.payment_method_token);
        write_xml!(s, "purchase-order-number", self.purchase_order_number);
        write_xml!(s, "recurring", self.recurring);
        write_xml!(s, "service-fee-amount", self.recurring);

        if let Some(ref shipping) = self.shipping { write!(s, "{}", shipping.to_xml(Some("shipping"))).unwrap(); }

        write_xml!(s, "shipping-address-id", self.shipping_address_id);
        write_xml!(s, "tax-amount", self.tax_amount);
        write_xml!(s, "tax-exempt", self.tax_exempt);

        write!(s, "</{}>", name).unwrap();
        s
    }
}

impl From<Box<Read>> for Transaction {
    fn from(xml: Box<Read>) -> Transaction {
        let root = elementtree::Element::from_reader(xml).unwrap();
        Transaction{
            id: String::from(root.find("id").unwrap().text()),
            typ: TransactionType::from(String::from(root.find("type").unwrap().text())),
            amount: String::from(root.find("amount").unwrap().text()),
            // TODO: This is incredibly incomplete, but may remain that way until the transition to serde.
            ..Transaction::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct Options {
    pub add_billing_address_to_payment_method: Option<bool>,
    pub hold_in_escrow: Option<bool>,
    // pub paypal: Option<PayPalOptions>,
    pub skip_avs: Option<bool>,
    pub skip_cvv: Option<bool>,
    pub store_in_vault: Option<bool>,
    pub store_in_vault_on_success: Option<bool>,
    pub store_shipping_address_in_vault: Option<bool>,
    pub submit_for_settlement: Option<bool>,
}

impl ::ToXml for Options {
    fn to_xml(&self, name: Option<&str>) -> String {
        let name = xml::escape(&name.unwrap_or("options"));
        let mut s = String::new();
        write!(s, "<{}>", name).unwrap();

        write_xml!(s, "add-billing-address-to-payment-method", self.add_billing_address_to_payment_method);
        write_xml!(s, "hold-in-escrow", self.hold_in_escrow);
        // write_xml!(s, "paypal", self.paypal);
        write_xml!(s, "skip-avs", self.skip_avs);
        write_xml!(s, "skip-cvv", self.skip_cvv);
        write_xml!(s, "store-in-vault", self.store_in_vault);
        write_xml!(s, "store-in-vault-on-success", self.store_in_vault_on_success);
        write_xml!(s, "store-shipping-address-in-vault", self.store_shipping_address_in_vault);
        write_xml!(s, "submit-for-settlement", self.submit_for_settlement);

        write!(s, "</{}>", name).unwrap();
        s
    }
}

// TODO: implement this and add it to Options above
// pub struct PayPalOptions {
//     
// }

#[derive(Debug)]
pub enum TransactionType {
    Sale,
    Refund,
}

impl Default for TransactionType {
    fn default() -> TransactionType {
        TransactionType::Sale
    }
}

impl From<String> for TransactionType {
    fn from(s: String) -> TransactionType {
        match s.as_ref() {
            "sale" => TransactionType::Sale,
            "refund" => TransactionType::Refund,
            _ => panic!("unknown transaction type: {}", s),
        }
    }
}
