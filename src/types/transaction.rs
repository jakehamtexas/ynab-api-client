use serde::Deserialize;
use serde::Serialize;
#[derive(Serialize, Deserialize)]
pub struct Subtransaction {
    pub amount: i32,
    pub payee_id: String,
    pub payee_name: String,
    pub category_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub account_id: String,
    pub date: String,
    pub amount: i32,
    pub payee_id: String,
    pub payee_name: String,
    pub category_id: String,
    pub cleared: String,
    pub approved: bool,
    pub subtransactions: Option<Vec<Subtransaction>>,
}

#[derive(Serialize)]
pub struct TransactionRequest {
    pub transaction: Transaction,
}

impl From<&str> for Transaction {
    fn from(s: &str) -> Self {
        serde_json::from_str(s).unwrap()
    }
}
