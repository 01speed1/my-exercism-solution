use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Display for Clock {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        write!(
            fmt,
            "{}:{}",
            Clock::format_number(self.hours),
            Clock::format_number(self.minutes)
        )
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Clock {
        let extra_hours = match minutes {
            0..=i32::MAX => Self::get_hours_when_positive_minutes(minutes),
            i32::MIN..=-1 => Self::get_hours_when_negative_minutes(minutes),
        };

        let minutes = match minutes {
            0..=i32::MAX => Self::calculate_when_positive_minutes(minutes),
            i32::MIN..=-1 => Self::calculate_when_negative_minutes(minutes),
        };

        let mut hours = extra_hours + hours;

        hours = Self::get_valid_hours(hours);

        Clock { hours, minutes }
    }

    pub fn add_minutes(self, additional: i32) -> Clock {
        let Clock { hours, minutes } = self;

        let mut minutes = minutes + additional;

        let extra_hours = match minutes {
            0..=i32::MAX => Self::get_hours_when_positive_minutes(minutes),
            i32::MIN..=-1 => Self::get_hours_when_negative_minutes(minutes),
        };

        minutes = match minutes {
            0..=i32::MAX => Self::calculate_when_positive_minutes(minutes),
            i32::MIN..=-1 => Self::calculate_when_negative_minutes(minutes),
        };

        let mut hours = extra_hours + hours;

        hours = Self::get_valid_hours(hours);

        Clock { hours, minutes }
    }

    fn format_number(number: i32) -> String {
        let mut number_to_string = number.to_string();
        if number_to_string.len() <= 1 {
            number_to_string.insert(0, '0');
        }
        number_to_string
    }

    fn calculate_when_positive_minutes(mut minutes: i32) -> i32 {
        if minutes > 59 {
            minutes %= 60;
        };
        minutes
    }

    fn get_hours_when_positive_minutes(minutes: i32) -> i32 {
        let mut hours = 0;
        if minutes > 59 {
            hours = minutes / 60
        }
        hours
    }

    fn get_hours_when_negative_minutes(mut negative_minutes: i32) -> i32 {
        let mut cycles = 0;
        while negative_minutes < 0 {
            negative_minutes += 60;
            cycles -= 1;
        }
        return cycles;
    }

    fn calculate_when_negative_minutes(mut negative_minutes: i32) -> i32 {
        while negative_minutes < 0 {
            negative_minutes += 60;
        }

        return negative_minutes;
    }

    fn get_valid_hours(hours: i32) -> i32 {
        match hours {
            24..=i32::MAX => hours % 24,
            0..=23 => hours,
            i32::MIN..=-1 => {
                let mut negative_hours = hours;
                while negative_hours < 0 {
                    negative_hours += 24;
                }
                negative_hours
            }
        }
    }
}
