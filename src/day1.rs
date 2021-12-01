use std::fs;

fn parse_input() -> Vec<i32> {
    fs::read_to_string("input/day1")
        .expect("File not found")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn solve_1() -> i32 {
    let report: Vec<i32> = parse_input();

    let mut larger_measurements = 0;

    for current in 1..report.len() {
        if report[current] > report[current - 1] {
            larger_measurements += 1;
        }
    }
    larger_measurements
}

pub fn solve_2() -> i32 {
    let report: Vec<i32> = parse_input();

    let mut larger_measurements = 0;
    let total_windows = report.len() - 3;

    for w in 0..total_windows {
        let shared_window = report[w + 1] + report[w + 2];

        let curr_window = report[w] + shared_window;
        let next_window = shared_window + report[w + 3];

        if next_window > curr_window {
            larger_measurements += 1;
        }
    }
    larger_measurements
}
