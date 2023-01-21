use crate::{model::user::NewUser, repository::UserRepo};
use actix_web::{get, post, web, Responder, Result};
use std::sync::Mutex;

#[get("/users")]
pub async fn index(data: web::Data<Mutex<UserRepo>>) -> Result<impl Responder> {
    // web::Json(data.lock().unwrap().get_users())
    let res = web::Json(data.lock().unwrap().get_users().await);
    Ok(res)
}

#[post("/users")]
pub async fn create_user(
    req_body: web::Json<NewUser>,
    data: web::Data<Mutex<UserRepo>>,
) -> Result<impl Responder> {
    let res = web::Json(data.lock().unwrap().create_user(&req_body).await);
    Ok(res)
}

#[get("/users/{id}")]
pub async fn get_by_id(
    path: web::Path<String>,
    data: web::Data<Mutex<UserRepo>>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    let res = web::Json(data.lock().unwrap().get_user_by_id(id.as_str()).await);
    Ok(res)
}
