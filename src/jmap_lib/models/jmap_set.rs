use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateParams {
    pub description: String,
    pub for_domain: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBody {
    pub account_id: String,
    pub create: HashMap<String, CreateParams>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JMAPSetReqBody {
    pub using: Vec<String>,
    pub method_calls: Vec<(String, CreateBody, String)>
}
