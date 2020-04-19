use serde::{Serialize,Deserialize};

pub enum TransactionType = {
    Income,
    Expense,

}

pub struct Transaction {
    pub title: String,
    pub type: TransactionType,
    pub category: String,
    pub amount: f64
}