pub fn log_year(year: &str) {
    println!("\u{001b}[4m[{}]\u{001b}[0m", year);
}

pub fn log_day(day: &str) {
    println!("|--- {}:", day);
}

pub fn log_answer(part: &str, answer: &str) {
    println!(
        "|---|--- {}: \u{001b}[32m\u{001b}[7m[{}]\u{001b}[0m",
        part, answer
    );
}
