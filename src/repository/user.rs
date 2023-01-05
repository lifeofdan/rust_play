use super::DbConnection;
use crate::model::user::User;
pub struct UserRepo {
    pub connection: DbConnection,
    pub users: Vec<User>,
}

impl UserRepo {
    // Fake data
    pub fn new() -> Self {
        UserRepo {
            connection: DbConnection {
                connection: "".to_owned(),
            },
            users: vec![
                User {
                    id: 1,
                    name: "Dan".to_owned(),
                },
                User {
                    id: 2,
                    name: "David".to_owned(),
                },
            ],
        }
    }

    pub fn get_users(&self) -> Vec<User> {
        let users = self.users.clone();
        return users.to_vec().clone();
    }

    pub fn get_user_by_id(&self, id: i32) -> Option<User> {
        for user in &self.users.clone() {
            if user.id == id {
                return Some(user.clone());
            }
        }

        return None;
    }

    pub fn create_user(&mut self, name: &str) -> User {
        let new_user = User {
            id: self.users.last().unwrap().id + 1,
            name: name.to_owned(),
        };

        self.users.push(new_user);

        let last_user = self.users.last().unwrap().clone();
        last_user
    }
}
