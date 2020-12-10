use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day10.txt").unwrap());
    let input: Vec<usize> = reader.lines().map(|l| l.unwrap().parse::<usize>().unwrap()).collect();
    let res = calc(&input);
    println!("Result of day10 p2: {}", res);
}

fn calc(data: &Vec<usize>) -> u64 {
    let mut res:Vec<usize> = Vec::new();
    let mut d = data.clone();
    d.push(0); // the source
    let dev = d.iter().max().unwrap() + 3;
    d.push(dev); // the device
    d.sort();
    for i in 0..d.len()-1 {
        res.push(d[i+1] - d[i]);
    }
    let mut cnt = 0;
    let mut start:u64 = 1;
    for i in res {
        match i {
            1 => { cnt+=1; },
            3 => {
                if cnt > 0 {
                    let x = prob(cnt) as u64;
                    start *= x;
                    cnt = 0;
                }
            }
            _ => {}
        };
    }
    start
}

fn prob(l:usize) -> usize {
    match l {
        1 => return 1,
        2 => return 2,
        3 => return 4,
        4 => return 7,
        _=> return 0
    }
}

mod tests {
    #[test]
    fn test_calc1() {
        let input:Vec<usize> = vec![16,10,15,5,1,11,7,19,6,12,4];
        assert_eq!(super::calc(&input), 8);
    }

    #[test]
    fn test_calc2() {
        let input:Vec<usize> = vec![28,33,18,42,31,14,46,20,48,47,24,23,49,45,19,38,39,11,1,32,25,35,8,17,7,9,4,2,34,10,3];
        assert_eq!(super::calc(&input), 19208);
    }    
}