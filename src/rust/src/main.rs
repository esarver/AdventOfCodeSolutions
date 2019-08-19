mod lib;
use lib::y2015;
fn main() {
    let mut years = Vec::new();
    years.push(y2015::y2015());

    for year in years {
        println!("{}", year);
    }

}
