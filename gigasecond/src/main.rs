use time::{Date, Time};
use time::{Month, PrimitiveDateTime as DateTime};

use gigasecond::after;

fn main() {
    after(DateTime::new(
        Date::from_calendar_date(2038, Month::December, 1).unwrap(),
        Time::from_hms(2, 3, 4).unwrap(),
    ));
}
