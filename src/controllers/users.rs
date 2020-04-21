use actix_web::{HttpResponse,web,Responder};
use crate::models::users::{User,get_user_by_id};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Error;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct IdPath {
    pub id: i64
}

pub async fn get_userdata(db: web::Data<Pool<SqliteConnectionManager>>,param: web::Path<IdPath>) -> impl Responder {
    let conn = db.get().unwrap();
    match get_user_by_id(&conn, param.id) {
        Ok(user) => {
            HttpResponse::Found()
                .content_type("application/json")
                .body(serde_json::to_string(&user).unwrap())
        },
        Err(Error::QueryReturnedNoRows) => {
            HttpResponse::NotFound()
                .content_type("application/json")
                .body("{status:404}")
        },
        Err(x) => {
            HttpResponse::InternalServerError()
                .body(format!("{}",x))
        }
    }
}

pub async fn change_password() -> HttpResponse {
    HttpResponse::NotImplemented()
    .body("Not implemented")
}

pub async fn delete_account() -> HttpResponse {
    HttpResponse::NotImplemented()
    .body("Not implenmented")
}
