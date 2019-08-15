
struct Year {
    year: String,
    days: Vec<Day>,
}

struct Day {
    day: String,
    parts: Vec<Part>,
    input: String,
}

struct Part {
    part: String,
    answer: i64,
}

/*
TODO: Each day should get it's own inputer
TODO: A Year should handle it's own printing by implementing the fmt trait
TODO: The answer for a part should take a generic that is bounded by a "ToString" trait.

*/