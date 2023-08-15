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
    pub max_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CardRequest {
    pub name: Option<String>,
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
pub struct ListMetricResponse {
    pub metrics: HashMap<String, Vec<Metric>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metric {
    pub name: String,
    pub value: Value,
    pub step: Option<Value>,
    pub timestamp: Option<Value>,
}

#[derive(Tabled)]
pub struct MetricTable {
    pub metric: String,
    pub value: Value,
    pub step: String,
    pub timestamp: String,
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelDataSchema {
    data_type: String,
    input_features: HashMap<String, Value>,
    output_features: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataSchema {
    model_data_schema: ModelDataSchema,
    input_data_schema: Option<HashMap<String, Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelMetadata {
    pub model_name: String,
    pub model_type: String,
    pub onnx_uri: Option<String>,
    pub onnx_version: Option<String>,
    pub model_uri: String,
    pub model_version: String,
    pub model_team: String,
    pub sample_data: HashMap<String, Value>,
    pub data_schema: DataSchema,
}
