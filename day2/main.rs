use std::fs;

fn parse_input() -> Vec<(String, i32)> {
    fs::read_to_string("input")
        .expect("File not found")
        .lines()
        .map(|s| {
            let mut parts = s.split_whitespace();
            (
                String::from(parts.next().unwrap()),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn solve_1() -> i32 {
    let course: Vec<(String, i32)> = parse_input();

    let mut h_pos = 0;
    let mut depth = 0;

    for current in course {
        match current.0.as_str() {
            "forward" => h_pos += current.1,
            "down" => depth += current.1,
            "up" => depth -= current.1,
            _ => (),
        }
    }
    h_pos * depth
}

fn solve_2() -> i32 {
    let course: Vec<(String, i32)> = parse_input();

    let mut h_pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for current in course {
        match current.0.as_str() {
            "forward" => {
                h_pos += current.1;
                depth += aim * current.1;
            }
            "down" => aim += current.1,
            "up" => aim -= current.1,
            _ => (),
        }
    }
    h_pos * depth
}

fn main() {
    let day2_part_1 = solve_1();
    let day2_part_2 = solve_2();

    println!(
        "Day 2:\n\tPart 1: {}\n\tPart 2: {}",
        day2_part_1, day2_part_2
    );
}
