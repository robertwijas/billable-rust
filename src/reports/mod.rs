pub mod toggl;

use std::fmt::Display;
use std::ops::Range;
use time::Date;
use colored::*;

pub mod month;
pub use month::*;

pub trait Billable {
    fn report(&self, range: Range<Date>) -> Result<Report, BillableError>;
}

#[derive(Debug)]
pub enum BillableError {
    Default,
}

#[derive(Debug)]
pub struct Report {
    total: Vec<(Client, i32)>,
}

#[derive(Debug)]
pub struct Client {
    name: String,
    // rate: u8, // TODO: implement currencies
}

impl Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.total.iter().for_each(|c| {
            let hours = format!("{}h", c.1).bold();
            writeln!(f, "{}: {}", c.0.name.dimmed(), hours).unwrap();
        });
        Ok(())
    }
}
