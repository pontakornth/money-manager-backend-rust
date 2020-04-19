use actix_web::{HttpResponse};
use crate::models::users::User;


pub async fn get_userdata() -> HttpResponse {
    HttpResponse::Ok()
    .content_type("application/json")
    .body(serde_json::to_string(
        &User 
        { username: String::from("most"),
        display_name: String::from("Most")
    }).unwrap())
}

pub async fn change_password() -> HttpResponse {
    HttpResponse::NotImplemented()
    .body("Not implemented")
}

pub async fn delete_account() -> HttpResponse {
    HttpResponse::NotImplemented()
    .body("Not implenmented")
}