use std::{cell::Cell, collections::HashMap};

use time::Duration;

use super::ClientReport;

pub struct Billable {
    data: HashMap<String, Vec<f64>>,
    counter: Cell<usize>,
}

impl Billable {
    pub fn new(data: HashMap<String, Vec<f64>>) -> Self {
        Billable {
            data,
            counter: 0.into(),
        }
    }
}

impl super::Billable for Billable {
    fn clients_report(
        &self,
        _range: &std::ops::RangeInclusive<time::Date>,
    ) -> Result<Vec<super::ClientReport>, super::BillableError> {
        let report = self
            .data
            .iter()
            .map(|(client, hours)| {
                let total = if hours.is_empty() {
                    None
                } else {
                    hours.get(self.counter.get() % hours.len())
                }
                .map(|h| Duration::seconds((h * 3600.0) as i64))
                .unwrap_or(Duration::ZERO);

                ClientReport {
                    client_name: client.to_string(),
                    total,
                }
            })
            .collect();

        self.counter.set(self.counter.get() + 1);

        Ok(report)
    }
}
