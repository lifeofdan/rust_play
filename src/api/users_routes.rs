use crate::repository::UserRepo;
use actix_web::{get, post, web, Responder, Result};
use serde::Deserialize;
use std::sync::Mutex;

#[derive(Deserialize)]
pub struct Foo {
    foo: String,
}

#[get("/users")]
pub async fn index(data: web::Data<Mutex<UserRepo>>) -> Result<impl Responder> {
    Ok(web::Json(data.lock().unwrap().get_users()))
}

#[post("/users")]
pub async fn create_user(
    req_body: web::Json<Foo>,
    data: web::Data<Mutex<UserRepo>>,
) -> Result<impl Responder> {
    let name = &req_body.foo;
    let new_user = data.lock().unwrap().create_user(name);
    Ok(web::Json(new_user))
}

#[get("/users/{id}")]
pub async fn get_by_id(
    path: web::Path<i32>,
    data: web::Data<Mutex<UserRepo>>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    Ok(web::Json(data.lock().unwrap().get_user_by_id(id)))
}
