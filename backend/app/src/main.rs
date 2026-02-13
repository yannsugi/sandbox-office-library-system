use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let ctx = web::Data::new(domain_context::AppContext::new(pool));

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(ctx.clone())
            .route("/divisions", web::post().to(controller_division::create))
            .route("/divisions", web::get().to(controller_division::list))
            .route("/users", web::post().to(controller_user::create))
            .route("/users", web::get().to(controller_user::list))
            .route("/books", web::post().to(controller_book::create))
            .route("/books", web::get().to(controller_book::list))
            .route("/books/{id}", web::get().to(controller_book::find))
            .route("/books/{id}/borrow", web::post().to(controller_book::borrow))
            .route("/books/{id}/return", web::post().to(controller_book::r#return))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
