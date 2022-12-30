use actix_web::{web, App, HttpServer};
use api::{get_user_by_id, users};
use repository::{DbConnection, UserRepo};

mod api;
mod model;
mod repository;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let conn = DbConnection {
        connection: "".to_owned(),
    };
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(UserRepo {
                connection: conn.clone(),
            }))
            .service(users)
            .service(get_user_by_id)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
