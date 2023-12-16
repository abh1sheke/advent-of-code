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
    let b: Vec<Vec<&str>> = l
        .split(";")
        .map(|l| l.split(",").map(|i| i.trim()).collect())
        .collect();
    let mut red = 0;
    let mut blue = 0;
    let mut green = 0;
    for s in b {
        for i in s {
            let (l, r) = i.split_once(" ").unwrap();
            let l: i64 = l.parse().unwrap();
            match r {
                "red" => red = red.lt(&l).then(|| l).unwrap_or(red),
                "blue" => blue = blue.lt(&l).then(|| l).unwrap_or(blue),
                "green" => green = green.lt(&l).then(|| l).unwrap_or(green),
                _ => (),
            }
        }
    }
    let p = red * green * blue;
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
