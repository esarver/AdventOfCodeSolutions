
struct Year {
    year: String,
    days: Vec<Day>,
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

struct Day {
    day: String,
    parts: Vec<Part>,
    input: String,
}

impl Day {
    fn new(day_number: u8) -> Day {
        Day {
            day: format!("Day {:2}", day_number),
            parts: Vec::new(),
            input: "".to_string(),
        }
    }
}

impl std::fmt::Display for Day {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut tmp = format!("|--- {}\n", self.day);
        for part in &self.parts {
           tmp.push_str(&format!("{}", part)) 
        }
        formatter.write_str(&tmp)
    }
}

trait Answer: std::fmt::Display {} 

struct Part {
    part: String,
    answer: Vec<Box<dyn Answer>>,
    closure: Box<dyn FnMut() -> ()>,
}

impl Part {
    fn new<F>(part_id: &str, call_back: F) -> Part 
        where F: FnMut() -> ()
    {
        Part {
            part: part_id.to_string(),
            answer: Vec::new(),
            closure: Box::new(call_back),
        }
    }
}

impl std::fmt::Display for Part {
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