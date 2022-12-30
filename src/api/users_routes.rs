use crate::repository::UserRepo;
use actix_web::{get, web, Responder, Result};

#[get("/users")]
pub async fn users(data: web::Data<UserRepo>) -> Result<impl Responder> {
    Ok(web::Json(data.get_users()))
}

#[get("/users/{id}")]
pub async fn get_user_by_id(
    path: web::Path<i32>,
    data: web::Data<UserRepo>,
) -> Result<impl Responder> {
    let id = path.into_inner();
    Ok(web::Json(data.get_user_by_id(id)))
}
