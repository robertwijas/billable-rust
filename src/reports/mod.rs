pub mod toggl;

use colored::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::ops::RangeInclusive;
use time::Date;

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
                client.name.dimmed(),
                format!("{}h", hours).bold()
            );

            // TODO: why the line below has to be so ugly ???
            let goal = configs.as_ref().and_then(|x| x.get(&client.name)?.goal);
            if let Some(goal) = goal {
                let estimated = month.estimated_hours(hours);
                print!(" {:^10}", format!("{}h/{}h", estimated, goal));
            }

            println!();
        }
    }
}

#[derive(Debug)]
pub enum BillableError {
    Default,
}

#[derive(Debug)]
pub struct Report {
    pub total: Vec<(Client, u16)>,
}

#[derive(Debug)]
pub struct Client {
    pub name: String,
    // rate: u8, // TODO: implement currencies
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct ClientConfig {
    pub rate: Option<u16>,
    pub goal: Option<u16>,
}
