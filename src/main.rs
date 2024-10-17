use tokio;

mod jmap_lib;
use jmap_lib::models::jmap_set_response::JMAPSetResBody;
use jmap_lib::jmap_reqs::set_masked_email;
use core::panic;
use std::env;
use std::io::{self, Write};
use getopts::Options;

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

    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("a", "alias", "set masked email alias", "ALIAS");
    opts.optopt("u", "url", "set the url for which this email will be used", "URL");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m },
        Err(e) => { panic!("{}", e.to_string()) }
    };
    let alias = match matches.opt_str("a") {
        Some(x) => x,
        None => panic!("Provide alias with -a option")
    };
    let url = match matches.opt_str("u") {
        Some(x) => x,
        None => panic!("Provide url with -u option")
    };

    let result = match set_masked_email(&api_key, &acc_id, &url, &alias).await {
        Ok(response) => response,
        Err(e) => panic!("{e}")
    };

    let body_txt = result.text().await.unwrap();
    let deserialized: JMAPSetResBody = serde_json::from_str(&body_txt).unwrap();

    let email = &deserialized.method_responses[0].1.created.get(&alias).unwrap().email;
    println!("{email}");
}
