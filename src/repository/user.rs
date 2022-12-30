use crate::model;

fn users_row() -> Vec<model::User> {
  let users: Vec<model::User> = vec![model::User {id: 1, name: "Dan"}, model::User {id: 2, name: "David"}];
  return users;
}

pub fn get_users() -> Vec<model::User>{
  return users_row();
}

pub fn get_user_by_id(id: i32) -> Option<model::User> {
  for user in users_row() {
    if user.id == id {
      return Some(user);
    }
  }

  return None
}