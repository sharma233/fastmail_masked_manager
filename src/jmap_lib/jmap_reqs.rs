use reqwest::Method;
use std::collections::HashMap;
use super::models::jmap_set::{CreateParams, CreateBody, JMAPSetReqBody};

pub const USING_MASKED: &str = "https://www.fastmail.com/dev/maskedemail";
pub const BASE_URL: &str = "https://api.fastmail.com/jmap/api/";
pub const MASKEDEMAIL_SET_METHOD_TYPE: &str = "MaskedEmail/set";

pub async fn set_masked_email(api_key: &str, acc_id: &str, for_domain: &str, desc: &str) ->
Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    let create_params = CreateParams {
        description: desc.to_string(),
        for_domain: for_domain.to_string(),
    };

    let mut create_map = HashMap::new();
    create_map.insert(
        desc.to_string(),
        create_params,
    );

    let create_body = CreateBody {
        account_id: acc_id.to_string(),
        create: create_map,
    };

    let req_body = JMAPSetReqBody {
        using: vec![USING_MASKED.to_string()],
        method_calls: vec![(
            MASKEDEMAIL_SET_METHOD_TYPE.to_string(),
            create_body,
            "0".to_string(),
        )]
    };

    let request_mask = client.request(Method::POST, BASE_URL)
        .bearer_auth(api_key)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&req_body).unwrap())
        .build().unwrap(); //never returns an error


    let body = client.execute(request_mask).await?;
    Ok(body)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_res() {
        let res_string = "{
              \"latestClientVersion\": \"\",
              \"sessionState\": \"cyrus-46338;p-23fba35051;s-6704316984f90643\",
              \"methodResponses\": [
                [
                  \"MaskedEmail/set\",
                  {
                    \"destroyed\": [],
                    \"updated\": {},
                    \"created\": {
                      \"CLITest\": {
                        \"lastMessageAt\": null,
                        \"email\": \"equal.rain8971@fastmail.com\",
                        \"createdAt\": \"2024-10-07T22:27:12Z\",
                        \"id\": \"masked-100679183\",
                        \"state\": \"pending\",
                        \"url\": null,
                        \"createdBy\": \"API Token: cli-masked-mail\"
                      }
                    },
                    \"oldState\": null,
                    \"newState\": null,
                    \"accountId\": \"u25b140fc\"
                  },
                  \"c\"
                ]
              ]
            }";

        let deserialized: JMAPSetResBody = serde_json::from_str(&res_string).unwrap();
        assert_eq!(deserialized.method_responses[0].1.account_id, "u25b140fc");
        assert_eq!(deserialized.method_responses[0].1.created.get("CLITest").unwrap().email, "equal.rain8971@fastmail.com");
    }
}
