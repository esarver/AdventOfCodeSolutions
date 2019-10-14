use super::part::Part;

pub struct Day {
    day: String,
    parts: Vec<Part>,
}

impl Day {
    pub fn new(day_number: u8) -> Day {
        Day {
            day: format!("Day {:2}", day_number),
            parts: Vec::new(),
        }
    }
    pub fn add_part(&mut self, new_part: Part) {
        self.parts.push(new_part);
    }
}

impl<'a> std::fmt::Display for Day {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut tmp = format!("\u{001b}[0m{}\u{001b}[0m\n", self.day);
        for part in &self.parts {
           tmp.push_str(&format!("{}", part)) 
        }
        formatter.write_str(&tmp)
    }
}
