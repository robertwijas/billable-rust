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
use time::Date;

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
                // TODO: implement From conversion?
                total: summary
                    .data
                    .iter()
                    .map(|x| {
                        let client = Client {
                            name: x.title.client.clone().unwrap_or(String::from("Unassigned")),
                        };
                        (
                            client,
                            u16::try_from(x.time / (60 * 60 * 1000))
                                .expect("hours should fit in u16"),
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
