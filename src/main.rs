use tokio;

mod jmap_lib;
use jmap_lib::jmap_reqs::set_masked_email;
use core::panic;
use std::env;

pub const API_KEY_ENV_VAR: &str = "MASKED_EMAIL_TOKEN";
pub const ACC_ID_KEY_ENV_VAR: &str = "MASKED_EMAIL_ACC_ID";

#[tokio::main]
async fn main() {
    let api_key = match env::var(API_KEY_ENV_VAR){
        Ok(key) => key,
        Err(_) => panic!(
            "Cannot find environment variable containing api key,
            expected name: {API_KEY_ENV_VAR}"
        )
    };
    let acc_id = match env::var(ACC_ID_KEY_ENV_VAR){
        Ok(id) => id,
        Err(_) => panic!(
            "Cannot find environment variable containing account id key, 
            expected name: {ACC_ID_KEY_ENV_VAR}"
        )
    };

    let result = set_masked_email(api_key, acc_id).await;
    let _ = match result {
        Err(e) => { dbg!(e) },
        Ok(_) => todo!()
    };
}
