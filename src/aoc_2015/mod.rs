use std::{fs, vec};

pub fn day_1() {
    let raw_input = fs::read_to_string("src/aoc_2015/day1_input").unwrap();
    let mut start_floor: i32 = 0;
    let mut negative_floor = None;

    let input_vec = raw_input.split("");
    input_vec.enumerate().for_each(|(i, s)| {
        if s == "(" {
            start_floor += 1;
        } else if s == ")" {
            start_floor -= 1;
        };


        if start_floor == -1 && negative_floor.is_none() {
            negative_floor = Some(i);
        }
    });

    println!("Final floor is {}", start_floor);
    println!("Negative floor is {}", negative_floor.unwrap());
}

#[derive(Debug)]
struct Dimension {
    length: i32,
    width: i32,
    height: i32,
}

pub fn day_2() {
    let raw_input = fs::read_to_string("src/aoc_2015/day2_input").unwrap();

    let dimensions: Vec<Dimension> = raw_input
        .lines()
        .filter_map(|line| {
            // Length, width, height
            let parts: Vec<i32> = line.split("x").filter_map(|p| p.parse().ok()).collect();
            if parts.len() == 3 {
                Some(Dimension {
                    length: parts[0],
                    width: parts[1],
                    height: parts[2],
                })
            } else {
                // noop
                None
            }
        })
        .collect();

    let total_area: i32 = dimensions
        .iter()
        .map(|d| {
            let s_1 = d.length * d.width;
            let s_2 = d.width * d.height;
            let s_3 = d.height * d.length;

            let total = 2 * (s_1 + s_2 + s_3);
            let smallest_side = std::cmp::min(s_1, std::cmp::min(s_2, s_3));
            let answer = total + smallest_side;
            println!("Answer is {:?}", answer);
            println!("Question is {:?}", d);
            println!("Dim is {} {} {}", s_1, s_2, s_3);
            answer
        })
        .sum();
    println!("{:?}", total_area);
}

pub fn run_2015() {
    day_1();
}
