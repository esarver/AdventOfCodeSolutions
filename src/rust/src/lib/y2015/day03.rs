use crate::lib::common::answer::Answer;
use crate::lib::common::day::Day;
use crate::lib::common::part::Part;
use std::collections::HashMap;

#[test]
#[ignore]
fn santa_movement() {
    let mut santa = Santa::new();
    assert_eq!(santa.current_location, Coordinates(0, 0));

    santa.move_direction('^');
    assert_eq!(santa.current_location, Coordinates(0, 1));

    santa.move_direction('v');
    assert_eq!(santa.current_location, Coordinates(0, 0));

    santa.move_direction('>');
    assert_eq!(santa.current_location, Coordinates(1, 0));

    santa.move_direction('<');
    assert_eq!(santa.current_location, Coordinates(0, 0));
}

#[test]
#[ignore]
fn visited_houses() {
    let mut santa = Santa::new();
    let input = "^>v<^"; // Should only be 4 total housed

    for mv in input.chars() {
        santa.move_direction(mv);
    }

    assert_eq!(santa.number_visited_houses(), 4);
}

#[test]
fn part_a() {
    use crate::lib::common::input;
    let part_a = part_a_answer(input::get_input("2015", "3").as_str());
    assert_eq!(part_a, Answer::Unsigned(2081));
}

#[test]
fn part_b() {
    use crate::lib::common::input;
    let part_b = part_b_answer(input::get_input("2015", "3").as_str());
    assert_eq!(part_b, Answer::Unsigned(2341));
}

pub fn day03(input: &str) -> Day {
    let mut day = Day::new(3);

    day.add_part({
        Part::new("Part A", {
            let mut ret = Vec::new();
            ret.push(part_a_answer(input));
            ret
        })
    });
    day.add_part({
        Part::new("Part B", {
            let mut ret = Vec::new();
            ret.push(part_b_answer(input));

            ret
        })
    });

    day
}

fn part_a_answer(input: &str) -> Answer {
    let mut santa = Santa::new();
    for movement in input.chars() {
        santa.move_direction(movement);
    }
    Answer::Unsigned(santa.number_visited_houses())
}

fn part_b_answer(input: &str) -> Answer {
    let mut santa = Santa::new();
    let mut robo_santa = Santa::new();

    for (i, movement) in input.chars().enumerate() {
        if (i % 2) == 0 {
            santa.move_direction(movement);
        } else {
            robo_santa.move_direction(movement);
        }
    }

    Answer::Unsigned(get_unique_visited(&santa, &robo_santa))
}

fn get_unique_visited(santa_1: &Santa, santa_2: &Santa) -> u64 {
    let mut merged = santa_1.visited_locations.clone();
    merged.extend(santa_2.visited_locations.clone());

    merged.keys().len() as u64
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Coordinates(i64, i64);

struct Santa {
    current_location: Coordinates,
    visited_locations: HashMap<Coordinates, u64>,
}

impl Santa {
    fn new() -> Santa {
        let mut visited_locations = HashMap::new();
        visited_locations.insert(Coordinates(0, 0), 1);
        Santa {
            current_location: Coordinates(0, 0),
            visited_locations,
        }
    }

    fn move_direction(&mut self, direction: char) {
        match direction {
            '^' => {
                self.current_location =
                    Coordinates(self.current_location.0, self.current_location.1 + 1)
            }
            '>' => {
                self.current_location =
                    Coordinates(self.current_location.0 + 1, self.current_location.1)
            }
            'v' => {
                self.current_location =
                    Coordinates(self.current_location.0, self.current_location.1 - 1)
            }
            '<' => {
                self.current_location =
                    Coordinates(self.current_location.0 - 1, self.current_location.1)
            }
            _ => panic!("Unknown input in file!"),
        }

        match self.visited_locations.get_mut(&self.current_location) {
            Some(x) => {
                *x += 1;
            }
            _ => {
                self.visited_locations
                    .insert(self.current_location.clone(), 1);
            }
        }
    }

    fn number_visited_houses(&self) -> u64 {
        self.visited_locations.keys().len() as u64
    }
}
