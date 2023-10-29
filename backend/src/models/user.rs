use serde::{Deserialize, Serialize};

pub struct User {
    id: String,
    name: String,
    phone: String,
    photo: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewUser {
    name: String,
    phone: String,
    photo: String,
    password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginInfo {
    phone: String,
    password: String,
}