use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day05.txt").unwrap());
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let res = calc(&input);
    println!("Result of day05 p2: {}", res.unwrap());
}

pub fn calc(input: &Vec<String>) -> Option<usize> {
    let mut vres:Vec<usize> = input.iter().map(|x| scalc(x)).collect();
    vres.sort();
    let mut res:Option<usize> = None;
    for i in 1..vres.len() {
        if vres[i-1] != vres[i] - 1 {
            res = Some(vres[i-1] + 1);
            break;
        }
    }
    res
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
}