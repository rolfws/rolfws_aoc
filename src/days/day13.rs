#![allow(unused)]

use itertools::izip;
use memchr::memchr;

use aoc_runner_derive::aoc;

fn fast_parse(slc: &[u8]) -> i32 {
    let mut out = 0i32;
    for b in slc {
        out *= 10;
        out += (b - b'0') as i32
    }
    out
}

unsafe fn part1_inner(inp:&[u8]) -> i32 {
    // let mut eqiter = memchr::memchr_iter(b'=', inp);
    let mut start_ind = 0usize;
    let mut token_cnt: i32 = 0;
    loop {
        let f1 = fast_parse(&inp[start_ind + 12.. start_ind+14]);
        let f2 = fast_parse(&inp[start_ind + 18.. start_ind+20]);
        let g1 = fast_parse(&inp[start_ind + 33.. start_ind+35]);
        let g2 = fast_parse(&inp[start_ind + 39.. start_ind+41]);
        let x_len = memchr(b',', &inp[start_ind+51..]).expect("There will be an X");
        let x = fast_parse(&inp[start_ind + 51.. start_ind+51+x_len]);
        let y_len = memchr(b'\n', &inp[start_ind + 55 +x_len..]).unwrap_or(inp.len() - start_ind - x_len - 55);
        let y = fast_parse(&inp[start_ind + x_len + 55.. start_ind+x_len+55 + y_len]);
        start_ind = start_ind + x_len + y_len + 57;

        let b = (y * f1 - x * f2) / (g2 * f1 - g1 * f2);
        let a = (y - b * g2) / f2;
        if a < 100 && b <100 && a * f1 + b * g1 == x && a * f2 + b * g2 == y {
            token_cnt += 3 * a + b;
        }
        
        if start_ind >= inp.len() {
            break
        }
        // let y_eq_ind= eqiter.next().expect("No X without Y");
    }

    token_cnt
}

fn fast_parse2(slc: &[u8]) -> i64 {
    let mut out = 0;
    for b in slc {
        out *= 10;
        out += (b - b'0') as i64
    }
    out
}

unsafe fn part2_inner(inp:&[u8]) -> i64 {
    // let mut eqiter = memchr::memchr_iter(b'=', inp);
    let mut start_ind = 0usize;
    let mut token_cnt: i64 = 0;
    loop {
        let f1 = fast_parse2(&inp[start_ind + 12.. start_ind+14]);
        let f2 = fast_parse2(&inp[start_ind + 18.. start_ind+20]);
        let g1 = fast_parse2(&inp[start_ind + 33.. start_ind+35]);
        let g2 = fast_parse2(&inp[start_ind + 39.. start_ind+41]);
        let x_len = memchr(b',', &inp[start_ind+51..]).expect("There will be an X");
        let x = fast_parse2(&inp[start_ind + 51.. start_ind+51+x_len]) + 10_000_000_000_000;
        let y_len = memchr(b'\n', &inp[start_ind + 55 +x_len..]).unwrap_or(inp.len() - start_ind - x_len - 55);
        let y = fast_parse2(&inp[start_ind + x_len + 55.. start_ind+x_len+55 + y_len]) + 10_000_000_000_000;
        start_ind = start_ind + x_len + y_len + 57;

        let b = (y * f1 - x * f2) / (g2 * f1 - g1 * f2);
        let a = (y - b * g2) / f2;
        if a * f1 + b * g1 == x && a * f2 + b * g2 == y {
            token_cnt += 3 * a + b;
        }
        
        if start_ind >= inp.len() {
            break
        }
        // let y_eq_ind= eqiter.next().expect("No X without Y");
    }
    token_cnt
}

#[aoc(day13, part1)]
pub fn part1(inp: &str) -> i32 {
    unsafe { part1_inner(inp.as_bytes()) }
}

#[aoc(day13, part2)]
pub fn part2(inp: &str) -> i64 {
    unsafe { part2_inner(inp.as_bytes()) }
}

// #[cfg(test)]
// mod tests{
//     use super::{part2,part1};

//     #[test]
//     fn part1_test() {
//         let inp = "Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400

// Button A: X+26, Y+66
// Button B: X+67, Y+21
// Prize: X=12748, Y=12176

// Button A: X+17, Y+86
// Button B: X+84, Y+37
// Prize: X=7870, Y=6450

// Button A: X+69, Y+23
// Button B: X+27, Y+71
// Prize: X=18641, Y=10279";
//         assert_eq!(part1(inp), 480);

//         let inp = "Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400

// Button A: X+26, Y+66
// Button B: X+67, Y+21
// Prize: X=12748, Y=12176

// Button A: X+17, Y+86
// Button B: X+84, Y+37
// Prize: X=7870, Y=6450

// Button A: X+69, Y+23
// Button B: X+27, Y+71
// Prize: X=18641, Y=10279
// ";
//         assert_eq!(part1(inp), 480)
//     }
//     #[test]
//     fn part2_test() {
//         let inp = "Button A: X+94, Y+34
// Button B: X+22, Y+67
// Prize: X=8400, Y=5400

// Button A: X+26, Y+66
// Button B: X+67, Y+21
// Prize: X=12748, Y=12176

// Button A: X+17, Y+86
// Button B: X+84, Y+37
// Prize: X=7870, Y=6450

// Button A: X+69, Y+23
// Button B: X+27, Y+71
// Prize: X=18641, Y=10279";
//         assert_eq!(part2(inp), 0);
//     }
// }