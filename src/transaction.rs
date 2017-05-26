use elementtree;
use std::convert::From;
use std::collections::HashMap;
use std::io::Read;
use std::fmt::Write;
use xml;

/// A record containing transaction details.
#[derive(Debug)]
pub struct Transaction {
    pub id: String,
    pub typ: Type,
    pub amount: String, // change to a decmial?
    pub currency_iso_code: String,
    pub status: Status,
}

impl From<Box<Read>> for Transaction {
    fn from(xml: Box<Read>) -> Transaction {
        let root = elementtree::Element::from_reader(xml).unwrap();
        Transaction{
            id: String::from(root.find("id").unwrap().text()),
            typ: Type::from(String::from(root.find("type").unwrap().text())),
            amount: String::from(root.find("amount").unwrap().text()),
            currency_iso_code: String::from(root.find("currency-iso-code").unwrap().text()),
            status: Status::from(String::from(root.find("status").unwrap().text())),
        }
    }
}

/// A record detailing a new transaction request.
///
/// Since you probably won't be using all of these fields each time,
/// you'll want to use the `Default` trait to fill it out:
///
/// ```rust
/// transaction::Request{
///     amount: String::from("10.00"),
///     ..Default::default()
/// }
/// ```
#[derive(Debug, Default)]
pub struct Request {
    pub typ: Type,
    pub amount: String, // change to a decmial?
    pub order_id: Option<String>,
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

impl ::ToXml for Request {
    fn to_xml(&self, name: Option<&str>) -> String {
        let name = xml::escape(&name.unwrap_or("transaction"));
        let mut s = String::new();
        write!(s, "<{}>", name).unwrap();

        write!(s, "<type>{}</type>", String::from(self.typ)).unwrap();
        write!(s, "<amount>{}</amount>", xml::escape(&self.amount)).unwrap();
        write_xml!(s, "order-id", self.order_id);
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

#[derive(Copy, Clone, Debug)]
pub enum Type {
    Sale,
    Credit,
}

impl Default for Type {
    fn default() -> Type {
        Type::Sale
    }
}

impl From<String> for Type {
    fn from(s: String) -> Type {
        match s.as_ref() {
            "sale" => Type::Sale,
            "credit" => Type::Credit,
            _ => panic!("unknown transaction type: {}", s),
        }
    }
}

impl From<Type> for String {
    fn from(t: Type) -> String {
        match t {
            Type::Sale => String::from("sale"),
            Type::Credit => String::from("credit"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Status {
    AuthorizationExpired,
    Authorizing,
    Authorized,
    GatewayRejected,
    Failed,
    ProcessorDeclined,
    Settled,
    SettlementConfirmed,
    SettlementDeclined,
    SettlementPending,
    Settling,
    SubmittedForSettlement,
    Voided,
    Unrecognized,
}

impl From<String> for Status {
    fn from(s: String) -> Status {
        match s.as_ref() {
            "authorization_expired" => Status::AuthorizationExpired,
            "authorizing" => Status::Authorizing,
            "authorized" => Status::Authorized,
            "gateway_rejected" => Status::GatewayRejected,
            "failed" => Status::Failed,
            "processor_declined" => Status::ProcessorDeclined,
            "settled" => Status::Settled,
            "settlement_confirmed" => Status::SettlementConfirmed,
            "settlement_declined" => Status::SettlementDeclined,
            "settlement_pending" => Status::SettlementPending,
            "settling" => Status::Settling,
            "submitted_for_settlement" => Status::SubmittedForSettlement,
            "voided" => Status::Voided,
            "unrecognized" => Status::Unrecognized,
            _ => panic!("unknown transaction status: {}", s),
        }
    }
}

impl From<Status> for String {
    fn from(s: Status) -> String {
        match s {
            Status::AuthorizationExpired => String::from("authorization_expired"),
            Status::Authorizing => String::from("authorizing"),
            Status::Authorized => String::from("authorized"),
            Status::GatewayRejected => String::from("gateway_rejected"),
            Status::Failed => String::from("failed"),
            Status::ProcessorDeclined => String::from("processor_declined"),
            Status::Settled => String::from("settled"),
            Status::SettlementConfirmed => String::from("settlement_confirmed"),
            Status::SettlementDeclined => String::from("settlement_declined"),
            Status::SettlementPending => String::from("settlement_pending"),
            Status::Settling => String::from("settling"),
            Status::SubmittedForSettlement => String::from("submitted_for_settlement"),
            Status::Voided => String::from("voided"),
            Status::Unrecognized => String::from("unrecognized"),
        }
    }
}
