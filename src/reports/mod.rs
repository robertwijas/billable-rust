pub mod toggl;

use colored::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::RangeInclusive;
use time::{Date, Duration, OffsetDateTime, Weekday};

pub mod month;
pub use month::*;

pub trait Billable {
    fn report(&self, range: &RangeInclusive<Date>) -> Result<Report, BillableError>;

    fn print_report(
        &self,
        month: Month,
        show_minutes: bool,
        configs: &Option<HashMap<String, ClientConfig>>,
    ) {
        println!("{}", format!("{}", month).bold().reversed());
        let report = self
            .report(&month.clone().into())
            .expect("failed to prepare report");

        for (client, hours) in report.total {
            print!("{:<25} {:>7}", client.dimmed(), hours.format(show_minutes));

            // TODO: why the line below has to be so ugly ???
            let goal = configs.as_ref().and_then(|x| x.get(&*client)?.goal);
            if let Some(goal) = goal {
                let progress = GoalProgress {
                    goal: Duration::hours(goal.into()),
                    done: hours,
                    working_time: Into::<RangeInclusive<Date>>::into(month.clone()),
                };

                let status = calculate_goal_status(progress);

                print!(" {:^10}", status);
            }

            println!();
        }
    }
}

trait FormattedDuration {
    fn format(&self, show_minutes: bool) -> String;
}

impl FormattedDuration for Duration {
    fn format(&self, show_minutes: bool) -> String {
        if show_minutes {
            let hours = self.whole_hours();
            let minutes = (*self - Duration::hours(hours)).whole_minutes();
            format!("{}:{:0>2}", hours, minutes)
        } else {
            format!("{}h", self.whole_hours())
        }
    }
}

fn calculate_goal_status<WT: WorkingTime>(progress: GoalProgress<WT>) -> GoalStatus<WT> {
    let estimated = progress.done
        * (1 as f64
            + progress.working_time.left().as_seconds_f64()
                / progress.working_time.used().as_seconds_f64());
    GoalStatus {
        progress,
        estimated,
    }
}

trait WorkingTime {
    fn used(&self) -> Duration;
    fn left(&self) -> Duration;
}

impl WorkingTime for RangeInclusive<Date> {
    fn used(&self) -> Duration {
        available_working_time_without_weekends(&RangeInclusive::new(
            *self.start(),
            OffsetDateTime::now_utc().date(),
        ))
    }
    fn left(&self) -> Duration {
        available_working_time_without_weekends(&RangeInclusive::new(
            OffsetDateTime::now_utc().date(),
            *self.end(),
        ))
    }
}

fn available_working_time<P>(range: &RangeInclusive<Date>, filter: P) -> Duration
where
    P: Fn(Date) -> bool,
{
    let mut duration = Duration::ZERO;
    for day in DaysIterator::new(range.clone()) {
        if filter(day) {
            duration = duration + Duration::days(1);
        }
    }
    duration
}

fn available_working_time_without_weekends(range: &RangeInclusive<Date>) -> Duration {
    available_working_time(range, |day| {
        ![Weekday::Saturday, Weekday::Sunday].contains(&day.weekday())
    })
}

#[cfg(test)]
mod tests {

    use super::*;
    use time::macros::date;

    #[test]
    fn available_working_time() {
        assert_eq!(
            available_working_time_without_weekends(&Into::<RangeInclusive<Date>>::into(
                Month::including(date!(2023 - 01 - 01))
            )),
            Duration::days(22)
        );
    }
}

#[derive(Debug, Clone)]
struct GoalProgress<WT: WorkingTime> {
    goal: Duration,
    done: Duration,
    working_time: WT,
}

impl<WT: WorkingTime> Display for GoalProgress<WT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.goal,
            self.done,
            self.working_time.used(),
            self.working_time.left()
        )
    }
}

// TODO:
// - daily target
// - weekly target
struct GoalStatus<WT: WorkingTime> {
    progress: GoalProgress<WT>,
    estimated: Duration,
}

impl<WT: WorkingTime> GoalStatus<WT> {
    fn emoji_indicator(&self) -> &str {
        if self.estimated < self.progress.goal {
            "🔴"
        } else {
            "🟢"
        }
    }
}

impl<WT: WorkingTime> Display for GoalStatus<WT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{} {}",
            self.estimated.format_as_hours(),
            self.progress.goal.format_as_hours(),
            self.emoji_indicator()
        )
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
