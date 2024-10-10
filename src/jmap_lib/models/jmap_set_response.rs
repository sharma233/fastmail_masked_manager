use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaskedEmail {
    pub last_message_at: Option<String>,
    pub email: String,
    pub created_at: String,
    pub id: String,
    pub state: String,
    pub url: Option<String>,
    pub created_by: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateResBody {
    pub created: HashMap<String, MaskedEmail>,
    pub old_state: Option<String>,
    pub new_state: Option<String>,
    pub account_id: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JMAPSetResBody {
    pub latest_client_version: String,
    pub session_state: String,
    pub method_responses: Vec<(String, CreateResBody, String)>,
}
