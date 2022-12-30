use super::DbConnection;
use crate::model::{self, User};

pub struct UserRepo {
    pub connection: DbConnection,
}

impl UserRepo {
    // Fake data
    fn users_row() -> Vec<User> {
        return vec![
            User { id: 1, name: "Dan" },
            model::User {
                id: 2,
                name: "David",
            },
        ];
    }

    pub fn get_users(&self) -> Vec<User> {
        return Self::users_row();
    }

    pub fn get_user_by_id(&self, id: i32) -> Option<User> {
        for user in Self::users_row() {
            if user.id == id {
                return Some(user);
            }
        }

        return None;
    }
}
