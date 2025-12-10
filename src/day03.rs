use std::io::{stdin, BufRead};

fn jotage(bytes: &[u8]) -> u64 {
    let line_len = bytes.len();
    let (i, n1) = bytes[0..(line_len - 1)]
        .iter()
        .enumerate()
        .max_by_key(|(i, n)| (**n, -(*i as i64)))
        .unwrap();

    let n2 = bytes[(i + 1)..].iter().max().unwrap();

    ((n1 - b'0') as u64) * 10 + ((n2 - b'0') as u64)
}

fn main() {
    let sum: u64 = stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|l| jotage(l.as_bytes()))
        .sum();

    println!("Joltage = {sum}");
}

#[cfg(test)]
mod test {
    use crate::jotage;

    #[test]
    fn jotage_test() {
        assert_eq!(jotage(b"987654321111111"), 98);
        assert_eq!(jotage(b"811111111111119"), 89);
        assert_eq!(jotage(b"234234234234278"), 78);
        assert_eq!(jotage(b"818181911112111"), 92);

        assert_eq!(jotage(b"12"), 12);
        assert_eq!(jotage(b"123"), 23);
        assert_eq!(jotage(b"31"), 31);
    }
}
