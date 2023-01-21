use crate::model::user::{NewUser, User};
use reqwest::{header::HeaderMap, Client};

use super::db_connection::DbResponse;

pub struct UserRepo {
    pub connection_url: String,
    pub client: Client,
}

impl UserRepo {
    pub fn new() -> Self {
        let mut connection_header_map = HeaderMap::new();
        connection_header_map.insert(reqwest::header::ACCEPT, "application/json".parse().unwrap());
        connection_header_map.insert("NS", "test".parse().unwrap());
        connection_header_map.insert("DB", "test".parse().unwrap());

        let c = reqwest::Client::builder()
            .default_headers(connection_header_map)
            .build()
            .unwrap();

        UserRepo {
            connection_url: "http://surrealdb:8000/key/users".to_owned(),
            client: c,
        }
    }

    pub async fn get_users(&self) -> Vec<User> {
        let resp = &self
            .client
            .get(&self.connection_url)
            .basic_auth("root", Some("root"))
            .send()
            .await
            .expect("expected a response")
            .text()
            .await
            .expect("expected some text");

        let raw: serde_json::Value = serde_json::from_str(&resp).expect("expect raw");

        let mut users: Vec<User> = Vec::new();

        if &raw[0]["status"] == "OK" {
            let r: DbResponse = serde_json::from_value(raw[0].clone()).expect("to be ok");
            for user in r.result {
                users.push(user);
            }
        }

        users
    }

    pub async fn get_user_by_id(&self, id: &str) -> Option<User> {
        let db_resp = &self
            .client
            .get(&self.connection_url)
            .json(id)
            .basic_auth("root", Some("root"))
            .send()
            .await
            .expect("Expected a response")
            .text()
            .await
            .expect("Expected some text");

        let raw: serde_json::Value =
            serde_json::from_str(&db_resp).expect("Expected raw Database response");

        if &raw[0]["status"] == "OK" {
            let r: DbResponse = serde_json::from_value(raw[0].clone())
                .expect("Expected database response with single user");
            return Some(r.result.first().unwrap().clone());
        }
        return None;
    }

    pub async fn create_user(&mut self, new_user: &NewUser) -> Option<User> {
        let resp = &self
            .client
            .post(&self.connection_url)
            .json(new_user)
            .basic_auth("root", Some("root"))
            .send()
            .await
            .expect("Expected a result")
            .text()
            .await
            .expect("expected some text");

        let raw: serde_json::Value = serde_json::from_str(&resp).expect("expect raw");

        let mut new_user = None;
        if &raw[0]["status"] == "OK" {
            let r: DbResponse = serde_json::from_value(raw[0].clone()).expect("Expected a result");
            new_user = Some(r.result.first().unwrap().clone());
        }

        new_user
    }
}
