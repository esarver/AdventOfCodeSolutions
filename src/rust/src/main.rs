mod lib;
use lib::common::input;
use lib::common::logger;
use lib::y2015::day01;
fn main() {
    let input_string = input::get_input("2015", "1");

    logger::log_year("2015");
    day01::day01(&input_string);
}
