use std::io::{BufRead, BufReader, Read};
use std::fs::File;

pub fn part1() {
    let f = File::open("data/day1.txt").unwrap();
    let reader = BufReader::new(f);
    let mut first = 0;
    let mut last = 0;
    let mut fst_set = false;
    let mut sum = 0;
    for char in reader.bytes() {
        let c = char.unwrap() as char;
        if c.is_ascii_digit() {
            last = c.to_digit(10).unwrap();
            if !fst_set {
                first = last * 10;
                fst_set = true;
            }
        } else if c == '\n' {
            sum = sum + first + last;
            fst_set = false;
        }
    }

    println!("{}", sum);
}