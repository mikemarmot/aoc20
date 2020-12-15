use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day13.txt").unwrap());
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let res = calc(&input);
    println!("Result of day13 p1: {}", res);
}

fn calc(input: &Vec<String>) -> u32 {
    let start = input[0].parse::<u32>().unwrap();
    let table = input[1].split(",")
                    .filter(|&x| x != "x")
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
    let mut res:(u32, u32) = (0,0);
    for e in table {
        let next_t = (start / e + 1) * e;
        let diff = next_t - start;
        if res.1 == 0 || diff < res.1 {
            res = (e,diff);
        }
    }
    res.0 * res.1
}

mod tests {
    #[test]
    fn test_calc() {
        let input:Vec<String> = vec![
            String::from("939"),
            String::from("7,13,x,x,59,x,31,19"),
        ];
        assert_eq!(super::calc(&input), 295);
    }
}