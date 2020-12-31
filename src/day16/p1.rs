use std::ops::Range;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day16.txt").unwrap());
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let res = calc(&input);
    println!("Result of day16 p1: {}", res);
}

fn calc(input:&Vec<String>) -> usize {
    let mut rules:HashMap<&str, Vec<Range<usize>>> = HashMap::new();
    
    
    let smt = input.iter().position(|x| x == "your ticket:").unwrap();
    // collect all the rules
    for r in &input[..smt-1] {
        let mut v:Vec<Range<usize>> = Vec::new();
        let a = r.splitn(2, ": ").collect::<Vec<&str>>();
        let b = a[1].split(" or ").collect::<Vec<&str>>();
        for ra in b {
            let mut c = ra.split("-").map(|x| x.parse::<usize>().unwrap());
            let mr:Range<usize> = c.next().unwrap()..c.next().unwrap()+1;
            v.push(mr);
        }
        rules.insert(a[0], v);        
    }
    // collect my ticket
    let _myticket:Vec<usize> = input[smt+1].split(',').map(|x| x.parse::<usize>().unwrap()).collect();
    let snt = input.iter().position(|x| x == "nearby tickets:").unwrap();
    // collect nearby tickets
    let mut nbtickets:Vec<Vec<usize>> = Vec::new();
    for nt in &input[snt+1..] {
        let v:Vec<usize> = nt.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
        nbtickets.push(v);
    }
    let mut res:usize = 0;
    for nbt in nbtickets {
        for val in nbt {
            let mut hit = false;
            for rule in rules.values() {
                for r in rule {
                    if r.contains(&val) {
                        hit = true;
                        break;
                    }
                }
                if hit { break; }
            }
            if !hit {
                res += val;
            }
        } 
    }
    res
}

mod tests {
    #[test]
    fn test_calc() {
        let input:Vec<String> = vec![
            String::from("class: 1-3 or 5-7"),
            String::from("row: 6-11 or 33-44"),
            String::from("seat: 13-40 or 45-50"),
            String::from(""),
            String::from("your ticket:"),
            String::from("7,1,14"),
            String::from(""),
            String::from("nearby tickets:"),
            String::from("7,3,47"),
            String::from("40,4,50"),
            String::from("55,2,20"),
            String::from("38,6,12"),
        ];
        assert_eq!(super::calc(&input), 71);
    }
}