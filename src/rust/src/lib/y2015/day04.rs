use crate::lib::common::answer::Answer;
use crate::lib::common::day::Day;
use crate::lib::common::part::Part;
extern crate md5;

#[test]
#[ignore]
fn given_a() {
    assert_eq!(part_a_answer("abcdef"), Answer::Unsigned(609_043));
}

#[test]
#[ignore]
fn given_b() {
    assert_eq!(part_a_answer("pqrstuv"), Answer::Unsigned(1_048_970));
}

#[test]
#[ignore]
fn test_md5_a() {
    let to_test = "abcdef609043".to_string();
    let hash = calculate_hash(to_test.as_str());

    println!("Hash: {}",hash);
    println!("First 5: {}", hash.split_at(5).0);
}

#[test]
fn part_a() {
    use crate::lib::common::input;
    println!("Input: [{}]", input::get_input("2015", "4"));
    let part_a = part_a_answer(input::get_input("2015", "4").as_str());
    assert_eq!(part_a, Answer::Unsigned(346_386));
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

fn part_a_answer(input: &str) -> Answer {
    Answer::Unsigned(find_lowest_positive_number(input.trim(), 5))
}

fn part_b_answer(input: &str) -> Answer {
    Answer::Unsigned(find_lowest_positive_number(input.trim(), 6))
}

fn find_lowest_positive_number(secret_key: &str, num_leading_zeroes: u8) -> u64 {
    for x in 0..u32::max_value() {
        let test_val = format!("{}{}", secret_key, x); 
        let hash = calculate_hash(test_val.as_str());
        let (interest, _) = hash.split_at(usize::from(num_leading_zeroes));
        let t = "0".repeat(usize::from(num_leading_zeroes));
        if t == interest {
            return u64::from(x);
        }

    }
    panic!("No value satisfies the rules!");
}

fn calculate_hash(to_hash: &str) -> String {
    format!("{:X}",md5::compute(to_hash.as_bytes()))
}