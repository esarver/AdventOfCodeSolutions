use super::day::Day;

pub struct Year {
    year: String,
    days: Vec<Day>,
}
impl Year {
    pub fn new(year_number: u16) -> Year {
        Year {
            year: format!("{:4}", year_number),
            days: Vec::new(),
        }
    }
    pub fn add_day(&mut self, new_day: Day) {
        self.days.push(new_day);
    }
}
impl std::fmt::Display for Year {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut tmp = format!("\u{001b}[0m\u{001b}[4;1m[{}]\u{001b}[0m\n", self.year);
        for day in &self.days {
           tmp.push_str(&format!("{}", day)) 
        }
        formatter.write_str(&tmp)
    }
}