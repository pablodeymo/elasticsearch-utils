use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct RespStruct {
    #[serde(rename(deserialize = "_shards"))]
    pub shards: ShardsStruct,
    pub hits: HitsStruct,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShardsStruct {
    pub failed: u64,
    pub skipped: u64,
    pub successful: u64,
    pub total: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HitsStruct {
    pub hits: Vec<HashMap<String, Value>>,
}
