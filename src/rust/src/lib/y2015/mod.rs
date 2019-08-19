use crate::lib::common::input;
use crate::lib::common::task;
pub mod day01;

pub fn y2015() -> task::Year { 
    let mut y2015 = task::Year::new(2015);
    y2015.add_day(day01::day01(input::get_input("2015", "1").as_str()));
    y2015
}

