use std::fmt::Display;
use std::ops::RangeInclusive;
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

    pub fn including(date: Date) -> Self {
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

    pub fn days(&self) -> DaysIterator {
        DaysIterator::new(self.clone().into())
    }

    pub fn iter(&self) -> MonthsIterator {
        MonthsIterator::new(self.clone())
    }
}

impl Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.month, self.year)
    }
}

impl Into<RangeInclusive<Date>> for Month {
    fn into(self) -> RangeInclusive<Date> {
        RangeInclusive::new(self.start(), self.end())
    }
}

pub struct DaysIterator {
    range: RangeInclusive<Date>,
    previous: Date,
}

impl DaysIterator {
    pub fn new(range: RangeInclusive<Date>) -> Self {
        let previous = range
            .start()
            .previous_day()
            .expect("expecting the past to be there");
        Self { range, previous }
    }
}

impl Iterator for DaysIterator {
    type Item = Date;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.previous.next_day();
        if let Some(next) = next {
            if self.range.contains(&next) {
                self.previous = next;
            } else {
                return None;
            }
        }

        next
    }
}

pub struct MonthsIterator {
    current: Month,
}

impl MonthsIterator {
    pub fn new(start: Month) -> Self {
        MonthsIterator { current: start }
    }
}

impl Iterator for MonthsIterator {
    type Item = Month;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.current.clone();
        self.current = self.current.next();
        Some(next)
    }
}

impl DoubleEndedIterator for MonthsIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        let next = self.current.clone();
        self.current = self.current.previous();
        Some(next)
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

    #[test]
    fn days() {
        assert_eq!(31, Month::including(date!(2023 - 01 - 01)).days().count());
        assert_eq!(28, Month::including(date!(2023 - 02 - 01)).days().count());
    }
}
