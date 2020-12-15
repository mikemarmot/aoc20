use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day13.txt").unwrap());
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let res = calc(&input);
    println!("Result of day13 p2: {}", res);
}

fn calc(input: &Vec<String>) -> usize {
    let table = input[1].split(",")
                    .enumerate()
                    .filter(|&(_,y)| y != "x")
                    .map(|(x,y)| (x, y.parse::<usize>().unwrap()))
                    .collect::<Vec<(usize,usize)>>();

    let mut soff = table[0].0;
    let mut base = table[0].1;
    for entry in 1..table.len() {
        let val = table[entry].1;
        let toff = table[entry].0;
        let mut nsoff:Option<usize> = None;

        for i in 1.. {
            let e = soff + i * base + toff;
            if e % val == 0 {
                if nsoff.is_none() {
                    nsoff = Some(e-toff);
                } else if nsoff.is_some() {
                    soff = nsoff.unwrap();
                    base = e - toff - soff;
                    break;
                }
            }
        }
    }
    soff
}

mod tests {
    #[test]
    fn test_calc1() {
        let input:Vec<String> = vec![
            String::from("939"),
            String::from("17,x,13,19"),
        ];
        assert_eq!(super::calc(&input), 3417);
    }
    #[test]
    fn test_calc2() {
        let input:Vec<String> = vec![
            String::from("939"),
            String::from("67,7,59,61"),
        ];
        assert_eq!(super::calc(&input), 754018);
    }
    #[test]
    fn test_calc3() {
        let input:Vec<String> = vec![
            String::from("939"),
            String::from("67,x,7,59,61"),
        ];
        assert_eq!(super::calc(&input), 779210);
    }
    #[test]
    fn test_calc4() {
        let input:Vec<String> = vec![
            String::from("939"),
            String::from("67,7,x,59,61"),
        ];
        assert_eq!(super::calc(&input), 1261476);
    }
    #[test]
    fn test_calc5() {
        let input:Vec<String> = vec![
            String::from("939"),
            String::from("1789,37,47,1889"),
        ];
        assert_eq!(super::calc(&input), 1202161486);
    }
    #[test]
    fn test_calc() {
        let input:Vec<String> = vec![
            String::from("939"),
            String::from("7,13,x,x,59,x,31,19"),
        ];
        assert_eq!(super::calc(&input), 1068781);
    }
}