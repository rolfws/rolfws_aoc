// #![allow(unused)]

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
fn unpack_inp_split(inp: &[u8]) -> (Vec<i8>, Vec<usize>) {
    let mut out: Vec<i8> = Vec::with_capacity(8000);
    let mut out_len: Vec<usize> = Vec::with_capacity(1001);
    out_len.push(0);
    let mut to_push = 0i8;
    let mut cnt = 0;
    for c in inp {
        match c {
            b'\n' => {
                cnt += 1;
                out_len.push(cnt);
                out.push(to_push);
                to_push = 0
            }
            b' ' => {
                out.push(to_push);
                cnt += 1;
                to_push = 0
            }
            a => {
                to_push *= 10;
                to_push += (a - b'0') as i8
            }
        }
    }
    out_len.push(cnt);

    (out, out_len)
}

fn safe_incrb(slc: &[i8]) -> bool {
    for p in slc.windows(2) {
        match p[1] - p[0] {
            1..=3 => {}
            _ => return false,
        }
    }
    true
}

fn safe_decrb(slc: &[i8]) -> bool {
    for p in slc.windows(2) {
        match p[1] - p[0] {
            -3..=-1 => {}
            _ => return false,
        }
    }
    true
}

fn safe_slc(slc: &[i8]) -> bool {
    match slc[1] - slc[0] {
        1..=3 => safe_incrb(&slc[1..]),
        -3..=-1 => safe_decrb(&slc[1..]),
        _ => false,
    }
}

#[aoc(day2, part1)]
fn part1_work(inp: &(Vec<i8>, Vec<usize>)) -> u32 {
    inp.1.windows(2).fold(0u32, |acc, p| {
        if safe_slc(&inp.0[p[0]..p[1]]) {
            acc + 1
        } else {
            acc
        }
    })
}

#[derive(Debug)]
enum Direction {
    Incr,
    Decr,
    Unkn,
}

fn safe_skipped(slc: &[i8], skip: usize) -> bool {
    if skip == 0 {
        safe_slc(&slc[1..])
    } else {
        let mut i = 0;
        let mut dir = Direction::Unkn;
        for j in (1..skip).chain(skip + 1..slc.len()) {
            match (slc[j] - slc[i], &mut dir) {
                (1..=3, Direction::Incr) => {}
                (1..=3, Direction::Unkn) => dir = Direction::Incr,
                (-3..=-1, Direction::Decr) => {}
                (-3..=-1, Direction::Unkn) => dir = Direction::Decr,
                _ => return false,
            }
            i = j
        }
        true
    }
}

fn safe_part2(slc: &[i8]) -> u32 {
    let mut dir = Direction::Unkn;
    for j in 1..slc.len() {
        match (slc[j] - slc[j - 1], &mut dir) {
            (1..=3, Direction::Incr) => {}
            (1..=3, Direction::Unkn) => dir = Direction::Incr,
            (-3..=-1, Direction::Decr) => {}
            (-3..=-1, Direction::Unkn) => dir = Direction::Decr,
            _ => {
                if (j == slc.len() - 1)         // Last is wrong, but that is okay
                    | safe_skipped(slc, j - 1)  // Check if safe if we skip left,
                    | safe_skipped(slc, j)      // or right
                    | (j == 2 && safe_slc(&slc[1..])) // First step was safe, but in wrong direction.
                {
                    return 1;
                } else {
                    return 0;
                }
            }
        }
    }
    1
}

#[aoc(day2, part2)]
fn part2_work(inp: &(Vec<i8>, Vec<usize>)) -> u32 {
    inp.1
        .windows(2)
        .fold(0u32, |acc, p| acc + safe_part2(&inp.0[p[0]..p[1]]))
}

pub fn part1(inp: &[u8]) -> u32 {
    part1_work(&unpack_inp_split(inp))
}

pub fn part2(inp: &[u8]) -> u32 {
    part2_work(&unpack_inp_split(inp))
}
