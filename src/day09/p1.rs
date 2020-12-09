use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Loc {
    inst: String,
    val: i32,
}

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day08.txt").unwrap());
    let input: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let data = prepare_data(&input);
    let res = calc(&data);
    println!("Result of day09 p1: {}", res);
}

fn prepare_data(data:&Vec<String>) -> Vec<Loc> {
    data.iter().map(|l| { 
        let ll:Vec<String> = l.split(" ").map(|x| String::from(x)).collect();
        Loc { inst: String::from(&ll[0]), val: ll[1].parse::<i32>().unwrap() }
    }).collect()
}

fn calc(data: &Vec<Loc>) -> i32 {
    let mut vis:HashMap<usize, usize> = HashMap::new();
    let mut acc:i32 = 0;
    let mut p:usize = 0;
    let limit:usize = 1; 
    loop {
        match vis.get_mut(&p) {
            Some(x) => { 
                if *x >= limit {
                    break;
                }
                else {
                    *x += 1;
                }
            },
            None => { vis.insert(p, 1); },
        }
        match &data[p] {
            Loc { inst, val } if inst == "acc" => { 
                acc += val;
                p += 1;
            },
            Loc { inst, val } if inst == "jmp" => {
                if p as i32 + *val < 0 {
                    println!("Unrechable address");
                } else {
                    p = (p as i32 + *val) as usize;
                }
            },
            Loc { inst, val:_ } if inst == "nop" => {
                p += 1;
            },
            &_ => { println!("Unknown instruction")}
        }
    }
    acc
}

mod tests {
    #[test]
    fn test_calc() {
        let input:Vec<String> = vec![
            String::from("nop +0"),
            String::from("acc +1"),
            String::from("jmp +4"),
            String::from("acc +3"),
            String::from("jmp -3"),
            String::from("acc -99"),
            String::from("acc +1"),
            String::from("jmp -4"),
            String::from("acc +6"),
        ];
        let data = super::prepare_data(&input);
        assert_eq!(super::calc(&data), 5);
    }
}