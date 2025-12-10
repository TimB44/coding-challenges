use std::io::{self, BufRead};

const MODULO: i32 = 100;

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn to_sign(self) -> i32 {
        match self {
            Direction::Left => -1,
            Direction::Right => 1,
        }
    }
}

fn distance_to_zero(pos: i32, dir: Direction) -> i32 {
    assert!(pos < MODULO);
    match dir {
        Direction::Left if pos == 0 => MODULO,
        Direction::Left => pos,
        Direction::Right => MODULO - pos,
    }
}

fn count_zero_hits(pos: i32, dir: Direction, amount: i32) -> i32 {
    let dist = distance_to_zero(pos, dir);
    if dist > amount {
        0
    } else {
        1 + (amount - dist) / MODULO
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin().lock();
    let mut cur_pos = 50;
    let mut zero_count = 0;

    for line_res in stdin.lines() {
        let line = line_res?;
        let dir = if &line[..1] == "R" {
            Direction::Right
        } else {
            Direction::Left
        };
        let n: i32 = line[1..].parse().unwrap();
        zero_count += count_zero_hits(cur_pos, dir, n);
        cur_pos += n * dir.to_sign();
        cur_pos = cur_pos.rem_euclid(MODULO);
    }
    println!("Zero count = {zero_count}");

    Ok(())
}
