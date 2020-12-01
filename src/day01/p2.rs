use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day01.txt").unwrap());
    let input: Vec<i32> = reader.lines().map(|l| l.unwrap().parse::<i32>().unwrap()).collect();
    let res = calc(&input);
    println!("Result of day01 p2: {}", res);
}

pub fn calc(input: &Vec<i32>) -> i32 {
    let mut res: i32 = 0;
    for (i, val1) in input.into_iter().enumerate() {
        for (j, val2) in input[i+1..].into_iter().enumerate() {
            match input[j+1..].into_iter().find(|&&x| x == 2020 - val1 - val2) {
                Some(x) => res = x * *val1 * *val2,
                None => ()
            }
        }
    }
    res
}

mod tests {
    #[test]
    fn test_calc() {
        assert!(super::calc(&vec![1721, 979, 366, 299, 675, 1456]) == 241861950);
    }
}