use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct NameAndId {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct Response<T> {
    pub data: T,
}
