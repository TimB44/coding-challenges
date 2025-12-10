use std::io::{self, BufRead};

const MODULO: i32 = 100;

fn main() -> io::Result<()> {
    let stdin = io::stdin().lock();
    let mut cur_pos = 50;
    let mut zero_count = 0;

    for line_res in stdin.lines() {
        let line = line_res?;
        let raw_line = line.as_bytes();
        let sign = if raw_line[0] == b'R' { 1 } else { -1 };
        let n: i32 = line[1..].parse().unwrap();
        cur_pos += n * sign;
        cur_pos %= MODULO;
        if cur_pos == 0 {
            zero_count += 1;
        }
    }
    println!("Zero count = {zero_count}");

    Ok(())
}
