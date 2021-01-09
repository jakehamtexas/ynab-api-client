use super::common::{NameAndId, Response};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CategoryGroup {
    pub id: String,
    pub name: String,
    pub categories: Vec<NameAndId>,
}

#[derive(Deserialize, Serialize)]
pub struct Categories {
    pub category_groups: Vec<CategoryGroup>,
}

pub type CategoriesResponse = Response<Categories>;
