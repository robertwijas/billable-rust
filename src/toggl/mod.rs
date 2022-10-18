use reqwest::header::CONTENT_TYPE;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::{collections::HashMap, marker::PhantomData};

pub struct Endpoint<T> {
    api: API,
    path: String,
    params: Option<HashMap<&'static str, String>>,
    result: PhantomData<T>,
}

enum API {
    V9,
    ReportsV2,
}

impl API {
    fn path(&self) -> &str {
        match self {
            API::V9 => "api/v9",
            API::ReportsV2 => "reports/api/v2",
        }
    }
}

impl<T: DeserializeOwned> Endpoint<T> {
    fn url(&self) -> String {
        format!(
            "https://api.track.toggl.com/{}/{}",
            self.api.path(),
            self.path
        )
    }
}

impl Endpoint<User> {
    pub fn me() -> Self {
        Endpoint {
            api: API::V9,
            path: "me".into(),
            params: None,
            result: PhantomData,
        }
    }
}

impl Endpoint<Vec<TimeEntry>> {
    pub fn time_entries() -> Self {
        Endpoint {
            api: API::V9,
            path: "me/time_entries".into(),
            params: None,
            result: PhantomData,
        }
    }
}

impl Endpoint<ClientSummaryReport> {
    pub fn client_summary_report(workspace_id: String) -> Self {
        Endpoint {
            api: API::ReportsV2,
            path: "summary".into(),
            params: Some(HashMap::from([
                ("workspace_id", workspace_id),
                ("grouping", "clients".into()),
                ("subgrouping", "users".into()),
                ("user_agent", "billable".into()),
                ("since", "2022-10-01".into()),
            ])),
            result: PhantomData,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub default_workspace_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct TimeEntry {
    pub duration: i32,
}

#[derive(Debug, Deserialize)]
pub struct ClientSummaryReport {
    pub data: Vec<ClientSummary>,
}

#[derive(Debug, Deserialize)]
pub struct ClientSummary {
    pub title: ClientSummaryTitle,
    pub time: i32,
}

#[derive(Debug, Deserialize)]
pub struct ClientSummaryTitle {
    pub client: String,
}

#[test]
fn me_url() {
    assert_eq!(
        Endpoint::me().url(),
        "https://api.track.toggl.com/api/v9/me"
    );
}

pub struct Service {
    api_token: String,
    client: reqwest::blocking::Client,
}

impl Service {
    pub fn new(api_token: String) -> Self {
        Service {
            api_token,
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn get<T: DeserializeOwned>(&self, endpoint: Endpoint<T>) -> Result<T, reqwest::Error> {
        let response = self
            .client
            .get(endpoint.url())
            .basic_auth(&self.api_token, Some("api_token"))
            .header(CONTENT_TYPE, "application/json")
            .query(&endpoint.params)
            .send();

        // println!("{:?}", response);

        response?.json()
    }
}
