use std::collections::HashMap;
use std::fs;

fn parse_input() -> Vec<Vec<u8>> {
    fs::read_to_string("input")
        .expect("File not found")
        .lines()
        .map(|s| {
            s.chars()
                .map(|bit| bit.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

fn solve_1() -> u32 {
    let report: Vec<Vec<u8>> = parse_input();

    let mut bits = HashMap::new();

    let report_len = report.len();
    let number_len = report[0].len();

    for number in report {
        for i in 0..number_len {
            let key = format!("{}-{}", i, number[i]);
            let count = bits.entry(key).or_insert(0 as u32);
            *count += 1;
        }
    }

    let mut gamma_rate: u32 = 0;
    let mut epsilon_rate: u32 = 0;

    for i in 0..number_len {
        let one_count = bits.get(&format!("{}-1", i)).unwrap();
        let zero_count = report_len as u32 - one_count;
        let bit_value = 2_u32.pow(number_len as u32 - i as u32 - 1);

        if zero_count > *one_count {
            epsilon_rate += bit_value;
        } else {
            gamma_rate += bit_value;
        }
    }

    gamma_rate * epsilon_rate
}

fn solve_2() -> u32 {
    let report: Vec<Vec<u8>> = parse_input();

    let oxygen_generator_rating = calc_rating(&report, 0, true);
    let co2_scubber_rating = calc_rating(&report, 0, false);

    oxygen_generator_rating * co2_scubber_rating
}

fn calc_rating(report: &Vec<Vec<u8>>, bit: usize, fewer: bool) -> u32 {
    let mut zeros = 0;
    let mut ones = 0;

    for number in report {
        match number[bit] {
            0 => zeros += 1,
            1 => ones += 1,
            _ => (),
        }
    }

    let item = if zeros > ones {
        if fewer {
            1
        } else {
            0
        }
    } else {
        if fewer {
            0
        } else {
            1
        }
    };

    let report: Vec<Vec<u8>> = report.iter().filter(|r| r[bit] == item).cloned().collect();

    if report.len() == 1 {
        let bit_size = report[0].len();
        let mut result: u32 = 0;

        for i in 0..bit_size {
            if report[0][i] == 1 {
                result += 2_u32.pow(bit_size as u32 - i as u32 - 1);
            }
        }
        return result;
    }

    calc_rating(&report, bit + 1, fewer)
}

fn main() {
    let day3_part_1 = solve_1();
    let day3_part_2 = solve_2();

    println!(
        "Day 3:\n\tPart 1: {}\n\tPart 2: {}",
        day3_part_1, day3_part_2
    );
}
