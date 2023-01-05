use actix_web::{web, App, HttpServer};
use api::users_routes;
use repository::UserRepo;
use std::sync::Mutex;

mod api;
mod model;
mod repository;
mod tests;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Mutex::new(UserRepo::new())))
            .service(users_routes::index)
            .service(users_routes::get_by_id)
            .service(users_routes::create_user)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
