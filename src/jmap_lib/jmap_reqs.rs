use serde::{Deserialize, Serialize};
use std::env;
use reqwest::Method;
use std::collections::HashMap;
use crate::JMAPError;

pub const USING_MASKED: &str = "https://www.fastmail.com/dev/maskedemail";

pub const BASE_URL: &str = "https://api.fastmail.com/jmap/api/";

pub const MASKEDEMAIL_GET_METHOD_TYPE: &str = "MaskedEmail/get";
pub const MASKEDEMAIL_SET_METHOD_TYPE: &str = "MaskedEmail/set";
pub const ACC_ID_PARAM_KEY: &str = "accountId";

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateReqBody {
    pub account_id: String,
    pub create: HashMap<String, HashMap<String, String>>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JMAPSetReqBody {
    pub using: Vec<String>,
    pub method_calls: Vec<(String, CreateReqBody, String)>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JMAPGetReqBody {
    pub using: Vec<String>,
    pub method_calls: Vec<(String, HashMap<String, String>, String)>
}

pub async fn get_masked_email(api_key: String, acc_id: String) -> Result<(), JMAPError> {
    let client = reqwest::Client::new();

    let req_body = JMAPGetReqBody {
        using: vec![USING_MASKED.to_string(), USING_MASKED.to_string()], 
        method_calls: vec![(
             MASKEDEMAIL_GET_METHOD_TYPE.to_string(),
             HashMap::from([(ACC_ID_PARAM_KEY.to_string(), acc_id.to_string())]),
             "0".to_string())
        ]
    };

    let request_mask = client.request(Method::POST, BASE_URL)
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


pub async fn set_masked_email(api_key: String, acc_id: String) -> Result<(), JMAPError> {
    let client = reqwest::Client::new();

    let mut create_map = HashMap::new();

    create_map.insert(
        "CLITest".to_string(),
        HashMap::from([
        ("description".to_string(), "CLITest".to_string()),
        ("forDomain".to_string(), "CLITest.com".to_string())
        ]),
    );

    let create_body = CreateReqBody {
        account_id: acc_id.to_string(),
        create: create_map
    };

    let req_body = JMAPSetReqBody {
        using: vec![USING_MASKED.to_string()],
        method_calls: vec![(
            MASKEDEMAIL_SET_METHOD_TYPE.to_string(),
            create_body,
            "0".to_string()
        )]
    };

    println!("serialized ={}", serde_json::to_string(&req_body).unwrap());

    let request_mask = client.request(Method::POST, BASE_URL)
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
