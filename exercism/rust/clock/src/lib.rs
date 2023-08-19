use std::fmt;

const MINUTES_PER_HOUR: i32 = 60;
const HOURS_PER_DAY: i32 = 24;


#[derive(fmt::Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: (hours + minutes.div_euclid(MINUTES_PER_HOUR)).rem_euclid(HOURS_PER_DAY),
            minutes: minutes.rem_euclid(MINUTES_PER_HOUR),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
