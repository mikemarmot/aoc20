use std::collections::HashMap;

pub fn doit() {
    let res = calc("2,20,0,4,1,17", 2020);
    println!("Result of day15 p1: {}", res);
}

fn calc(input:&str, c:usize) -> usize {
    let mut mem:HashMap<usize, Vec<usize>> = HashMap::new();
    let input:Vec<usize> = input.split(",").map(|x| x.parse::<usize>().unwrap()).collect();
    let mut ls:usize = 0;
    for i in 0..c-1 {
        if i < input.len() {
            ls = input[i];
        }
        if !mem.contains_key(&ls) { mem.insert(ls, Vec::new()); }
        let rs = mem.get_mut(&ls).unwrap();
        rs.push(i);
        if rs.len() == 1 { // spoken first time
            ls = 0;
        } else {
            let o:Vec<&usize> = rs.iter().rev().take(2).collect();
            ls = o[0] - o[1];
        }
    }
    ls
}

mod tests {
    #[test]
    fn test_calc() {
        assert_eq!(super::calc("0,3,6", 10), 0);
        assert_eq!(super::calc("0,3,6", 2020), 436);
        assert_eq!(super::calc("1,3,2", 2020), 1);
        assert_eq!(super::calc("2,1,3", 2020), 10);
        assert_eq!(super::calc("1,2,3", 2020), 27);
        assert_eq!(super::calc("2,3,1", 2020), 78);
        assert_eq!(super::calc("3,2,1", 2020), 438);
        assert_eq!(super::calc("3,1,2", 2020), 1836);
    }
}