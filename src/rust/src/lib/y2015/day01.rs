use crate::lib::common::logger;

pub fn day01(input: &str) {
    let mut santa = Santa::new(input.to_string(), None);

    logger::log_day("Day 01");
    logger::log_answer("Part A", santa.final_floor().to_string().as_str());
    logger::log_answer("Part B", santa.first_at_floor(-1).to_string().as_str());
}

trait Movement {
    fn up(&mut self);
    fn down(&mut self);
}

struct Santa {
    instructions: String,
    initial_floor: i64,
    current_floor: i64,
    steps: u64,
}

impl Santa {
    fn new(instructions: String, starting_floor: Option<i64>) -> Santa {
        Santa {
            current_floor: 0,
            initial_floor: { starting_floor.unwrap_or(0) },
            steps: 0,
            instructions,
        }
    }
    fn final_floor(&mut self) -> i64 {
        self.current_floor = self.initial_floor;
        self.steps = 0;

        let inst = self.instructions.clone();

        for instruction in inst.chars() {
            match instruction {
                '(' => self.up(),
                ')' => self.down(),
                _ => {}
            }
        }

        self.current_floor
    }
    fn first_at_floor(&mut self, floor_number: i64) -> u64 {
        self.current_floor = self.initial_floor;
        self.steps = 0;

        let inst = self.instructions.clone();

        for instruction in inst.chars() {
            if self.current_floor == floor_number {
                break;
            }
            match instruction {
                '(' => self.up(),
                ')' => self.down(),
                _ => {}
            }
        }

        {
            self.steps
        }
    }
}

impl Movement for Santa {
    fn up(&mut self) {
        self.current_floor = self.current_floor + 1;
        self.steps = self.steps + 1;
    }

    fn down(&mut self) {
        self.current_floor = self.current_floor - 1;
        self.steps = self.steps + 1;
    }
}
