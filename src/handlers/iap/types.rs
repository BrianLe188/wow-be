use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SaveReceiptPayload {
    #[serde(rename = "appType")]
    pub app_type: String,
    pub purchase: serde_json::Value,
}
