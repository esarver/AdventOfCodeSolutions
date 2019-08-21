use crate::lib::common::answer::Answer;
use crate::lib::common::day::Day;
use crate::lib::common::part::Part;

#[test]
fn part_a() {
    use crate::lib::common::input;
    let part_a = part_a_answer(input::get_input("2015", "4").as_str());
    assert_eq!(part_a, Answer::None);
}

#[test]
fn part_b() {
    use crate::lib::common::input;
    let part_b = part_b_answer(input::get_input("2015", "4").as_str());
    assert_eq!(part_b, Answer::None);
}

pub fn day04(input: &str) -> Day {
    let mut day = Day::new(4);

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

fn part_a_answer(_input: &str) -> Answer {
    Answer::None
}

fn part_b_answer(_input: &str) -> Answer {
    Answer::None
}