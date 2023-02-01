use std::fmt::Display;
use std::ops::Range;
use time::{Date, OffsetDateTime};

#[derive(Debug, PartialEq, Clone)]
pub struct Month {
    year: i32,
    month: time::Month,
}

impl Month {
    pub fn current() -> Self {
        Self::including(OffsetDateTime::now_utc().date())
    }

    fn including(date: Date) -> Self {
        let (year, month, _) = date.to_calendar_date();
        Month { year, month }
    }

    pub fn start(&self) -> Date {
        Date::from_calendar_date(self.year, self.month, 1)
            .expect("this should always be a valid year")
    }

    pub fn end(&self) -> Date {
        self.next()
            .start()
            .previous_day()
            .expect("there should be a previous day")
    }

    pub fn previous(&self) -> Self {
        match self.month {
            time::Month::January => Month {
                year: self.year - 1,
                month: self.month.previous(),
            },
            _ => Month {
                year: self.year,
                month: self.month.previous(),
            },
        }
    }

    pub fn next(&self) -> Self {
        match self.month {
            time::Month::December => Month {
                year: self.year + 1,
                month: self.month.next(),
            },
            _ => Month {
                year: self.year,
                month: self.month.next(),
            },
        }
    }
}

impl Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.month, self.year)
    }
}

impl Into<Range<Date>> for Month {
    fn into(self) -> Range<Date> {
        Range {
            start: self.start(),
            end: self.end(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::date;

    #[test]
    fn month() {
        assert_eq!(
            Month::including(date!(2022 - 03 - 23)),
            Month {
                year: 2022,
                month: time::Month::March
            }
        );
    }
}
