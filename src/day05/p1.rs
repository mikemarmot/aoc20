use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day05.txt").unwrap());
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let res = calc(&input);
    println!("Result of day05 p1: {}", res);
}

pub fn calc(input: &Vec<String>) -> usize {
    input.iter().map(|x| scalc(x)).max().unwrap()
}

fn scalc(input:&str) -> usize {
    let mut m:usize = 0;
    for c in input.chars() {
        m = m << 1;
        m = match c {
            'B' | 'R' =>  m | 1,
            _ => m,
        };
    }
    m
}

mod tests {
    #[test]
    fn test_scalc() {
        assert_eq!(super::scalc("FBFBBFFRLR"), 357);
        assert_eq!(super::scalc("BFFFBBFRRR"), 567);
        assert_eq!(super::scalc("FFFBBBFRRR"), 119);
        assert_eq!(super::scalc("BBFFBBFRLL"), 820);
    }
    #[test]
    fn test_calc() {
        let input:Vec<String> = vec![
            String::from("BFFFBBFRRR"),
            String::from("FFFBBBFRRR"),
            String::from("BBFFBBFRLL"),
        ];
        assert_eq!(super::calc(&input), 820);
    }
}