use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub phone: String,
    pub photo: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewUser {
    pub name: String,
    pub phone: String,
    pub photo: Option<String>,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginInfo {
    phone: String,
    password: String,
}