use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day07.txt").unwrap());
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let res = calc(&input, "shiny gold");
    println!("Result of day07 p1: {}", res);
}

pub fn calc(input: &Vec<String>, bag:&str) -> usize {
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
    let mut res_set:HashSet<&str> = HashSet::new();
    let mut search_set:HashSet<&str> = HashSet::new();
    search_set.insert(bag);
    while !search_set.is_empty() {
        let mut newsearch:HashSet<&str> = HashSet::new();
        for search in &search_set {
            for (bag, content) in rules.iter() {
                if content.contains_key(search) {
                    newsearch.insert(bag);
                    res_set.insert(bag);
                }
            }
        }
        search_set = newsearch;
    }
    //println!("XXX {:#?}", rules);
    res_set.len()
}

mod tests {
    #[test]
    fn test_calc() {
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
        assert_eq!(super::calc(&input, "shiny gold"), 4);
    }
}