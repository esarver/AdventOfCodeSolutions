use crate::lib::common::input;
use crate::lib::common::year::Year;
pub mod day01;
pub mod day02;

pub fn y2015() -> Year { 
    let mut y2015 = Year::new(2015);
    y2015.add_day(day01::day01(input::get_input("2015", "1").as_str()));
    y2015.add_day(day02::day02(input::get_input("2015", "2").as_str()));
    y2015
}

