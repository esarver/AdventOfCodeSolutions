mod lib;
use lib::common::input;
fn main() {
    let input_string = input::get_input("2015", "1");

    println!("Input:\n{}", input_string.as_str());
}
