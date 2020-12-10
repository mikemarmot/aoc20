use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day10.txt").unwrap());
    let input: Vec<usize> = reader.lines().map(|l| l.unwrap().parse::<usize>().unwrap()).collect();
    let res = calc(&input);
    println!("Result of day10 p1: {}", res);
}

fn calc(data: &Vec<usize>) -> usize {
    let mut res:Vec<usize> = Vec::new();
    let mut d = data.clone();
    d.push(0); // the source
    let dev = d.iter().max().unwrap() + 3;
    d.push(dev); // the device
    d.sort();
    for i in 0..d.len()-1 {
        res.push(d[i+1] - d[i]);
    }
    res.iter().filter(|&x| *x == 1).count() * res.iter().filter(|&x| *x == 3).count()
}

mod tests {
    #[test]
    fn test_calc1() {
        let input:Vec<usize> = vec![16,10,15,5,1,11,7,19,6,12,4];
        assert_eq!(super::calc(&input), 35);
    }

    #[test]
    fn test_calc2() {
        let input:Vec<usize> = vec![28,33,18,42,31,14,46,20,48,47,24,23,49,45,19,38,39,11,1,32,25,35,8,17,7,9,4,2,34,10,3];
        assert_eq!(super::calc(&input), 220);
    }    
}