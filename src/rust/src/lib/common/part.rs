use super::answer::Answer;

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
                tmp.push_str(&format!("|---|---|--- {}\n", *ans)) 
            }
            formatter.write_str(&tmp)
        } else if self.answers.len() == 1 {
                formatter.write_str(&format!("|---|--- {}:\t{}\n", self.part, self.answers[0]))
        } else {
            formatter.write_str(&format!("|---|--- {}:\t\u{001b}[33m[???]\u{001b}[0m\n", self.part))
        }
    }
}

