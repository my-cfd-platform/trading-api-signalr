use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentGroupSignalRModel {
    pub id: String,
    pub name: String,
    pub weight: i32,
}