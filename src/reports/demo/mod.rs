use std::{cell::Cell, collections::HashMap};

use time::Duration;

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
    fn report(
        &self,
        _range: &std::ops::RangeInclusive<time::Date>,
    ) -> Result<super::Report, super::BillableError> {
        let report = super::Report {
            total: self
                .data
                .iter()
                .map(|(client, hours)| {
                    let hours = if hours.is_empty() {
                        None
                    } else {
                        hours.get(self.counter.get() % hours.len())
                    }
                    .unwrap_or(&0.0);

                    (
                        super::Client {
                            name: client.to_string(),
                        },
                        Duration::seconds((hours * 3600.0) as i64),
                    )
                })
                .collect(),
        };

        self.counter.set(self.counter.get() + 1);

        Ok(report)
    }
}
