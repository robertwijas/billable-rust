use super::{BillableError, ClientReport};
use crate::toggl::{Endpoint, Service};
use std::ops::RangeInclusive;
use time::{Date, Duration};

pub struct Billable {
    service: Service,
}

impl Billable {
    pub fn new(api_token: String) -> Self {
        Billable {
            service: Service::new(api_token),
        }
    }
}

impl super::Billable for Billable {
    fn clients_report(
        &self,
        range: &RangeInclusive<Date>,
    ) -> Result<Vec<ClientReport>, BillableError> {
        self.service
            .get(Endpoint::me())
            .and_then(|user| {
                self.service.get(Endpoint::client_summary_report(
                    user.default_workspace_id.to_string(),
                    &range.start(),
                    &range.end(),
                ))
            })
            .map_err(|e| {
                eprintln!("{:?}", e);
                BillableError::Default
            })
            .map(|report| {
                report
                    .data
                    .iter()
                    .map(|client_summary| ClientReport {
                        client_name: client_summary
                            .title
                            .client
                            .clone()
                            .unwrap_or(String::from("Unassigned")),
                        total: Duration::milliseconds(client_summary.time.into()),
                    })
                    .collect()
            })
    }
}
