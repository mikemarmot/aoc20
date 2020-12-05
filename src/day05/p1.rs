use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day05.txt").unwrap());
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let res = calc(&input);
    println!("Result of day05 p1: {}", res);
}

pub fn calc(input: &Vec<String>) -> usize {
    let mut res:usize = 0;
    for line in input {
        let r = scalc(line);
        if r > res {
            res = r;
        }
    }
    res
}

fn scalc(input:&str) -> usize {
    let mut m:u8 = 0;
    for c in input[..7].chars() {
        m = m << 1;
        match c {
            'B' => m = m | 1,
            _ => ()
        }
    }
    let mut n:u8 = 0;
    for c in input[7..].chars() {
        n = n << 1;
        match c {
            'R' => n = n | 1,
            _ => ()
        }
    }
    let res = m as usize * 8 + n as usize;
    res as usize
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