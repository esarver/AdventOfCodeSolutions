use crate::lib::common::day::Day;
use crate::lib::common::part::Part;
use crate::lib::common::answer::Answer;

pub fn day02(_input: &str) -> Day {
    let mut day = Day::new(2);

    day.add_part({
       Part::new(
           "Part A", 
           {
               let mut ret = Vec::new();
                ret.push(Answer::None);

                ret
           }
        ) 
    });

    day
}