use std::fmt;

#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Interval {
    OneMinute,
    ThreeMinutes,
    FiveMinutes,
    FivteenMinutes,
    ThirtyMinutes,
    OneHour,
    OneDay,
    OneWeek,
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Interval::OneMinute => "1m",
                Interval::ThreeMinutes => "3m",
                Interval::FiveMinutes => "5m",
                Interval::FivteenMinutes => "15m",
                Interval::ThirtyMinutes => "30m",
                Interval::OneHour => "1h",
                Interval::OneDay => "1d",
                Interval::OneWeek => "1w",
            }
        )
    }
}
