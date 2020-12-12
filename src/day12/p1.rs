use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day12.txt").unwrap());
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let res = calc(&input);
    println!("Result of day12 p1: {}", res);
}

fn cdir<'a>(o:&'a str, a:&str, v:&usize) -> &'a str {
    let dirs = vec!["N", "E", "S", "W"];
    let v = match a {
        "L" => 360 - v,
        _ => *v
    };
    dirs[(dirs.iter().position(|&x| x == o).unwrap() + (v % 360) / 90) % 4]
}

fn calc(input: &Vec<String>) -> i32 {
    let mut data:Vec<(&str, usize)> = Vec::new();
    let mut dir = "E";
    for line in input.iter().filter(|x| x.len() > 0) {
        let val = &line[1..].parse::<usize>().unwrap();
        match &line[0..1] {
            x if x == "L" || x == "R" => { dir = cdir(dir, x, val); }, 
            "F" => data.push((dir, *val)),
            y => data.push((y, *val))
        };
    }
    let mut pos:(i32,i32) = (0,0);
    for d in data.iter().map(|(d,v)| (d, *v as i32)) {
        match d {
            (&"N", val) => pos.1 -= val,
            (&"S", val) => pos.1 += val,
            (&"E", val) => pos.0 += val,
            (&"W", val) => pos.0 -= val,
            x => println!("Error {:?}", x)
        }
    }
    pos.0.abs() + pos.1.abs()
}

mod tests {
    #[test]
    fn test_calc() {
        let input:Vec<String> = vec![
            String::from("F10"),
            String::from("N3"),
            String::from("F7"),
            String::from("R90"),
            String::from("F11"),
        ];
        assert_eq!(super::calc(&input), 25);
    }   
}