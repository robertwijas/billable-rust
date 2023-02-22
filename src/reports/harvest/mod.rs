use super::{BillableError, ClientReport};
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
    fn clients_report(
        &self,
        range: &RangeInclusive<Date>,
    ) -> Result<Vec<ClientReport>, BillableError> {
        self.service
            .clients_report(range)
            .map(|response| {
                response
                    .results
                    .iter()
                    .map(|x| ClientReport {
                        client_name: x.client_name.clone(),
                        total: Duration::seconds((x.billable_hours * 3600.0) as i64),
                    })
                    .collect()
            })
            .map_err(|e| {
                println!("{:?}", e);
                BillableError::Default
            })
    }
}
