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

struct Dimension {
    length: i32,
    width: i32,
    height: i32,
}

pub fn day_2() {
    let raw_input = fs::read_to_string("src/aoc_2015/day2_input").unwrap();
    let input_vec = raw_input.split("\n");
    let rows = input_vec.for_each(|s| {
        // S is l,w,h
        // Formula for the wrapping is 2*l*w + 2*w*h + 2*h*l
        // Split more
        let h_w_l: Vec<Option<i32>> = s.split("x").map(|s| s.parse::<i32>().ok()).collect();
        println!("Vec {:?}", h_w_l);

        if (h_w_l.len() == 3) {
            let height = h_w_l.get(0);
            let width = h_w_l.get(1);
            let length = h_w_l.get(2);
        }
    });
}
