use std::fs;

pub fn day_1() {
    let raw_input = fs::read_to_string("src/aoc_2015/day1_input").unwrap();
    let mut start_floor = 0;
    let input_vec = raw_input.split("");
    input_vec.for_each(|s| {
        if s == "(" {
            start_floor += 1;
        } else if s == ")" {
            start_floor -= 1;
        };
    });

    println!("Final floor is {}", start_floor);
}
