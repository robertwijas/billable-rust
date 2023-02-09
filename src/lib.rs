use serde::Deserialize;
use std::collections::HashMap;

pub mod reports;
pub mod toggl;

use reports::ClientConfig;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub api_token: String,
    pub clients: Option<HashMap<String, ClientConfig>>,
}
