use actix_web::{HttpResponse};
use crate::models::transactions::Transaction;
use serde_json::to_string;

pub async fn get_all_transaction() -> HttpResponse {
    let mock_transactions: Vec<Transaction> = vec![
    Transaction {
        title: String::from("Bubble tea"),
        transaction_type: String::from("EXPENSE"),
        category: String::from("Food"),
        amount: 50.0
    },
    Transaction {
        title: String::from("Work"),
        transaction_type: String::from("INCOME"),
        category: String::from("Salary"),
        amount: 2500.0
    }
    ];

    HttpResponse::NotImplemented()
        .body(to_string(&mock_transactions).unwrap())
}