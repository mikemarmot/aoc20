use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day09.txt").unwrap());
    let input: Vec<usize> = reader.lines().map(|l| l.unwrap().parse::<usize>().unwrap()).collect();
    let sres = scalc(&input, 25);
    let res = calc(&input, sres);
    println!("Result of day09 p2: {}", res);
}

fn scalc(data: &Vec<usize>, preamble:usize) -> usize{
    let mut res:Option<usize> = None;
    for (i,d) in data[preamble+1..].iter().enumerate() {
        let mut ok:bool = false;
        for (j,v1) in data[i..=preamble+i-1].iter().enumerate() {
            for v2 in data[i+1+j..=preamble+i].iter() {
                if v1 + v2 == *d {
                    ok = true;
                    break;
                }
                if ok {
                    break;
                }
            } 
        }
        if !ok {
            res = Some(*d);
            break;
        }
    }
    res.unwrap()
}

fn calc(data: &Vec<usize>, sres:usize) -> usize{
    let mut res:Option<usize> = None;
    for i in 0..data.len()-1 {
        for j in 1..data.len()-i {
            let e = &data[i..i+j];
            if e.iter().sum::<usize>() == sres {
                res = Some(e.iter().min().unwrap() + e.iter().max().unwrap());
                break;
            }
        }
        if res.is_some() {
            break;
        }
    }
    res.unwrap()
}

mod tests {
    #[test]
    fn test_calc() {
        let input:Vec<usize> = vec![35,20,15,25,47,40,62,55,65,95,102,117,150,182,127,219,299,277,309,576];
        let sres = super::scalc(&input, 5);
        assert_eq!(super::calc(&input, sres), 62);
    }
}