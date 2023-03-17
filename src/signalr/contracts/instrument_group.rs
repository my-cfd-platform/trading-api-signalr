use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentGroupSignalRModel {
    pub id: String,
    pub name: String,
    pub weight: i32,
}
