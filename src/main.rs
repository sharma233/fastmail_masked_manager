use tokio;
use reqwest::Method;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JMAPReqBody {
    pub using: Vec<String>,
    pub method_calls: Vec<(String, HashMap<String, String>, String)>
}

#[tokio::main]
async fn main() {
    get_masked_email().await;
}

async fn get_masked_email() {
    let api_key = env::var("MASKED_EMAIL_TOKEN").unwrap();
    let client = reqwest::Client::new();
    let acc_id = env::var("MASKED_EMAIL_ACC_ID").unwrap();

    let req_body = JMAPReqBody {
        using: vec!["https://www.fastmail.com/dev/maskedemail".to_string(), "urn:ietf:params:jmap:core".to_string()], 
        method_calls: vec![("MaskedEmail/get".to_string(), HashMap::from([("accountId".to_string(), acc_id)]), "0".to_string())] 
    };

    let request_mask = client.request(Method::POST, "https://api.fastmail.com/jmap/api/")
        .bearer_auth(api_key)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&req_body).unwrap())
        .build().unwrap();


    let body = client.execute(request_mask).await.unwrap().text().await.unwrap();

    println!("body = {body:?}");
}
