use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day06.txt").unwrap());
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let res = calc(&input);
    println!("Result of day06 p2: {}", res);
}

pub fn calc(input: &Vec<String>) -> usize {
    let mut pp:Vec<HashSet<char>> = Vec::new();
    let mut count:usize = 0;
    for line in input {
        if pp.len() < count + 1 {
            pp.push(HashSet::new());
        }
        if line.trim().len() > 0 {
            for c in line.chars() {
                if c != ' ' {
                    pp[count].insert(c);
                }
            }
        } else {
            count += 1;
        }
    }
    pp.iter().map(|x| x.len()).sum()
}

mod tests {
    #[test]
    fn test_calc() {
        let input:Vec<String> = vec![
            String::from("abc"),
            String::from(""),
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from(""),
            String::from("ab"),
            String::from("ac"),
            String::from(""),
            String::from("a"),
            String::from("a"),
            String::from("a"),
            String::from("a"),
            String::from(""),
            String::from("b"),
        ];
        assert_eq!(super::calc(&input), 11);
    }
}