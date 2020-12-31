use std::ops::Range;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day16.txt").unwrap());
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let res = calc(&input);
    println!("Result of day16 p2: {}", res);
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
    let myticket:Vec<usize> = input[smt+1].split(',').map(|x| x.parse::<usize>().unwrap()).collect();
    let snt = input.iter().position(|x| x == "nearby tickets:").unwrap();
    // collect nearby tickets
    let mut nbtickets:Vec<Vec<usize>> = Vec::new();
    for nt in &input[snt+1..] {
        let v:Vec<usize> = nt.split(',').map(|x| x.parse::<usize>().unwrap()).collect();
        nbtickets.push(v);
    }
    let mut vtickets:Vec<Vec<usize>> = Vec::new(); 
    let mut res:usize = 0;
    for nbt in nbtickets {
        let mut valid = true;
        for val in &nbt {
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
                valid = false;
                res += val;
            }
        }
        if valid { vtickets.push(nbt); }
    }
    vtickets.push(myticket.clone());
    let mut ergs:HashMap<usize, Vec<&str>> = HashMap::new();
    for i in 0..rules.len() {
        let mut z:Vec<&str> = Vec::new();
        let f:Vec<usize> = vtickets.iter().map(|x| x[i]).collect();
        for (nr, rule) in rules.iter() {
            let mut m = true;
            for v in &f {
                if !rule[0].contains(v) && !rule[1].contains(v) { 
                    m = false;
                    break; 
                }
            }
            if m {
                z.push(nr); 
            };
        }
        ergs.insert(i, z);
    }

    let mut res:HashMap<usize, &str> = HashMap::new();
    loop {
        let mut tz:Option<(&usize, &Vec<&str>)> = None;
        for e in &ergs {
            if e.1.len() == 1 {
                tz = Some(e);
                break;
            }
        }
        assert!(tz.is_some());
        let (l,m) = (*tz.unwrap().0, tz.unwrap().1[0]);
        res.insert(l, m);
        ergs.remove(&l);
        if ergs.is_empty() { break; }
        let keys:Vec<usize> = ergs.keys().map(|x| *x).collect();
        for k in keys {
            match ergs.get(&k).unwrap().iter().position(|&x| x == m) {
                Some(x) => { ergs.get_mut(&k).unwrap().remove(x); },
                None => {}
            };
        } 
    }
    let mut ires = 1;
    for (rk, rv) in res {
        if rv.starts_with("departure") { ires *= &myticket[rk]; } 
    }
    ires
}

mod tests {
    #[test]
    fn test_calc() {
        let input:Vec<String> = vec![
            String::from("class: 0-1 or 4-19"),
            String::from("row: 0-5 or 8-19"),
            String::from("seat: 0-13 or 16-19"),
            String::from(""),
            String::from("your ticket:"),
            String::from("11,12,13"),
            String::from(""),
            String::from("nearby tickets:"),
            String::from("3,9,18"),
            String::from("15,1,5"),
            String::from("5,14,9"),
        ];
        assert_eq!(super::calc(&input), 1);
    }
}