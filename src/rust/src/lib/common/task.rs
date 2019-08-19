
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
        let mut tmp = format!("\u{001b}[4m[{}]\u{001b}[0m\n", self.year);
        for day in &self.days {
           tmp.push_str(&format!("{}", day)) 
        }
        formatter.write_str(&tmp)
    }
}

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
        let mut tmp = format!("|--- {}\n", self.day);
        for part in &self.parts {
           tmp.push_str(&format!("{}", part)) 
        }
        formatter.write_str(&tmp)
    }
}
#[derive(Clone)]
#[allow(dead_code)]
pub enum Answer {
    None,
    Unsigned(u64),
    Signed(i64),
    Float(f64),
    Text(String),
} 

impl std::fmt::Display for Answer {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str({
                    match self{
                        Answer::None => "???".to_string(),
                        Answer::Unsigned(x) => x.to_string(),
                        Answer::Signed(x) => x.to_string(),
                        Answer::Float(x) => x.to_string(),
                        Answer::Text(x) => x.clone(),
                    }
                }.as_str())
    }
}

pub struct Part {
    part: String,
    answers: Vec<Answer>,
}

impl Part {
    pub fn new(part_id: &str, answers: Vec<Answer>) -> Part 
    {
        Part {
            part: part_id.to_string(),
            answers 
        }
    }
}

impl std::fmt::Display for Part {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.answers.len() > 1 {
            let mut tmp = format!("|---|--- {}\n", self.part);
            for ans in &self.answers {
                tmp.push_str(&format!("|---|---|--- \u{001b}[32m\u{001b}[7m[{}]\u{001b}[0m\n", *ans)) 
            }
            formatter.write_str(&tmp)
        } else if self.answers.len() == 1 {
                formatter.write_str(&format!("|---|--- {}:\t\u{001b}[32m\u{001b}[7m[{}]\u{001b}[0m\n", self.part, self.answers[0]))
        } else {
            formatter.write_str(&format!("|---|--- {}:\t\u{001b}[32m\u{001b}[7m[???]\u{001b}[0m\n", self.part))
        }
    }
}

/*
TODO: Each day should get it's own input
*/