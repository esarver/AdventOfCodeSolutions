mod lib;
use lib::common::input;
use lib::y2015::day01;
fn main() {
    let input_string = input::get_input("2015", "1");

    println!("\u{001b}[4m[2015]\u{001b}[0m");
    day01::day01(&input_string);
}
