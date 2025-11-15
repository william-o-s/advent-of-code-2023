use std::fs;

mod part1;
mod part2;

fn part1() {
    let filepath = "src/input.txt";

    let contents = fs::read_to_string(filepath)
        .expect("Please provide a valid input.txt file.");

    let mut total = 0;
    for substring in contents.lines() {
        total += part1::parse_using_vec(substring).unwrap_or(0);
    }

    println!("Total = {total}");
}

fn part2() {
    let filepath = "src/input.txt";

    let contents = fs::read_to_string(filepath)
        .expect("Please provide a valid input.txt file.");

    let mut total = 0;
    for substring in contents.lines() {
        total += part2::parse_str(substring);
    }

    println!("Total = {total}");
}

fn main() {
    println!("-- Part 1 --");
    part1();

    println!("-- Part 2 --");
    part2();
}
