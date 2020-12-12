use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day12.txt").unwrap());
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let res = calc(&input);
    println!("Result of day12 p2: {}", res);
}

fn rot(w:&(i32,i32), a:i32) -> (i32,i32) {
    let a = match a {
        x if x > 180 => x - 360,
        x if x <= -180 => x + 360,
        y => y 
    };
    match a {
        90 => (0-w.1,w.0),
        -90 => (w.1, 0-w.0),
        180 => (0-w.0, 0-w.1),
        _ => { println!("Error2"); (0,0) }
    }
}

fn calc(input: &Vec<String>) -> i32 {
    let mut wpos:(i32, i32) = (10, -1);
    let mut pos:(i32,i32) = (0,0);
    for line in input.iter().filter(|x| x.len() > 0) {
        let val = &line[1..].parse::<i32>().unwrap();
        match &line[0..1] {
            "N" => wpos.1 -= val,
            "S" => wpos.1 += val,
            "E" => wpos.0 += val,
            "W" => wpos.0 -= val,
            "R" => wpos = rot(&wpos, *val),
            "L" => wpos = rot(&wpos, 0-val),
            "F" => pos = (pos.0 + val * wpos.0, pos.1 + val * wpos.1),
            _ => println!("Error")
        };
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
        assert_eq!(super::calc(&input), 286);
    }   
}