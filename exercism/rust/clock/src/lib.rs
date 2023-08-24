use std::fmt;

const MINUTES_PER_HOUR: i32 = 60;
const MINUTES_PER_DAY: i32 = 24 * MINUTES_PER_HOUR;

#[derive(fmt::Debug, PartialEq)]
pub struct Clock(i32);

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock((hours * MINUTES_PER_HOUR + minutes).rem_euclid(MINUTES_PER_DAY))
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.0 + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}",
            self.0 / MINUTES_PER_HOUR,
            self.0 % MINUTES_PER_HOUR
        )
    }
}
