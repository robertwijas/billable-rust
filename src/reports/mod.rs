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
        options: FormattingOptions,
        configs: &Option<HashMap<String, ClientConfig>>,
    ) {
        println!("{}", format!("{}", month).bold().reversed());
        let report = self
            .report(&month.clone().into())
            .expect("failed to prepare report");

        for (client, hours) in report.total {
            print!("{:<23} {:>5}", client.dimmed(), hours.format(&options));

            // TODO: why the line below has to be so ugly ???
            let goal = configs.as_ref().and_then(|x| x.get(&*client)?.goal);
            if let Some(goal) = goal {
                let goal = Goal {
                    target: Duration::hours(goal.into()),
                    done: hours,
                    working_time: Into::<RangeInclusive<Date>>::into(month.clone()),
                };

                let status = calculate_goal_status(goal);

                print!(" {:^10}", status.format(&options));
            }

            println!();
        }
    }
}

pub struct FormattingOptions {
    pub show_minutes: bool,
}

trait Formatting {
    fn format(&self, options: &FormattingOptions) -> String;
}

impl Formatting for Duration {
    fn format(&self, options: &FormattingOptions) -> String {
        if options.show_minutes {
            let hours = self.whole_hours();
            let minutes = (*self - Duration::hours(hours)).whole_minutes();
            format!("{}:{:0>2}", hours, minutes)
        } else {
            format!("{}h", self.whole_hours())
        }
    }
}

fn calculate_goal_status<WT: WorkingTime>(goal: Goal<WT>) -> GoalStatus<WT> {
    let estimated = goal.done
        * (1 as f64
            + goal.working_time.left().as_seconds_f64()
                / goal.working_time.used().as_seconds_f64());
    let daily_target = (goal.target - goal.done) / goal.working_time.left().whole_days() as f64;
    GoalStatus {
        goal,
        estimated,
        daily_target,
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
struct Goal<WT: WorkingTime> {
    target: Duration,
    done: Duration,
    working_time: WT,
}

impl<WT: WorkingTime> Display for Goal<WT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.target,
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
    goal: Goal<WT>,
    estimated: Duration,
    daily_target: Duration,
}

impl<WT: WorkingTime> GoalStatus<WT> {
    fn emoji_indicator(&self) -> &str {
        if self.estimated < self.goal.target {
            "🔴"
        } else {
            "🟢"
        }
    }
}

impl<WT: WorkingTime> Formatting for GoalStatus<WT> {
    fn format(&self, options: &FormattingOptions) -> String {
        let weekly_target: Duration = self.daily_target * 5;
        format!(
            "{} {}/{} 🎯 {} a day, {} a week",
            self.emoji_indicator(),
            self.estimated.format(options),
            self.goal.target.format(options),
            self.daily_target.format(options),
            weekly_target.format(options),
        )
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
