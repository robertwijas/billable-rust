pub mod toggl;

use colored::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::ops::RangeInclusive;
use time::{Date, Duration};

pub mod month;
pub use month::*;

pub trait Billable {
    fn report(&self, range: &RangeInclusive<Date>) -> Result<Report, BillableError>;

    fn print_report(&self, month: Month, configs: &Option<HashMap<String, ClientConfig>>) {
        println!("{}", format!("{}", month).bold().reversed());
        let report = self
            .report(&month.clone().into())
            .expect("failed to prepare report");

        for (client, hours) in report.total {
            print!(
                "{:<25} {:^10}",
                client.dimmed(),
                hours.format_as_hours().bold()
            );

            // TODO: why the line below has to be so ugly ???
            let goal = configs.as_ref().and_then(|x| x.get(&*client)?.goal);
            if let Some(goal) = goal {
                let goal = Duration::hours(goal.into());
                let estimated = month.estimated_hours(hours);
                let indicator = if estimated < goal { "🔴" } else { "🟢" };

                print!(
                    " {:^10}",
                    format!(
                        "{}/{} {}",
                        estimated.format_as_hours(),
                        goal.format_as_hours(),
                        indicator
                    )
                );
            }

            println!();
        }
    }
}

trait FormatAsHours {
    fn format_as_hours(&self) -> String;
}

impl FormatAsHours for Duration {
    fn format_as_hours(&self) -> String {
        format!("{}h", self.whole_hours())
    }
}

#[derive(Debug)]
pub enum BillableError {
    Default,
}

#[derive(Debug)]
pub struct Report {
    pub total: Vec<(Client, Duration)>,
}

#[derive(Debug)]
pub struct Client {
    pub name: String,
}

impl std::ops::Deref for Client {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.name
    }
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct ClientConfig {
    pub rate: Option<u16>,
    pub goal: Option<u16>,
}
