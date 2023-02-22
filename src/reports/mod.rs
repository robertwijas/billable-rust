use serde::Deserialize;
use std::collections::HashMap;
use std::ops::RangeInclusive;
use time::{Date, Duration, OffsetDateTime, Weekday};

pub mod demo;
pub mod harvest;
pub mod toggl;

pub mod display;
pub mod month;

pub use month::*;

pub trait Billable {
    fn clients_report(
        &self,
        range: &RangeInclusive<Date>,
    ) -> Result<Vec<ClientReport>, BillableError>;

    fn monthly_report(
        &self,
        month: Month,
        configs: &Option<HashMap<String, ClientConfig>>,
    ) -> Result<MonthlyReport, BillableError> {
        self.clients_report(&month.clone().into())
            .map(|clients_reports| {
                clients_reports
                    .iter()
                    .map(|client_report| {
                        let goal_status = configs
                            .as_ref()
                            .and_then(|x| x.get(&*client_report.client_name)?.goal)
                            .map(|target| {
                                calculate_goal_status(
                                    Goal {
                                        target: Duration::hours(target.into()),
                                        working_time: month.clone(),
                                    },
                                    client_report.total,
                                )
                            });
                        (client_report.clone(), goal_status)
                    })
                    .collect()
            })
            .map(|clients| MonthlyReport { month, clients })
    }
}

#[derive(Debug, Clone)]
pub struct Goal<WT: WorkingTime> {
    pub target: Duration,
    pub working_time: WT,
}

pub struct GoalStatus<WT: WorkingTime> {
    pub goal: Goal<WT>,
    pub estimated: Duration,
    pub daily_target: Option<Duration>,
}

#[derive(Debug)]
pub enum BillableError {
    Default,
}

#[derive(Clone)]
pub struct ClientReport {
    client_name: String,
    total: Duration,
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct ClientConfig {
    pub rate: Option<u16>,
    pub goal: Option<u16>,
}

pub struct MonthlyReport {
    pub month: Month,
    pub clients: Vec<(ClientReport, Option<GoalStatus<Month>>)>,
}

fn calculate_goal_status<WT: WorkingTime>(goal: Goal<WT>, done: Duration) -> GoalStatus<WT> {
    let estimated = done
        * (1 as f64
            + goal.working_time.left().as_seconds_f64()
                / goal.working_time.used().as_seconds_f64());
    let days_left = goal.working_time.left().whole_days() as f64;
    let daily_target: Option<Duration> = if days_left > 0.0 {
        Some((goal.target - done) / days_left)
    } else {
        None
    };
    GoalStatus {
        goal,
        estimated,
        daily_target,
    }
}

pub trait WorkingTime {
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

impl WorkingTime for Month {
    fn used(&self) -> Duration {
        Into::<RangeInclusive<Date>>::into(self.clone()).used()
    }

    fn left(&self) -> Duration {
        Into::<RangeInclusive<Date>>::into(self.clone()).left()
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
