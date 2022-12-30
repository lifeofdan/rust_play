use actix_web::{get, post, web, App, HttpServer, HttpResponse, Responder, Result};
mod model;


const USERS: [model::User; 2] = [model::User {id: 1, name: "Dan"}, model::User {id: 2, name: "David"}];

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("hello there!")
}

#[get("/users")]
async fn users() -> impl Responder {
    let list_of_users = &USERS.clone();
    HttpResponse::Ok().body(format!("{:?}", list_of_users))
}

#[get("/users/{id}")]
async fn get_user_by_id(path: web::Path<i32>) -> Result<impl Responder> {
    let users = &USERS.clone();
    let id = path.into_inner();
    let mut found: Option<&model::User> = None;
    for user in users {
        if user.id == id {
            found = Some(user);
            break;
        } else {
            found = None;
        }
    }
    Ok(web::Json(*found.unwrap()))
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