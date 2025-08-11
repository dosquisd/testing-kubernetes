use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserCreate {
    pub email: String,
    pub name: String,
    pub age: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserUpdate {
    pub email: Option<String>,
    pub name: Option<String>,
    pub age: Option<i32>,
    pub is_active: Option<bool>,
}
