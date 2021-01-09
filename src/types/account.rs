use super::common::{NameAndId, Response};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Accounts {
    pub accounts: Vec<NameAndId>,
}

pub type AccountsResponse = Response<Accounts>;
