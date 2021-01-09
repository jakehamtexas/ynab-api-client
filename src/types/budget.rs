use super::common::{NameAndId, Response};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Serialize)]
pub struct Budgets {
    pub budgets: Vec<NameAndId>,
    pub default_budget: Option<NameAndId>,
}

pub type BudgetsResponse = Response<Budgets>;
