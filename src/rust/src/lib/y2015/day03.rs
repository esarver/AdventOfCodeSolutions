use crate::lib::common::answer::Answer;
use crate::lib::common::day::Day;
use crate::lib::common::part::Part;

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
