pub struct UserBase {
    pub email: String,
    pub name: String,
    pub age: Option<i32>,
}

pub struct UserCreate {
    pub email: String,
    pub name: String,
    pub age: Option<i32>,
}

pub struct UserUpdate {
    pub email: Option<String>,
    pub name: Option<String>,
    pub age: Option<i32>,
    pub is_active: Option<bool>,
}
