use itertools::Itertools;

fn get_symbols() -> Vec<u8> {
    include_bytes!("../../../inputs/2023/d3.txt")
        .split(|b| *b == b'\n')
        .map(|l| {
            return l
                .iter()
                .filter_map(|ch| {
                    if !ch.is_ascii_digit() && ch.ne(&&b'.') {
                        return Some(*ch);
                    }
                    return None;
                })
                .collect::<Vec<_>>();
        })
        .concat()
        .into_iter()
        .unique()
        .collect()
}

fn find_pos(r: usize, c: usize, l: usize) -> Vec<(usize, usize)> {
    let mut pos: Vec<(usize, usize)> = Vec::new();
    let s = c - l;
    (s..c).for_each(|i| {
        if i == s {
            pos.append(&mut vec![(r, c), (r + 1, c)]);
            if r > 0 {
                pos.push((r - 1, c));
                if s > 0 {
                    pos.push((r - 1, s - 1));
                }
            }
            if s > 0 {
                pos.append(&mut vec![(r, s - 1), (r + 1, s - 1)]);
            }
        }
        pos.push((r + 1, i));
        if r > 0 {
            pos.push((r - 1, i));
        }
    });
    return pos;
}

fn is_partno(s: &[&[u8]], pos: &[(usize, usize)], symbols: &[u8]) -> bool {
    return pos
        .iter()
        .filter(|i| {
            if let Some(l) = s.get(i.0) {
                if let Some(m) = l.get(i.1) {
                    return symbols.contains(&m);
                }
            }
            return false;
        })
        .count()
        > 0;
}

#[derive(Debug, PartialEq)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

fn check_y(s: &[&[u8]], r: usize, c: usize, y: Direction) -> Option<Vec<i64>> {
    if r < 1 && y.eq(&Direction::Top) {
        return None;
    } else if r == 139 && y.eq(&Direction::Bottom) {
        return None;
    }
    let range = match c {
        0 => 0..=4,
        1 => c - 1..=c + 4,
        2 => c - 2..=c + 4,
        _ => c - 3..=c + 4,
    };
    let mut nr: Vec<(i64, usize, usize)> = Vec::new();
    let mut num = 0i64;
    let mut st = 0usize;
    for i in range {
        if i > 139 {
            continue;
        }
        let elem = match y {
            Direction::Top => s[r - 1][i],
            Direction::Bottom => s[r + 1][i],
            _ => 0,
        };
        if !elem.is_ascii_digit() {
            if num > 0 {
                nr.push((num, st, i - 1));
            }
            st = 0;
            num = 0;
            continue;
        }
        if st == 0 {
            st = i;
        }
        let d = (elem - 48u8) as i64;
        num = num * 10 + d;
    }
    let nr: Vec<i64> = nr
        .iter()
        .filter(|(_, s, e)| {
            *s == c + 1 || *e == c - 1 || *s == c || *e == c || (*s == c - 1 && *e == c + 1)
        })
        .map(|(i, _, _)| *i)
        .collect();
    if nr.len() < 1 {
        return None;
    }
    return Some(nr);
}

fn check_x(s: &[&[u8]], r: usize, c: usize, y: Direction) -> Option<i64> {
    if y.eq(&Direction::Left) {
        if c < 1 || !s[r][c - 1].is_ascii_digit() {
            return None;
        }
    } else if y.eq(&Direction::Right) {
        if c == 139 || !s[r][c + 1].is_ascii_digit() {
            return None;
        }
    }
    let range = if y.eq(&Direction::Left) {
        match c {
            1 => c - 1..c,
            2 => c - 2..c,
            _ => c - 3..c,
        }
    } else {
        match c {
            138 => c + 1..c + 2,
            137 => c + 1..c + 3,
            _ => c + 1..c + 4,
        }
    };
    let mut num = 0i64;
    for i in range {
        let elem = s[r][i];
        if !elem.is_ascii_digit() {
            num = 0;
            continue;
        }
        let d = (elem - 48u8) as i64;
        num = num * 10 + d;
    }
    if num > 0 {
        return Some(num);
    }
    return None;
}

fn get_ratio(s: &[&[u8]], r: usize, c: usize) -> i64 {
    let mut top = check_y(s, r, c, Direction::Top).unwrap_or(vec![]);
    let mut bottom = check_y(s, r, c, Direction::Bottom).unwrap_or(vec![]);
    let left = check_x(s, r, c, Direction::Left);
    let right = check_x(s, r, c, Direction::Right);
    let mut x: Vec<i64> = Vec::new();
    x.append(&mut top);
    x.append(&mut bottom);
    if let Some(left) = left {
        x.push(left);
    }
    if let Some(right) = right {
        x.push(right);
    }
    if x.len() == 2 {
        return x.iter().product();
    }
    return 0;
}

pub fn p1() -> i64 {
    let symbols = get_symbols();
    let bytes = include_bytes!("../../../inputs/2023/d3.txt")
        .split(|b| *b == b'\n')
        .collect::<Vec<&[u8]>>();
    let mut sum = 0;
    for (i, l) in bytes.iter().enumerate() {
        let mut j: usize = 0;
        loop {
            if j >= l.len() {
                break;
            }
            let mut num: i64 = 0;
            let mut numl: usize = 0;
            while j < l.len() && l[j].is_ascii_digit() {
                let d: i64 = (l[j] - 48u8) as i64;
                num = num * 10 + d;
                j += 1;
                numl += 1;
            }
            if num > 0 {
                let pos = find_pos(i, j, numl);
                if is_partno(&bytes, &pos, &symbols) {
                    sum += num;
                };
            } else {
                j += 1;
            }
        }
    }
    return sum;
}

pub fn p2() -> i64 {
    let bytes = include_bytes!("../../../inputs/2023/d3.txt")
        .split(|b| *b == b'\n')
        .collect::<Vec<&[u8]>>();
    bytes
        .iter()
        .enumerate()
        .map(|(i, l)| {
            l.iter()
                .enumerate()
                .map(|(j, m)| {
                    if *m == b'*' {
                        return get_ratio(&bytes, i, j);
                    }
                    return 0;
                })
                .sum::<i64>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use common;

    #[test]
    fn d3p1() {
        common::timed(p1);
    }

    #[test]
    fn d3p2() {
        common::timed(p2);
    }
}
