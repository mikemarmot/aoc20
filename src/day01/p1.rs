use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day01.txt").unwrap());
    let input: Vec<i32> = reader.lines().map(|l| l.unwrap().parse::<i32>().unwrap()).collect();
    let res = calc(&input);
    println!("Result of day01 p1: {}", res);
}

pub fn calc(input: &Vec<i32>) -> i32 {
    let mut res: i32 = 0;
    for (i, val) in input.into_iter().enumerate() {
        match input[i+1..].into_iter().find(|&&x| x == 2020 - val) {
            Some(x) => res = x * *val,
            None => ()
        }
    }
    res
}

mod tests {
    #[test]
    fn test_calc() {
        assert!(super::calc(&vec![1721, 979, 366, 299, 675, 1456]) == 514579);
    }
}