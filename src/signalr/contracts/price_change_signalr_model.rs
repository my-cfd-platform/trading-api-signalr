use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PriceChangeSignalRModel {
    pub id: String,
    pub chng: f64,
}
