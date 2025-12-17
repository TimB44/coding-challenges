use std::io::stdin;

static NUMBERS_KEYS: [(u32, &[u8]); 18] = [
    (1, b"one"),
    (2, b"two"),
    (3, b"three"),
    (4, b"four"),
    (5, b"five"),
    (6, b"six"),
    (7, b"seven"),
    (8, b"eight"),
    (9, b"nine"),
    (1, b"1"),
    (2, b"2"),
    (3, b"3"),
    (4, b"4"),
    (5, b"5"),
    (6, b"6"),
    (7, b"7"),
    (8, b"8"),
    (9, b"9"),
];

fn main() {
    let mut sum = 0;
    for line in stdin().lines().map(Result::unwrap) {
        let mut first = 0;
        let mut second = 0;

        for i in 0..line.len() {
            if let Some(&(n, _)) = NUMBERS_KEYS
                .iter()
                .filter(|(_, b)| line.as_bytes()[i..].starts_with(b))
                .next()
            {
                first = n;
                break;
            }
        }

        for i in (0..line.len()).into_iter().rev() {
            if let Some(&(n, _)) = NUMBERS_KEYS
                .iter()
                .filter(|(_, b)| line.as_bytes()[i..].starts_with(b))
                .next()
            {
                second = n;
                break;
            }
        }

        assert!(first != 0 && second != 0, "No number found in the line");
        sum += first * 10 + second;
    }

    println!("Result: {sum}");
}
