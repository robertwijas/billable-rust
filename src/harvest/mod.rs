use std::ops::RangeInclusive;
use time::{macros::format_description, Date};

use reqwest::{blocking::Client, header::CONTENT_TYPE, header::USER_AGENT};
use serde::Deserialize;

pub struct Config {
    pub account_id: String,
    pub api_token: String,
}

pub struct Service {
    config: Config,
    client: Client,
}

impl Service {
    pub fn new(config: Config) -> Self {
        Service {
            config,
            client: Client::new(),
        }
    }

    pub fn clients_report(
        &self,
        range: &RangeInclusive<Date>,
    ) -> Result<ClientsReportResponse, reqwest::Error> {
        let url = "https://api.harvestapp.com/v2/reports/time/clients";
        let params = [("from", format(range.start())), ("to", format(range.end()))];

        self.client
            .get(url)
            .bearer_auth(self.config.api_token.clone())
            .header(CONTENT_TYPE, "application/json")
            .header("Harvest-Account-Id", self.config.account_id.clone())
            .header(USER_AGENT, "Billable")
            .query(&params)
            .send()?
            .json()
    }
}

fn format(date: &Date) -> String {
    date.format(format_description!("[year][month][day]"))
        .expect("formatting should work")
}

#[derive(Debug, Deserialize)]
pub struct ClientsReportResponse {
    // I could use generics here, but making it deserializable with serde is very complicated:
    // https://serde.rs/attr-bound.html
    pub results: Vec<ClientsReport>,
}

#[derive(Debug, Deserialize)]
pub struct ClientsReport {
    pub client_name: String,
    pub billable_hours: f64,
}
