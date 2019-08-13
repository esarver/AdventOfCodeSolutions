pub fn day01(input: &str) -> i64 {

    let mut santa: Santa;

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
    fn up(&self);
    fn down(&self);
}

struct Santa {
    floor: i64,
    steps: u64
}

impl Santa {
    fn new(starting_floor: i64 = 0){

    }
}

impl Movement for Santa {
    fn up(&self) {
        self.floor = self.floor + 1;
    }

    fn down(&self) {
        self.floor = self.floor - 1;
    }
}

