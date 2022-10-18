use serde::Deserialize;
use std::collections::HashMap;

pub mod reports;
pub mod toggl;

#[derive(Deserialize)]
pub struct Config {
    pub api_token: String,
    pub rates: HashMap<String, u16>,
}
