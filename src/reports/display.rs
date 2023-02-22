use super::MonthlyReport;
use crate::reports::GoalStatus;
use crate::reports::WorkingTime;
use colored::Colorize;
use time::Duration;

pub struct Printer {
    pub formatting_options: FormattingOptions,
}

impl Printer {
    pub fn print(&self, monthly_report: MonthlyReport) {
        println!("{}", format!("{}", monthly_report.month).bold().reversed());

        for (client_report, goal_status) in monthly_report.clients {
            print!(
                "{:<23} {:>5}",
                client_report.client_name.dimmed(),
                client_report.total.format(&self.formatting_options)
            );

            if let Some(goal_status) = goal_status {
                print!(" {:^10}", goal_status.format(&self.formatting_options));
            }

            println!();
        }
    }
}

pub struct FormattingOptions {
    pub show_minutes: bool,
}

#[derive(Default)]
enum Rounding {
    #[allow(unused)]
    Floor,
    #[default]
    Round,
    Ceil,
}

impl Rounding {
    fn apply(&self, value: f64) -> f64 {
        match self {
            Self::Floor => value.floor(),
            Self::Round => value.round(),
            Self::Ceil => value.ceil(),
        }
    }
}

trait Formatting {
    fn format_rounding(&self, options: &FormattingOptions, rounding: Rounding) -> String;

    fn format(&self, options: &FormattingOptions) -> String {
        self.format_rounding(options, Rounding::default())
    }
}

impl Formatting for Duration {
    fn format_rounding(&self, options: &FormattingOptions, rounding: Rounding) -> String {
        if options.show_minutes {
            let hours = self.whole_hours();
            let minutes = (*self - Duration::hours(hours)).whole_minutes();
            format!("{}:{:0>2}", hours, minutes)
        } else {
            let rounded_hours = rounding.apply(self.whole_minutes() as f64 / 60.0);
            format!("{}h", rounded_hours)
        }
    }
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
    fn format_rounding(&self, options: &FormattingOptions, _rounding: Rounding) -> String {
        let status = format!(
            "{} {}/{}",
            self.emoji_indicator(),
            self.estimated.format(options),
            self.goal.target.format(options)
        );

        if let Some(daily_target) = self.daily_target {
            let weekly_target: Duration = daily_target * 5;

            format!(
                "{} 🎯 {} a day, {} a week",
                status,
                daily_target.format_rounding(options, Rounding::Ceil),
                weekly_target.format(options),
            )
        } else {
            status
        }
    }
}
