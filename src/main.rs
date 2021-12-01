mod day1;

fn main() {
    let day1_part_1 = day1::solve_1();
    let day1_part_2 = day1::solve_2();

    println!(
        "Day 1:\n\tPart 1: {}\n\tPart 2: {}",
        day1_part_1, day1_part_2
    );
}
