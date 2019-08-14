use std::fs;

pub fn get_input(year: &str, day: &str) -> String {
    let input_path = format!("../../inputs/{}/{}/input.txt", year, day);

    fs::read_to_string(&input_path)
        .unwrap_or_else(|_| panic!("File \"{}\" could not be retreived.", &input_path))
}
