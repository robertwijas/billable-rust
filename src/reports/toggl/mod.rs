use crate::toggl::{Endpoint, Service};

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

use super::{BillableError, Client, Report};
use std::ops::RangeInclusive;
use time::{Date, Duration};

impl super::Billable for Billable {
    fn report(&self, range: &RangeInclusive<Date>) -> Result<super::Report, super::BillableError> {
        self.service
            .get(Endpoint::me())
            .and_then(|user| {
                self.service.get(Endpoint::client_summary_report(
                    user.default_workspace_id.to_string(),
                    &range.start(),
                    &range.end(),
                ))
            })
            .map(|summary| Report {
                total: summary
                    .data
                    .iter()
                    .map(|x| {
                        (
                            Client {
                                name: x.title.client.clone().unwrap_or(String::from("Unassigned")),
                            },
                            Duration::milliseconds(x.time.into()),
                        )
                    })
                    .collect(),
            })
            .map_err(|e| {
                println!("{:?}", e);
                BillableError::Default
            })
    }
}
