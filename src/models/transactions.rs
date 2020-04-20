use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct Transaction {
    pub title: String,
    pub transaction_type: String,
    pub category: String,
    pub amount: f64
}