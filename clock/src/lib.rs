use std::fmt::{Debug, Formatter, Result};

const MINUTES_AN_HOUR: i32 = 60;
const HOURS_A_DAY: i32 = 24;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{:02}:{:02}",
            self.minutes / MINUTES_AN_HOUR,
            self.minutes % MINUTES_AN_HOUR
        )
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let minutes_a_day = MINUTES_AN_HOUR * HOURS_A_DAY;
        Self {
            minutes: {
                (((hours * MINUTES_AN_HOUR + minutes) % minutes_a_day) + minutes_a_day)
                    % minutes_a_day
            },
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.minutes + minutes)
    }
}
