use crate::reports::ClientConfig;
use serde::Deserialize;
use std::collections::HashMap;

pub mod harvest;
pub mod reports;
pub mod toggl;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub api_token: String,
    pub clients: Option<HashMap<String, ClientConfig>>,
    pub services: Option<Vec<ServiceConfig>>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ServiceConfig {
    Toggl {
        name: Option<String>,
        api_token: String,
    },
    Harvest {
        name: Option<String>,
        account_id: String,
        api_token: String,
    },
}

impl ServiceConfig {
    pub fn display_name(&self) -> String {
        match self {
            Self::Toggl { name, .. } => name.clone().unwrap_or("Toggl".to_string()),
            Self::Harvest { name, .. } => name.clone().unwrap_or("Harvest".to_string()),
        }
    }
    pub fn billable(&self) -> Box<dyn reports::Billable> {
        // TODO: how to design this without Boxing?
        match self {
            Self::Toggl { name: _, api_token } => {
                Box::new(reports::toggl::Billable::new(api_token.clone()))
            }
            Self::Harvest {
                name: _,
                account_id,
                api_token,
            } => Box::new(reports::harvest::Billable::new(harvest::Config {
                account_id: account_id.to_string(),
                api_token: api_token.to_string(),
            })),
        }
    }
}
