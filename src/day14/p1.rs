use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day14.txt").unwrap());
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let res = calc(&input);
    println!("Result of day14 p1: {}", res);
}

fn calc(input: &Vec<String>) -> usize {
    let mut mask:HashMap<usize, usize> = HashMap::new();
    let mut mem:HashMap<usize, usize> = HashMap::new();
    for line in input {
        if line.starts_with("mask") {
            mask = line
                    .chars()
                    .rev()
                    .take(36)
                    .enumerate()
                    .filter(|&(_,v)| v != 'X')
                    .map(|(i,v)| (i, v.to_digit(10).unwrap() as usize))
                    .collect::<HashMap<usize, usize>>();

        } else if line.starts_with("mem") {
            let (p1,p2) = (line.find(|c| c == '[').unwrap(), line.find(|c| c == ']').unwrap());
            let address = line[p1+1..p2].parse::<usize>().unwrap();
            let mut val = line.split(" = ").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
            for (p, v) in mask.iter() {
                let m = 1 << p;
                if *v == 0 {
                    if val & m != 0 { val ^= m }
                } else {
                    val |= m;
                }
            }
            &mem.insert(address, val);
        }
    }
    mem.values().sum()
}

mod tests {
    #[test]
    fn test_calc() {
        let input:Vec<String> = vec![
            String::from("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"),
            String::from("mem[8] = 11"),
            String::from("mem[7] = 101"),
            String::from("mem[8] = 0"),
        ];
        assert_eq!(super::calc(&input), 165);
    }
}