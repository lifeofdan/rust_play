use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct User {
    pub id: i32,
    pub name: &'static str
}