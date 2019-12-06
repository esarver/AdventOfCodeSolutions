// #[macro_use] 
// extern crate lazy_static;

use crate::lib::common::answer::Answer;
use crate::lib::common::day::Day;
use crate::lib::common::part::Part;
use regex::Regex;
use regex::RegexSet;

#[test]
fn part_a() {
    use crate::lib::common::input;
    println!("Input: [{}]", input::get_input("2015", "5"));
    let part_a = part_a_answer(input::get_input("2015", "5").as_str());
    assert_eq!(part_a, Answer::Unsigned(238));
}

// #[test]
// fn part_b() {
//     use crate::lib::common::input;
//     let part_b = part_b_answer(input::get_input("2015", "5").as_str());
//     assert_eq!(part_b, Answer::Unsigned(9_958_218));
// }

#[test]
fn vowel_check_test() {
    assert_eq!(true, vowel_check("At least three vowels") );
    assert_eq!(false, vowel_check("nope"));
}

#[test]
fn double_letters_check_test() {
    assert_eq!(true, double_letters_check("double letters true"));
    assert_eq!(false, double_letters_check("double leters false"));
}

#[test]
fn forbidden_substring_check_test() {
    assert_eq!(true, forbidden_substring_check("absolutely"));
    assert_eq!(true, forbidden_substring_check("contains cd"));
    assert_eq!(true, forbidden_substring_check("contains pq"));
    assert_eq!(true, forbidden_substring_check("contains xy"));
    assert_eq!(false, forbidden_substring_check("Does not contain a-b, c-d, p-q, or x-y."));
}

pub fn day05(input: &str) -> Day {
    let mut day = Day::new(5);

    day.add_part({
        Part::new("Part A", {
            let mut ret = Vec::new();
            ret.push(part_a_answer(input));
            ret
        })
    });
    // day.add_part({
    //     Part::new("Part B", {
    //         let mut ret = Vec::new();
    //         ret.push(part_b_answer(input));

    //         ret
    //     })
    // });

    day
}

fn part_a_answer(input: &str) -> Answer {
    let mut nice_lines = 0;
    for line in input.lines() {
        if is_string_nice(line) {
            nice_lines += 1;
        }
    }

    Answer::Unsigned(nice_lines)
}

// fn part_b_answer(input: &str) -> Answer {
//     Answer::None
// }

fn is_string_nice(string: &str) -> bool {
    (vowel_check(string) && double_letters_check(string) && !forbidden_substring_check(string))
}

fn vowel_check(string: &str) -> bool {
    let re = Regex::new(r"([aeiou])").unwrap();

    let num_found = re.captures_iter(string).count();

    if num_found >= 3 {
        return true;
    }

    false
}

fn double_letters_check(string: &str) -> bool {
    let mut prev_char = ' ';
    for c in string.chars() {
        if c == prev_char {
            return true;
        }
        prev_char = c;
    }

    false
}

fn forbidden_substring_check(string: &str) -> bool {
    let forbidden = RegexSet::new(&[
        r"(ab)",
        r"(cd)",
        r"(pq)",
        r"(xy)",
    ]).unwrap();

    let num_found = forbidden.matches(string).iter().count();

    if num_found >= 1 {
        return true
    }

    false
}