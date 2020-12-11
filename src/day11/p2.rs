use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn doit() {
    let reader = BufReader::new(File::open("data/input_day11.txt").unwrap());
    let input: Vec<Vec<char>> = reader.lines().map(|l| l.unwrap().chars().collect()).collect();
    let res = calc(&input);
    println!("Result of day11 p1: {}", res);
}

fn calc(data: &Vec<Vec<char>>) -> usize {
    let mut occ:usize;
    let mut locc:usize = data.iter().map(|x| x.iter().map(|_| 1).sum::<usize>()).sum::<usize>() + 1;
    let mut old = data.clone();
    let mut new = data.clone();
    //println!("{:?}", old);
    loop {
        for (y, v) in data.iter().enumerate() {
            for (x, _e) in v.iter().enumerate() {
                new[y][x] = val(&old, x, y);
            }
        }
        //println!("{:?}", new);
        old = new.clone();
        occ = new.iter().map(|x| x.iter().map(|x| if *x == '#' { 1 } else { 0 }).sum::<usize>()).sum::<usize>();
        if occ == locc {
            break;
        } else {
            locc = occ;
        }
    }
    occ
}

fn val(input: &Vec<Vec<char>>, xpos:usize, ypos:usize) -> char {
    let mut a:[char; 8] = [' '; 8];
    let state = input[ypos][xpos];
    let xpos = xpos as i16;
    let ypos = ypos as i16;
    //println!("SSS {} {} {}", xpos, ypos, input[ypos as usize][xpos as usize]);
    a[0] = f(input, xpos, ypos, -1, -1); // lo
    a[1] = f(input, xpos, ypos, 0, -1); // o
    a[2] = f(input, xpos, ypos, 1, -1); // ro
    a[3] = f(input, xpos, ypos, 1, 0); // r
    a[4] = f(input, xpos, ypos, 1, 1); // ru
    a[5] = f(input, xpos, ypos, 0, 1); // u
    a[6] = f(input, xpos, ypos, -1, 1); // lu
    a[7] = f(input, xpos, ypos, -1, 0); // l

    let occ:u8 = a.iter().map(|b| if *b == '#' { 1 } else { 0 }).sum();
    //let emp:u8 = a.iter().map(|b| if *b == 'L' { 1 } else { 0 }).sum();
    //let all:u8 = a.iter().map(|b| if *b != '.' { 1 } else { 0 }).sum();
    // if xpos == 2 && ypos == 5 {
    //     println!("YYY {:?} {}", a, c);
    // }
    match state {
        'L' if occ == 0 => '#',
        'L' if occ > 0 => 'L',
        '#' if occ >= 5 => 'L',
        _ => state 
    }
}

fn f(input: &Vec<Vec<char>>, xpos:i16, ypos:i16, xdif:i16, ydif:i16) -> char {
    let (minx, maxx) = (0 as i16, input[0].len() as i16 - 1);
    let (miny, maxy) = (0 as i16, input.len() as i16 - 1);
    let mut res:char = '.';
    let mut xpos = xpos;
    let mut ypos = ypos;
    loop {
        xpos += xdif;
        ypos += ydif;
        if xpos > maxx || ypos > maxy ||
            xpos < minx || ypos < miny {
                break;
        } else if input[ypos as usize][xpos as usize] == '.' {
            continue;
        } else {
            res = input[ypos as usize][xpos as usize];
            break;
        }
    }
    res
}

mod tests {
    #[test]
    fn test_calc() {
        let input:Vec<&str> = vec![
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ];
        let data:Vec<Vec<char>> = input.iter().map(|x| x.chars().collect()).collect();
        assert_eq!(super::calc(&data), 26);
    }   
}