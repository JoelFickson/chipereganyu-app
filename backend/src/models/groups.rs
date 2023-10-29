use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Group {
    id: String,
    name: String,
    total_members: i32,
    admins: Vec<String>,
    group_owner: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewMember {
    pub id: Option<String>,
    pub phone: Option<String>,
}