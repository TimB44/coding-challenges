use std::io::{stdin, BufRead};

const NUM_LEN: usize = 12;

fn jotage(bytes: &[u8]) -> u64 {
    let mut result = 0;
    let mut min_i = 0;
    for i in 0..NUM_LEN {
        result *= 10;
        let max_i = bytes.len() - (NUM_LEN - i - 1);
        let len_i = max_i - min_i;
        let (digit_i, digit) = bytes
            .iter()
            .enumerate()
            .skip(min_i)
            .take(len_i)
            .max_by_key(|(i, n)| (**n, -(*i as i64)))
            .unwrap();
        result += (digit - b'0') as u64;
        min_i = digit_i + 1
    }
    result
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
        assert_eq!(jotage(b"987654321111111"), 987654321111);
        assert_eq!(jotage(b"811111111111119"), 811111111119);
        assert_eq!(jotage(b"234234234234278"), 434234234278);
        assert_eq!(jotage(b"818181911112111"), 888911112111);
    }
}
