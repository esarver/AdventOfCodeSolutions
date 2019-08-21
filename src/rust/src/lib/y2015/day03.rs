use crate::lib::common::answer::Answer;
use crate::lib::common::day::Day;
use crate::lib::common::part::Part;
use std::collections::HashMap;

#[test]
fn santa_movement() {
    let mut santa = Santa::new();
    assert_eq!(santa.current_location, Coordinates(0,0));

    santa.move_direction('^');
    assert_eq!(santa.current_location, Coordinates(0,1));

    santa.move_direction('v');
    assert_eq!(santa.current_location, Coordinates(0,0));

    santa.move_direction('>');
    assert_eq!(santa.current_location, Coordinates(1,0));

    santa.move_direction('<');
    assert_eq!(santa.current_location, Coordinates(0,0));
}

pub fn day03(_input: &str) -> Day {
    let mut day = Day::new(3);

    day.add_part({
        Part::new("Part A", {
            let mut ret = Vec::new();
            ret.push(Answer::None);

            ret
        })
    });
    day.add_part({
        Part::new("Part B", {
            let mut ret = Vec::new();
            ret.push(Answer::None);

            ret
        })
    });

    day
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
            '^' => self.current_location = Coordinates(self.current_location.0, self.current_location.1 + 1),
            '>' => self.current_location = Coordinates(self.current_location.0 + 1, self.current_location.1),
            'v' => self.current_location = Coordinates(self.current_location.0, self.current_location.1 - 1),
            '<' => self.current_location = Coordinates(self.current_location.0 - 1, self.current_location.1),
            _ => panic!("Unknown input in file!"),
        }

        match self.visited_locations.get_mut(&self.current_location) {
            Some(x) => {*x += 1;}
            _ => {self.visited_locations.insert(self.current_location.clone(), 1);},
        }
    }
}