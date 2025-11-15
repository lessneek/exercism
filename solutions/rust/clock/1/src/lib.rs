use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        fn true_hours(hours: i32) -> i32 {
            let mut hours = hours % 24;
            if hours < 0 {
                hours = 24 + hours;
            }
            hours
        }

        let mut hours = true_hours(hours + minutes / 60);

        let mut minutes = minutes % 60;
        if minutes < 0 {
            hours = true_hours(hours - 1);
            minutes = 60 + minutes;
        }

        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minutes = self.minutes + minutes;
        let hours = self.hours;

        Clock::new(hours, minutes)
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}