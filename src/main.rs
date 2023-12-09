use std::fs;

mod day1;

fn main() {
    println!("{}", day1::part1(&fs::read_to_string("data/day1.txt").unwrap()));
}
