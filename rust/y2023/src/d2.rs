use std::collections::HashMap;
use std::fs;

fn parse_balls(l: &str) -> bool {
    let c: HashMap<&str, i64> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let b: Vec<Vec<&str>> = l
        .split(";")
        .map(|l| l.split(",").map(|i| i.trim()).collect())
        .collect();
    for s in b {
        for i in s {
            let (l, r) = i.split_once(" ").unwrap();
            let l: i64 = l.parse().unwrap();
            if &l > c.get(r).unwrap() {
                return false;
            }
        }
    }
    return true;
}

fn parse_min_balls(l: &str) -> i64 {
    let mut c: HashMap<&str, Vec<i64>> =
        HashMap::from([("red", vec![]), ("green", vec![]), ("blue", vec![])]);
    let b: Vec<Vec<&str>> = l
        .split(";")
        .map(|l| l.split(",").map(|i| i.trim()).collect())
        .collect();
    for s in b {
        for i in s {
            let (l, r) = i.split_once(" ").unwrap();
            let l: i64 = l.parse().unwrap();
            c.get_mut(r).unwrap().push(l);
        }
    }
    let p = c.get("red").unwrap().iter().max().unwrap()
        * c.get("blue").unwrap().iter().max().unwrap()
        * c.get("green").unwrap().iter().max().unwrap();
    return p;
}

pub fn p1() -> i64 {
    let input = fs::read_to_string("../../inputs/2023/d2.txt").unwrap();
    let s: i64 = input
        .split("\n")
        .map(|l| {
            if let Some((l, r)) = l.split_once(":") {
                let i: i64 = l.split_at(4).1.trim().parse().unwrap();
                if parse_balls(r) {
                    return i;
                }
            }
            return 0;
        })
        .sum();
    return s;
}

pub fn p2() -> i64 {
    let input = fs::read_to_string("../../inputs/2023/d2.txt").unwrap();
    let s: i64 = input
        .split("\n")
        .map(|l| {
            if let Some((_, r)) = l.split_once(":") {
                return parse_min_balls(r);
            }
            return 0;
        })
        .sum();
    return s;
}

#[cfg(test)]
mod tests {
    use super::*;
    use common;

    #[test]
    fn d2_p1() {
        common::timed(p1);
    }

    #[test]
    fn d2_p2() {
        common::timed(p2);
    }
}
