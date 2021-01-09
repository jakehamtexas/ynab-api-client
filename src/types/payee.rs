use super::common::{NameAndId, Response};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Payees {
    pub payees: Vec<NameAndId>,
}

pub type PayeesResponse = Response<Payees>;
