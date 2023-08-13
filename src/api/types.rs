use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tabled::Tabled;

#[derive(Debug, Serialize, Deserialize)]
pub struct ListTableRequest {
    pub table_name: String,
    pub name: Option<String>,
    pub team: Option<String>,
    pub version: Option<String>,
    pub uid: Option<String>,
    pub limit: Option<i16>,
    pub tags: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DownloadMetadataRequest {
    pub name: Option<String>,
    pub team: Option<String>,
    pub version: Option<String>,
    pub uid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelDownloadRequest {
    pub read_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Card {
    pub name: String,
    pub team: String,
    pub date: String,
    pub user_email: String,
    pub version: String,
    pub uid: String,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListCardResponse {
    pub cards: Vec<Card>,
}

#[derive(Tabled)]
pub struct CardTable {
    pub name: String,
    pub team: String,
    pub date: String,
    pub user_email: String,
    pub version: String,
    pub uid: String,
    pub tags: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelMetadata {
    pub model_name: String,
    pub model_type: String,
    pub onnx_uri: String,
    pub onnx_version: String,
    pub model_uri: String,
    pub model_version: String,
    pub sample_data: HashMap<String, Value>,
    pub data_schema: HashMap<String, HashMap<String, Value>>,
}
