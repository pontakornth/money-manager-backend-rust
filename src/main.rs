use actix_web::{HttpServer,HttpResponse,App,web};
use r2d2::Pool;
mod database;

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
            .route("/api", web::get().to(hello))
    })
    .bind("127.0.0.1:7500")?
    .run()
    .await
}