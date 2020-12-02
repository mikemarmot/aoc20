use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day02.txt").unwrap());
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let res = calc(&input);
    println!("Result of day02 p1: {}", res);
}

pub fn calc(input: &Vec<String>) -> u32 {
    let mut res: u32 = 0;
    for line in input {
        //println!("process: {}", line);
        let data:Vec<&str> = line.split(" ").collect();
        let lim:Vec<u16> = data[0].split("-").map(|v| v.parse::<u16>().unwrap()).collect();

        if check(data[2], data[1].chars().collect::<Vec<char>>()[0], lim[0], lim[1]) {
            res += 1;
        }
    }
    res
}

fn check(pw: &str, cs: char, min: u16, max: u16) -> bool {
    let mut hmap: HashMap<char, u16> = HashMap::new();
    hmap.insert(cs, 0);
    for cp in pw.chars() {
        if !hmap.contains_key(&cp) {
            hmap.insert(cp, 0);
        }
        *hmap.get_mut(&cp).unwrap() += 1;
    }
    *hmap.get(&cs).unwrap() >= min && *hmap.get(&cs).unwrap() <= max
}

mod tests {
    #[test]
    fn test_check() {
        assert!(super::check("abcde", 'a', 1, 3));
        assert!(!super::check("cdefg", 'b', 1, 3));
        assert!(super::check("ccccccccc", 'c', 2, 9));
    }

    #[test]
    fn test_calc() {
        assert!(super::calc(&vec![
            String::from("1-3 a: abcde"),
            String::from("1-3 b: cdefg"),
            String::from("2-9 c: ccccccccc")
        ]) == 2);
    }
}