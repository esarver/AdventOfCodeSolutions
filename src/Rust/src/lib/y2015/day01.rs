pub fn day01(input: &str) -> i64 {

    let mut santa = Santa::new(None);

    for instruction in input.chars() {
        match instruction {
            '(' => santa.up(),
            ')' => santa.down(),
            _ => {},
        }
    }

    santa.floor
}

trait Movement {
    fn up(&mut self);
    fn down(&mut self);
}

struct Santa {
    floor: i64,
    steps: u64
}

impl Santa {
    fn new(starting_floor: Option<i64>) -> Santa {
        Santa {
            floor: {starting_floor.unwrap_or(0)},
            steps: 0,
        }

    }
}

impl Movement for Santa {
    fn up(&mut self) {
        self.floor = self.floor + 1;
        self.steps = self.steps + 1;
    }

    fn down(&mut self) {
        self.floor = self.floor - 1;
        self.steps = self.steps + 1;
    }
}

