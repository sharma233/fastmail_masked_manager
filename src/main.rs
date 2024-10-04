use tokio;
use reqwest::Method;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::Display;

pub const API_KEY_ENV_VAR: &str = "MASKED_EMAIL_TOKEN";
pub const ACC_ID_KEY_ENV_VAR: &str = "MASKED_EMAIL_ACC_ID";

pub const USING_CORE: &str = "urn:ietf:params:jmap:core";
pub const USING_MASKED: &str = "https://www.fastmail.com/dev/maskedemail";

pub const BASE_URL: &str = "https://api.fastmail.com/jmap/api/";

pub const MASKEDEMAIL_GET_METHOD_TYPE: &str = "MaskedEmail/get";
pub const ACC_ID_PARAM_KEY: &str = "accountId";


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JMAPReqBody {
    pub using: Vec<String>,
    pub method_calls: Vec<(String, HashMap<String, String>, String)>
}

#[derive(Debug)]
enum JMAPError {
    VarError(env::VarError),
    Error(reqwest::Error)
}

impl Display for JMAPError {
    fn fmt(&self, f: & mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JMAPError::Error(e) => {
                write!(f, "{}", e)
            },
            JMAPError::VarError(e) => {
                write!(f, "{}", e)
            }
        }
    }
}

impl From<reqwest::Error> for JMAPError {
    fn from(err: reqwest::Error) -> Self {
        JMAPError::Error(err)
    }
}

impl From<std::env::VarError> for JMAPError {
    fn from(err: std::env::VarError) -> Self {
        JMAPError::VarError(err)
    }
}

impl std::error::Error for JMAPError {}

#[tokio::main]
async fn main() {
    let result = get_masked_email().await;
    let _ = match result {
        Err(e) => { dbg!(e) },
        Ok(_) => todo!()
    };
}

async fn get_masked_email() -> Result<(), JMAPError> {
    let api_key = env::var(API_KEY_ENV_VAR)?;
    let client = reqwest::Client::new();
    let acc_id = env::var(ACC_ID_KEY_ENV_VAR)?;

    let req_body = JMAPReqBody {
        using: vec![USING_MASKED.to_string(), USING_MASKED.to_string()], 
        method_calls: vec![(
             MASKEDEMAIL_GET_METHOD_TYPE.to_string(),
             HashMap::from([(ACC_ID_PARAM_KEY.to_string(), acc_id.to_string())]),
             "0".to_string())
        ]
    };

    let request_mask = client.request(Method::POST, "https://api.fastmail.com/jmap/api/")
        .bearer_auth(api_key)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&req_body).unwrap())
        .build().unwrap(); //never returns an error


    let body = client.execute(request_mask).await?;
    //let body = client.execute(request_mask).await.unwrap().text().await.unwrap();
    let body_txt = body.text().await.unwrap();

    println!("body = {body_txt:?}");
    Ok(())
}
