use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day04.txt").unwrap());
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let res = calc(&input);
    println!("Result of day04 p1: {}", res);
}

pub fn calc(input: &Vec<String>) -> u32 {
    let fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];
    let mut pp:Vec<String> = Vec::new();
    let mut count:usize = 0;
    for line in input {
        if pp.len() < count + 1 {
            pp.push(String::new());
        }
        if line.trim().len() > 0 {
            pp[count] = format!("{} {}", pp[count], line);
        } else {
            count += 1;
        }
    }
    for p in &pp {
        println!("UUU {}", p.trim());
        let dd:Vec<&str> = p.trim().split(" ").collect();
        let mut hh:HashMap<&str, &str> = HashMap::new();
        for d in dd {
            println!("WWW {}", d);
            let cc:Vec<&str> = d.split(":").collect();
            println!("VVV {} {}", cc[0], cc[1]);
            hh.insert(cc[0], cc[1]);
        }

    }
    pp.len() as u32
}

mod tests {
    #[test]
    fn test_calc() {
        let input:Vec<String> = vec![
            String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd"),
            String::from("byr:1937 iyr:2017 cid:147 hgt:183cm"),
            String::from(""),
            String::from("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884"),
            String::from("hcl:#cfa07d byr:1929"),
            String::from(""),
            String::from("hcl:#ae17e1 iyr:2013"),
            String::from("eyr:2024"),
            String::from("ecl:brn pid:760753108 byr:1931"),
            String::from("hgt:179cm"),
            String::from(""),
            String::from("hcl:#cfa07d eyr:2025 pid:166559648"),
            String::from("iyr:2011 ecl:brn hgt:59in"),
        ];
        assert_eq!(super::calc(&input), 2);
    }
}