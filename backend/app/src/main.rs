use actix_web::{web, App, HttpServer};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let ctx = web::Data::new(domain_context::AppContext::new(pool));

    HttpServer::new(move || {
        App::new()
            .app_data(ctx.clone())
            .route("/divisions", web::post().to(controller_division::create))
            .route("/users", web::post().to(controller_user::create))
            .route("/books", web::post().to(controller_book::create))
            .route("/books", web::get().to(controller_book::list))
            .route("/books/{id}", web::get().to(controller_book::find))
            .route("/books/{id}/borrow", web::post().to(controller_book::borrow))
            .route("/books/{id}/return", web::post().to(controller_book::r#return))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
