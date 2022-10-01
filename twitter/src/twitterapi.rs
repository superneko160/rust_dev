use std::env;

#[derive(Debug)]
pub struct TwitterAPI {
  pub screen_name: String,
  pub bearer_token: String,
}

impl TwitterAPI {
    pub fn new() -> Self {
        Self {
            screen_name: env::var("SCREEN_NAME").expect("SCREEN_NAME is not defined"),
            bearer_token: env::var("BEARER_TOKEN").expect("BEARER_TOKEN is not defined"),
        }
    }
}