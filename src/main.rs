use actix_web::{get, post, web, App, HttpServer, HttpResponse, Responder, Result};
mod model;
mod repository;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("hello there!")
}

#[get("/users")]
async fn users() -> impl Responder {
    HttpResponse::Ok().body(format!("{:?}", repository::get_users()))
}

#[get("/users/{id}")]
async fn get_user_by_id(path: web::Path<i32>) -> Result<impl Responder> {
    let id = path.into_inner();
    Ok(web::Json(repository::get_user_by_id(id)))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(index).service(echo).service(get_user_by_id)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}