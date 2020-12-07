use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day07.txt").unwrap());
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let res = calc(&input, "shiny gold");
    println!("Result of day07 p2: {}", res);
}

pub fn calc(input: &Vec<String>, bag:&str) -> u32 {
    let mut rules:HashMap<&str, HashMap<&str, u8>> = HashMap::new();
    for line in input {
        let line:Vec<&str> = line.split(" bags contain ").collect();
        let mut vals:HashMap<&str, u8> = HashMap::new();
        for content in line[1].split(",") {
            let content = content.trim();
            if content != "no other bags." {
                let content:Vec<&str> = content.splitn(2, " ").collect();
                let m:u8 = content[0].parse::<u8>().unwrap();
                let n = content[1].split(" bag").collect::<Vec<&str>>()[0];
                vals.insert(n, m);
            }
        }
        rules.insert(line[0], vals);
    }
    scalc(&rules, bag)
}

fn scalc(rules:&HashMap<&str, HashMap<&str, u8>>, search:&str) -> u32 {
    let ee:&HashMap<&str, u8> = rules.get(search).unwrap();
    if ee.is_empty() {
        return 0;
    } else {
        let mut res:u32 = 0;
        for (ek, ev) in ee.iter() {
            res += *ev as u32 + *ev as u32 * scalc(rules, ek);
        }
        return res;
    }
}

mod tests {
    #[test]
    fn test_calc1() {
        let input:Vec<String> = vec![
            String::from("light red bags contain 1 bright white bag, 2 muted yellow bags."),
            String::from("dark orange bags contain 3 bright white bags, 4 muted yellow bags."),
            String::from("bright white bags contain 1 shiny gold bag."),
            String::from("muted yellow bags contain 2 shiny gold bags, 9 faded blue bags."),
            String::from("shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags."),
            String::from("dark olive bags contain 3 faded blue bags, 4 dotted black bags."),
            String::from("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags."),
            String::from("faded blue bags contain no other bags."),
            String::from("dotted black bags contain no other bags."),
        ];
        assert_eq!(super::calc(&input, "shiny gold"), 32);
    }
    #[test]
    fn test_calc2() {
        let input:Vec<String> = vec![
            String::from("shiny gold bags contain 2 dark red bags."),
            String::from("dark red bags contain 2 dark orange bags."),
            String::from("dark orange bags contain 2 dark yellow bags."),
            String::from("dark yellow bags contain 2 dark green bags."),
            String::from("dark green bags contain 2 dark blue bags."),
            String::from("dark blue bags contain 2 dark violet bags."),
            String::from("dark violet bags contain no other bags."),
        ];
        assert_eq!(super::calc(&input, "shiny gold"), 126);
    }
}