#![allow(unused)]

use std::ops::Neg;

use aoc_runner_derive::aoc;
use memchr::{memchr, memchr3_iter, memchr_iter};

#[cfg(test)]
const W: i32 = 11;
#[cfg(test)]
const H: i32 = 7;

#[cfg(test)]
const WU: usize = 11;
#[cfg(test)]
const HU: usize = 7;

#[cfg(test)]
const HW: i32 = 5;
#[cfg(test)]
const HH: i32 = 3;

#[cfg(not(test))]
const W: i32 = 101;
#[cfg(not(test))]
const H: i32 = 103;

#[cfg(not(test))]
const WU: usize = 101;
#[cfg(not(test))]
const HU: usize = 103;
#[cfg(not(test))]
const HW: i32 = 50;
#[cfg(not(test))]
const HH: i32 = 51;

fn fast_parse(slc: &[u8]) -> i32 {
    let mut out = 0i32;
    let mut neg = slc[0] == b'-';
    if neg {
        for b in &slc[1..] {
            out *= 10;
            out += (b - b'0') as i32
        }
        out.neg()
    } else {
        for b in slc {
            out *= 10;
            out += (b - b'0') as i32
        }
        out
    }
}

unsafe fn part1_inner(inp: &[u8]) -> u32 {
    let mut pos_iter = memchr3_iter(b',', b'v', b'\n', inp);
    let mut start = 0usize;
    let mut cnts = [0u32; 4];
    while let Some(c1_pos) = pos_iter.next() {
        let v_pos = pos_iter.next().expect("If there is a comma there is a v");
        let c2_pos = pos_iter.next().expect("Commas come in pairs");
        let end_pos = pos_iter.next().unwrap_or(inp.len()); // Last \n might be ommitted

        let x_start = fast_parse(&inp[start + 2..c1_pos]);
        let y_start = fast_parse(&inp[c1_pos + 1..v_pos - 1]);
        let vx = fast_parse(&inp[v_pos + 2..c2_pos]);
        let vy = fast_parse(&inp[c2_pos + 1..end_pos]);
        start = end_pos + 1;

        let x_end = (x_start + 100 * vx).rem_euclid(W);
        let y_end = (y_start + 100 * vy).rem_euclid(H);
        match (x_end, y_end) {
            (HW, _) | (_, HH) => {}
            (0..HW, 0..HH) => cnts[0] += 1,
            (HW..W, 0..HH) => cnts[1] += 1,
            (0..HW, HH..H) => cnts[2] += 1,
            (HW..W, HH..H) => cnts[3] += 1,
            _ => {}
        }
    }
    cnts.into_iter().product()
}

// function extended_gcd(a, b)
//     (old_r, r) := (a, b)
//     (old_s, s) := (1, 0)
//     (old_t, t) := (0, 1)
    
//     while r ≠ 0 do
//         quotient := old_r div r
//         (old_r, r) := (r, old_r − quotient × r)
//         (old_s, s) := (s, old_s − quotient × s)
//         (old_t, t) := (t, old_t − quotient × t)
    
//     output "Bézout coefficients:", (old_s, old_t)
//     output "greatest common divisor:", old_r
//     output "quotients by the gcd:", (t, s)
fn extended_gcd(a:i32,b:i32) -> (i32, i32, i32) {
    let (mut old_r, mut r) = (a, b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);
    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_s, s) = (s, old_s - q * s);
        (old_t, t) = (t, old_t - q * t);
    };
    (old_s, old_t, old_r)

}
unsafe fn part2_inner(inp: &[u8]) -> u32 {
    let mut pos_iter = memchr3_iter(b',', b'v', b'\n', inp);
    let mut pos = [(0i32, 0i32); 500];
    let mut vc = [(0i32, 0i32); 500];
    let mut start = 0usize;
    let mut i = 0;
    let mut cur_occ = [false; WU * HU];
    while let Some(c1_pos) = pos_iter.next() {
        let v_pos = pos_iter.next().expect("If there is a comma there is a v");
        let c2_pos = pos_iter.next().expect("Commas come in pairs");
        let end_pos = pos_iter.next().unwrap_or(inp.len()); // Last \n might be ommitted

        let init_pos = (fast_parse(&inp[start + 2..c1_pos]),
        fast_parse(&inp[c1_pos + 1..v_pos - 1]));
        *cur_occ.get_unchecked_mut(init_pos.0 as usize * WU + init_pos.1 as usize) = true;
        *pos.get_unchecked_mut(i) = init_pos;
        *vc.get_unchecked_mut(i) = (
            fast_parse(&inp[v_pos + 2..c2_pos]),
            fast_parse(&inp[c2_pos + 1..end_pos]),
        );
        start = end_pos + 1;
        i += 1;
    }
    
    let mut rw_len: usize;
    let mut sec_cnt = 0;
    'outer: loop {
        for r in 0..HU {
            rw_len = 0;
            for c in 0..WU {
                if *cur_occ.get_unchecked(r * WU + c) {
                    rw_len += 1
                } else {
                    rw_len = 0
                };
                if rw_len == 10 {
                    break 'outer;
                }
            }
        }
        cur_occ.iter_mut().for_each(|c| *c = false);
        for (pos, vel) in pos.iter_mut().zip(vc) {
            pos.0 = (pos.0 + vel.0).rem_euclid(W);
            pos.1 = (pos.1 + vel.1).rem_euclid(H);
            *cur_occ.get_unchecked_mut(pos.0 as usize * WU + pos.1 as usize) = true;
        }
        sec_cnt += 1;
    }

    sec_cnt
}

#[aoc(day14, part1)]
pub fn part1(inp: &str) -> u32 {
    unsafe { part1_inner(inp.as_bytes()) }
}

#[aoc(day14, part2)]
pub fn part2(inp: &str) -> u32 {
    unsafe { part2_inner(inp.as_bytes()) }
}

// #[cfg(test)]
// mod tests {
//     use super::{extended_gcd, part1, part2};

//     #[test]
//     fn part1_test() {
//         let inp = "p=0,4 v=3,-3
// p=6,3 v=-1,-3
// p=10,3 v=-1,2
// p=2,0 v=2,-1
// p=0,0 v=1,3
// p=3,0 v=-2,-2
// p=7,6 v=-1,-3
// p=3,0 v=-1,-2
// p=9,3 v=2,3
// p=7,3 v=-1,2
// p=2,4 v=2,-3
// p=9,5 v=-3,-3
// ";
//         assert_eq!(part1(inp), 12);
//         // let a =
//         println!("{:?}",  extended_gcd(7, 26));
//     }

// }

// ..... .....
// ...........XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
// ...........
//  ..........
// ...........
// ...........
// ...........
