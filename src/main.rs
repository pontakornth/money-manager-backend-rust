use actix_web::{HttpServer,HttpResponse,App,web};
use r2d2::Pool;
mod database;
mod controllers;
mod models;
use controllers::transactions;
use controllers::users;
async fn hello() -> HttpResponse {
    HttpResponse::NotImplemented()
        .body("Not implemented yet")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Database.db is not commited in repo. You can create it yourself.
    let manager = database::connect("database.db");
    let pool = Pool::new(manager).unwrap();
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/hello", web::get().to(hello))
            .service(web::scope("/api/users")
                .route("/change_password", web::post().to(users::change_password))
                .route("/{id}", web::get().to(users::get_userdata))
                .route("/{id}",web::delete().to(users::delete_account))
            )
            .service(web::scope("/api/transactions")
                .route("", web::get().to(transactions::get_all_transaction))
            )
    })
    .bind("127.0.0.1:7500")?
    .run()
    .await
}