use crate::lib::common::day::Day;
use crate::lib::common::part::Part;
use crate::lib::common::answer::Answer;

#[test]
fn final_floor_test_zero() {
    let mut santa = Santa::new("((()))".to_string(), None);

    assert_eq!(santa.final_floor(), 0);
}

#[test]
fn final_floor_test_positive() {
    let mut santa = Santa::new("((()))((((".to_string(), None);

    assert_eq!(santa.final_floor(), 4);
}

#[test]
fn final_floor_test_negative() {
    let mut santa = Santa::new("((()))))))))".to_string(), None);

    assert_eq!(santa.final_floor(), -6);
}

#[test]
fn first_at_floor_test() {
    let mut santa = Santa::new("((())))".to_string(), None);

    assert_eq!(santa.first_at_floor(-1), 7);
}

#[test]
fn first_at_floor_test_extra_steps() {
    let mut santa = Santa::new("))))".to_string(), None);

    assert_eq!(santa.first_at_floor(-2), 2);
}

pub fn day01(input: &str) -> Day {
    let mut day = Day::new(1);

    day.add_part(
        Part::new(
            "Part A",
            {
                let mut return_vec = Vec::new();
                let mut santa = Santa::new(input.to_string(), None);

                return_vec.push(Answer::Signed(santa.final_floor()));

                return_vec
            }
        )
    );

    day.add_part(
        Part::new(
            "Part B", 
            {
                let mut return_vec = Vec::new();
                let mut santa = Santa::new(input.to_string(), None);

                return_vec.push(Answer::Unsigned(santa.first_at_floor(-1)));

                return_vec
               
            }
            )
    );

    day
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
        self.current_floor += 1;
        self.steps += 1;
    }

    fn down(&mut self) {
        self.current_floor -= 1;
        self.steps += 1;
    }
}
