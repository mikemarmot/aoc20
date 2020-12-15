use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, HashSet};

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day14.txt").unwrap());
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let res = calc(&input);
    println!("Result of day14 p2: {}", res);
}

fn calc(input: &Vec<String>) -> usize {
    let mut mask:HashMap<usize, char> = HashMap::new();
    let mut mem:HashMap<usize, usize> = HashMap::new();
    for line in input {
        if line.starts_with("mask") {
            mask = line
                    .chars()
                    .rev()
                    .take(36)
                    .enumerate()
                    .filter(|&(_,v)| v != '0')
                    .collect::<HashMap<usize, char>>();
        } else if line.starts_with("mem") {
            let (p1,p2) = (line.find(|c| c == '[').unwrap(), line.find(|c| c == ']').unwrap());
            let mut addresses:HashSet<usize> = HashSet::new();
            addresses.insert(line[p1+1..p2].parse::<usize>().unwrap());
            let val = line.split(" = ").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();
            for (p, v) in mask.iter() {
                let mut nads:HashSet<usize> = HashSet::new();
                for a in addresses {
                    let m = 1 << p;
                    if *v == '1' {
                        let n = a | m;
                        nads.insert(n);
                    } else {
                        let n = a ^ m;
                        nads.insert(a);
                        nads.insert(n);
                    }
                }
                addresses = nads;
            }
            for address in addresses {
                &mem.insert(address, val);
            }
        }
    }
    mem.values().sum()
}

mod tests {
    #[test]
    fn test_calc() {
        let input:Vec<String> = vec![
            String::from("mask = 000000000000000000000000000000X1001X"),
            String::from("mem[42] = 100"),
            String::from("mask = 00000000000000000000000000000000X0XX"),
            String::from("mem[26] = 1"),
        ];
        assert_eq!(super::calc(&input), 208);
    }
}