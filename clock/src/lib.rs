use std::fmt::Display;

const MINUTES_PER_DAY : i32 = 1440;
const MINUTES_PER_HOUR : i32 = 60;

#[derive(Debug, PartialEq)]
pub struct Clock{
    hours: i32,
    minutes: i32
}

impl Clock {

    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = (hours * MINUTES_PER_HOUR + minutes).rem_euclid(MINUTES_PER_DAY);
        Clock {
            hours: total_minutes / MINUTES_PER_HOUR,
            minutes: total_minutes % MINUTES_PER_HOUR,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:02}:{:02}", self.hours, self.minutes))
    }
}

