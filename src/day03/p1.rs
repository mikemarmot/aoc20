use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day03.txt").unwrap());
    let input: Vec<Vec<bool>> = reader.lines().map(|l| l.unwrap().chars().map(|y| y == '#').collect()).collect();
    let res = calc(&input);
    println!("Result of day03 p1: {}", res);
}

pub fn calc(input: &Vec<Vec<bool>>) -> u32 {
    let (mut x, mut y) = (0,0);
    let mut trees = 0;
    while y < input.len() - 1 {
        x = (x + 3) % input[0].len();
        y += 1;
        if input[y][x] {
            trees += 1;
        }
    }
    trees
}

mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn test_calc() {
        let reader = BufReader::new(File::open("data/input_day03_test.txt").unwrap());
        let input: Vec<Vec<bool>> = reader.lines().map(|l| l.unwrap().chars().map(|y| y == '#').collect()).collect();
        assert_eq!(super::calc(&input), 7);
    }
}