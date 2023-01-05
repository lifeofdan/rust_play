#[test]
pub fn test_users() {
    use crate::api::users_routes::index;
    use crate::repository::{DbConnection, UserRepo};
    use actix_web::web;
    let conn = DbConnection {
        connection: "".to_owned(),
    };
    let user_repo = UserRepo::new();
    let index_result = index;
    let repo_result = web::Json(user_repo.get_users());
}
