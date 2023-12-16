fn to_digit(i: &u8) -> i64 {
    return (i - 48u8) as i64;
}

fn from_words(i: usize, b: &[u8]) -> Option<i64> {
    match &b[i..b.len()] {
        [122, 101, 114, 111, ..] => Some(0),
        [111, 110, 101, ..] => Some(1),
        [116, 119, 111, ..] => Some(2),
        [116, 104, 114, 101, 101, ..] => Some(3),
        [102, 111, 117, 114, ..] => Some(4),
        [102, 105, 118, 101, ..] => Some(5),
        [115, 105, 120, ..] => Some(6),
        [115, 101, 118, 101, 110, ..] => Some(7),
        [101, 105, 103, 104, 116, ..] => Some(8),
        [110, 105, 110, 101, ..] => Some(9),
        _ => None,
    }
}

pub fn p1() -> i64 {
    let bytes = include_bytes!("../inputs/d1.txt");
    let mut sum = 0;
    bytes.split(|b| *b == b'\n').for_each(|l| {
        let l = l
            .iter()
            .filter_map(|b| {
                if *b >= 48u8 && *b <= 57u8 {
                    Some(to_digit(b))
                } else {
                    None
                }
            })
            .collect::<Vec<i64>>();
        let f = l.first().unwrap_or(&0);
        sum += (f * 10) + l.last().unwrap_or(f);
    });
    return sum;
}

pub fn p2() -> i64 {
    let bytes = include_bytes!("../inputs/d1.txt");
    let mut sum = 0;
    bytes.split(|b| *b == b'\n').for_each(|l| {
        let l = l
            .iter()
            .enumerate()
            .filter_map(|(i, b)| {
                if *b >= 48u8 && *b <= 57u8 {
                    Some(to_digit(b))
                } else if *b >= 65 {
                    return from_words(i, l);
                } else {
                    None
                }
            })
            .collect::<Vec<i64>>();
        let f = l.first().unwrap_or(&0);
        sum += (f * 10) + l.last().unwrap_or(f);
    });
    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;
    use common;

    #[test]
    fn test_d1p1() {
        common::timed(p1);
    }

    #[test]
    fn test_d1p2() {
        common::timed(p2);
    }
}
