use std::fs;

pub fn get_input(year: &str, day: &str) -> String {
    let input_path = format!("../../inputs/{}/{}/input.txt", year, day);

    let file_contents = fs::read_to_string(&input_path)
        .expect(format!("File \"{}\" could not be retreived.", &input_path).as_str());

    file_contents
}
