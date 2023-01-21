use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

use crate::model::user::User;

#[derive(Clone)]
pub struct DbConnection {
    pub connection: HeaderMap,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DbResponse {
    pub result: Vec<User>,
    pub status: String,
    pub time: String,
}
