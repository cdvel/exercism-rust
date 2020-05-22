// use std::fmt;

#[derive(std::fmt::Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub const MINS_HOUR: i32 = 60;
    pub const HOURS_DAY: i32 = 24;



    pub fn new(hours: i32, minutes: i32) -> Self {

        let roll_hours = hours >= Clock::HOURS_DAY;
        let roll_minutes = minutes >= Clock::MINS_HOUR;
        // let neg_hours = hours < 0;
        // let neg_minutes = minutes < 0;

        let (mut n_hours, mut n_minutes) = (hours, minutes);

        match (roll_hours, roll_minutes, n_hours < 0, n_minutes < 0) {
            (_, true,_, _) => {
                n_hours = (n_hours + n_minutes / Clock::MINS_HOUR) % Clock::HOURS_DAY;
                n_minutes %= Clock::MINS_HOUR;
            }
            (true, _, _, _) => n_hours %= Clock::HOURS_DAY,
            (_, _, _, true) => {
                // let hours_in_min = (n_minutes / Clock::MINS_HOUR) * -1;
                // n_hours -= hours_in_min % Clock::HOURS_DAY;
                // n_hours -= n_minutes / Clock::MINS_HOUR; 
                
                n_hours = (n_minutes / Clock::MINS_HOUR) % Clock::HOURS_DAY;

                if n_minutes < -Clock::MINS_HOUR {n_hours += Clock::HOURS_DAY};
                if n_minutes == -Clock::MINS_HOUR{
                    n_minutes = 0;
                    n_hours *= -1;
                }else{
                    n_minutes = (n_minutes % Clock::MINS_HOUR) + Clock::MINS_HOUR;
                }

                // ((n_hours + n_minutes / Clock::MINS_HOUR) % Clock::HOURS_DAY) + Clock::HOURS_DAY;
            }
            (_, _, true, _) => n_hours = (n_hours % Clock::HOURS_DAY) + Clock::HOURS_DAY,
 
            _ => (),
        };
        Self {
            hours: n_hours,
            minutes: n_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut new_minutes: i32 = self.minutes;
        let mut new_hours: i32 = self.hours;
        new_minutes += minutes;
        if new_minutes >= 60 {
            new_hours += new_minutes / 60;
            new_minutes += new_minutes % 60;
        }
        Clock::new(new_hours, new_minutes)
    }

}
