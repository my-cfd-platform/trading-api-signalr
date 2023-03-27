use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SetActiveAccountCommand {
    pub account_id: String,
}

impl SetActiveAccountCommand {
    pub fn new(account_id: String) -> Self {
        Self { account_id }
    }
}