use super::{BillableError, Client, Report};
use crate::harvest::{Config, Service};
use std::ops::RangeInclusive;
use time::{Date, Duration};

pub struct Billable {
    service: Service,
}

impl Billable {
    pub fn new(config: Config) -> Self {
        Billable {
            service: Service::new(config),
        }
    }
}

impl super::Billable for Billable {
    fn report(&self, range: &RangeInclusive<Date>) -> Result<super::Report, super::BillableError> {
        self.service
            .clients_report(range)
            .map(|response| Report {
                total: response
                    .results
                    .iter()
                    .map(|x| {
                        (
                            Client {
                                name: x.client_name.clone(),
                            },
                            Duration::seconds((x.billable_hours * 3600.0) as i64),
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
