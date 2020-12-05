use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day03.txt").unwrap());
    let input: Vec<Vec<bool>> = reader.lines().map(|l| l.unwrap().chars().map(|y| y == '#').collect()).collect();
    let res = mult(&input);
    println!("Result of day03 p2: {}", res);
}

pub fn calc(input: &Vec<Vec<bool>>, xstep:usize, ystep:usize) -> u64 {
    let (mut x, mut y) = (0,0);
    let mut trees = 0;
    while y < input.len() - 1 {
        x = (x + xstep) % input[0].len();
        y += ystep;
        if input[y][x] {
            trees += 1;
        }
    }
    trees
}

fn mult(input: &Vec<Vec<bool>>) -> u64 {
    let mut res = calc(input, 1, 1);
    res *= calc(input, 3, 1);
    res *= calc(input, 5, 1);
    res *= calc(input, 7, 1);
    res *= calc(input, 1, 2);
    res
}

#[allow(unused_imports)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[test]
    fn test_calc() {
        let reader = BufReader::new(File::open("data/input_day03_test.txt").unwrap());
        let input: Vec<Vec<bool>> = reader.lines().map(|l| l.unwrap().chars().map(|y| y == '#').collect()).collect();
        assert_eq!(super::mult(&input), 336);
    }
}