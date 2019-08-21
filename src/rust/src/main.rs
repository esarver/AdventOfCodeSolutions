mod lib;
use lib::y2015;
fn main() {
    let mut years = Vec::new();
    years.push(y2015::y2015());

    let intro = r#"

   \       |               |          _|   __|          |     
  _ \   _` |\ \ / -_)   \   _|   _ \  _|  (     _ \  _` |  -_)
_/  _\\__,_| \_/\___|_| _|\__| \___/_|   \___|\___/\__,_|\___| 

                                       -- Solutions in Rust --

    "#; // Generated using the "smshadow" font at https://www.kammerl.de/ascii/AsciiSignature.php

    println!("{}", intro);

    for year in years {
        println!("{}", year);
    }

}
