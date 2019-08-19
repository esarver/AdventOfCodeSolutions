
pub struct Year<'a> {
    year: String,
    days: Vec<Day<'a>>,
}
impl<'a> Year<'a> {
    pub fn new(year_number: u16) -> Year<'a> {
        Year {
            year: format!("{:4}", year_number),
            days: Vec::new(),
        }
    }
    pub fn add_day(&mut self, new_day: Day<'a>) {
        self.days.push(new_day);
    }
}
impl<'a> std::fmt::Display for Year<'a> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut tmp = format!("\u{001b}[4m[{}]\u{001b}[0m\n", self.year);
        for day in &self.days {
           tmp.push_str(&format!("{}", day)) 
        }
        formatter.write_str(&tmp)
    }
}

pub struct Day<'a> {
    day: String,
    parts: Vec<Part<'a>>,
    input: String,
}

impl<'a> Day<'a> {
    pub fn new(day_number: u8) -> Day<'a> {
        Day {
            day: format!("Day {:2}", day_number),
            parts: Vec::new(),
            input: "".to_string(),
        }
    }
    pub fn add_part(&self, new_part: Part<'a>) {
        self.parts.push(new_part);
    }
}

impl<'a> std::fmt::Display for Day<'a> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut tmp = format!("|--- {}\n", self.day);
        for part in &self.parts {
           tmp.push_str(&format!("{}", part)) 
        }
        formatter.write_str(&tmp)
    }
}

pub trait Answer: std::fmt::Display {} 

pub struct FuncBoxStruct<'a>(Box<dyn FnMut() + 'a>);

pub type FuncBox<'a> = FuncBoxStruct<'a>; 

pub struct Part<'a> {
    part: String,
    answer: Vec<Box<dyn Answer>>,
    closure: FuncBox<'a> 
}

impl<'a> Part<'a> {
    pub fn new<F>(part_id: &str, call_back: FuncBox<'a>) -> Part<'a> 
        where F: FnMut() -> ()
    {
        Part {
            part: part_id.to_string(),
            answer: Vec::new(),
            closure: call_back,
        }
    }
}

impl<'a> std::fmt::Display for Part<'a> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.answer.len() > 1 {
            let mut tmp = format!("|---|--- {}\n", self.part);
            for ans in &self.answer {
                tmp.push_str(&format!("|---|---|--- \u{001b}[32m\u{001b}[7m[{}]\u{001b}[0m\n", ans)) 
            }
            formatter.write_str(&tmp)
        } else {
            formatter.write_str(&format!("|---|--- {}:\t\u{001b}[32m\u{001b}[7m[{}]\u{001b}[0m\n", self.part, self.answer.first().unwrap()))
        }
    }
}

/*
TODO: Each day should get it's own input
*/