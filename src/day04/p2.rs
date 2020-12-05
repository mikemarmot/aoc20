use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day04.txt").unwrap());
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let res = calc(&input);
    println!("Result of day04 p2: {}", res);
}

pub fn calc(input: &Vec<String>) -> usize {
    let fields = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
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
    count = 0;
    for p in &pp {
        let dd:Vec<&str> = p.trim().split(" ").collect();
        let mut hh:HashMap<&str, &str> = HashMap::new();
        for d in dd {
            let cc:Vec<&str> = d.split(":").collect();
            hh.insert(cc[0], cc[1]);
        }
        let mut valid = true;
        for f in &fields {
            valid = match hh.get(f) {
                Some(x) => validate(f, x),
                None => false
            };
            if !valid {
                break;
            }
        }
        if valid {
            count += 1;
        }
    }
    count
}

fn validate(n:&str, val:&str) -> bool {
    match n {
        "byr" => check_year(val.parse::<u16>().unwrap(), 1920, 2002),
        "iyr" => check_year(val.parse::<u16>().unwrap(), 2010, 2020),
        "eyr" => check_year(val.parse::<u16>().unwrap(), 2020, 2030),
        "hgt" => check_height(val),
        "hcl" => check_hair_color(val),
        "ecl" => check_eye_color(val),
        "pid" => check_pid(val),
        "cid" => true,
        _ => false
    }
}

fn check_year(y:u16, miny:u16, maxy:u16) -> bool {
    if y >= miny && y <= maxy {
        true
    } else {
        false
    }
}

fn check_height(height:&str) -> bool {
    let h = height[..height.len()-2].parse::<u16>().unwrap();
    if height.ends_with("cm") && h >= 150 && h <= 193 {
        true
    } else if height.ends_with("in") && h >= 59 && h <= 76 {
        true
    } else {
        false
    }
}

fn check_hair_color(hair_color:&str) -> bool {
    if hair_color.len() == 7 {
        match u64::from_str_radix(hair_color.trim_start_matches("#"),16) {
            Ok(_) => true,
            Err(_) => false
        }
    } else {
        false
    }
}

fn check_eye_color(eye_color:&str) -> bool {
    let vals = vec!["amb","blu","brn","gry","grn","hzl","oth"];
    vals.contains(&eye_color)
}

fn check_pid(val:&str) -> bool {
    if val.len() == 9 {
        match val.parse::<u32>() {
            Ok(_) => true,
            Err(_) => false
        }
    } else {
        false
    }
}

mod tests {
    #[test]
    fn test_calc_invalid() {
        let input:Vec<String> = vec![
            String::from("eyr:1972 cid:100"),
            String::from("hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"),
            String::from(""),
            String::from("iyr:2019"),
            String::from("hcl:#602927 eyr:1967 hgt:170cm"),
            String::from("ecl:grn pid:012533040 byr:1946"),
            String::from(""),
            String::from("hcl:dab227 iyr:2012"),
            String::from("ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"),
            String::from(""),
            String::from("hgt:59cm ecl:zzz"),
            String::from("eyr:2038 hcl:74454a iyr:2023"),
            String::from("pid:3556412378 byr:2007"),
        ];
        assert_eq!(super::calc(&input), 0);
    }

    #[test]
    fn test_calc_valid() {
        let input:Vec<String> = vec![
            String::from("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980"),
            String::from("hcl:#623a2f"),
            String::from(""),
            String::from("eyr:2029 ecl:blu cid:129 byr:1989"),
            String::from("iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"),
            String::from(""),
            String::from("hcl:#888785"),
            String::from("hgt:164cm byr:2001 iyr:2015 cid:88"),
            String::from("pid:545766238 ecl:hzl"),
            String::from("eyr:2022"),
            String::from(""),
            String::from("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"),
        ];
        assert_eq!(super::calc(&input), 4);
    }    
}