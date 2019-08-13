mod lib;
use lib::common::input;
use lib::y2015::day01;
fn main() {
    let input_string = input::get_input("2015", "1");

    let ending_floor = day01::day01(&input_string);

    println!("Ending floor: {}", ending_floor);
}
