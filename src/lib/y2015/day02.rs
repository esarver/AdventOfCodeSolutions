use crate::lib::common::answer::Answer;
use crate::lib::common::day::Day;
use crate::lib::common::part::Part;

#[test]
#[ignore]
fn needed_surface_area_test() {
    let input = "2x2x2\n3x3x3\n";
    let presents: Vec<Present> = generate_presents(input);
    assert_eq!(presents.len(), 2);

    let sa_1 = presents[0].surface_area_needed();
    assert_eq!(sa_1, 28);

    let sa_2 = presents[1].surface_area_needed();
    assert_eq!(sa_2, 63);

    let total = total_surface_area(presents);
    assert_eq!(total, 91);
}

#[test]
#[ignore]
fn needed_ribbon_test() {
    let input = "2x2x2\n3x3x3\n";
    let presents: Vec<Present> = generate_presents(input);
    assert_eq!(presents.len(), 2);

    let ribbon_1 = presents[0].ribbon_needed();
    assert_eq!(ribbon_1, 16);

    let ribbon_2 = presents[1].ribbon_needed();
    assert_eq!(ribbon_2, 39);

    let total = total_ribbon_needed(presents);
    assert_eq!(total, 55);
}

#[test]
fn part_a() {
    use crate::lib::common::input;

    let part_a = part_a_answer(input::get_input("2015", "2").as_str());
    assert!(part_a == Answer::Unsigned(1_598_415));
}

#[test]
fn part_b() {
    use crate::lib::common::input;

    let part_b = part_b_answer(input::get_input("2015", "2").as_str());
    assert!(part_b == Answer::Unsigned(3_812_909));
}

pub fn day02(input: &str) -> Day {
    let mut day = Day::new(2);

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
    Answer::Unsigned(total_surface_area(generate_presents(input)))
}

fn part_b_answer(input: &str) -> Answer {
    Answer::Unsigned(total_ribbon_needed(generate_presents(input)))
}

fn generate_presents(input: &str) -> Vec<Present> {
    let mut presents: Vec<Present> = Vec::new();
    for line in input.lines() {
        presents.push(Present::new(line));
    }

    presents
}

fn total_surface_area(presents: Vec<Present>) -> u64 {
    let mut total: u64 = 0;
    for present in presents {
        total += present.surface_area_needed();
    }

    total
}

fn total_ribbon_needed(presents: Vec<Present>) -> u64 {
    let mut total: u64 = 0;
    for present in presents {
        total += present.ribbon_needed();
    }

    total
}

struct Present {
    sides: Vec<u64>,
}

impl Present {
    fn new(dimension_string: &str) -> Present {
        let mut sides: Vec<u64> = Vec::new();
        for dimension in dimension_string.split('x') {
            match dimension.parse::<u64>() {
                Ok(x) => sides.push(x),
                _ => panic!("Could not parse box size!"),
            }
        }

        if sides.len() != 3 {
            panic!("Box had the incorrect number of dimensions.");
        }

        sides.sort();

        Present { sides }
    }

    fn surface_area_needed(&self) -> u64 {
        let (l, w, h) = (self.sides[0], self.sides[1], self.sides[2]);

        (2 * l * w) + (2 * w * h) + (2 * h * l) + (l * w)
    }

    fn ribbon_needed(&self) -> u64 {
        let (l, w, h) = (self.sides[0], self.sides[1], self.sides[2]);

        ((2 * l) + (2 * w)) + (l * w * h)
    }
}
