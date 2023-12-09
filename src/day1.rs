pub fn part1(file: &String) -> usize {
    let mut first = 0;
    let mut last = 0;
    let mut fst_set = false;
    let mut sum = 0;
    for char in file.bytes() {
        let c = char as char;
        if c.is_ascii_digit() {
            last = c.to_digit(10).unwrap() as usize;
            if !fst_set {
                first = last * 10;
                fst_set = true;
            }
        } else if c == '\n' {
            sum = sum + first + last;
            fst_set = false;
        }
    }

    sum
}

#[cfg(test)]
mod tests
{
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::fs;

    #[test]
    fn test_part1_answer() {
        assert_eq!(part1(&fs::read_to_string("data/day1.txt").unwrap()), 55029);
    }
}