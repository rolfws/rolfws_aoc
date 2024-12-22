#![allow(unused)]

use std::slice::Windows;

use aoc_runner_derive::aoc;
use memchr::memchr_iter;


unsafe fn parse_u8(ps: &[u8]) -> u32 {
    let mut out = 0;
    for b in ps {
        out *= 10;
        out += (b - b'0') as u32;
    }
    out
}

fn evolve_secret(mut secret:u32) -> u32 {
    secret = ((secret << 6) ^ secret) & 16777215;
    secret = ((secret >> 5) ^ secret) & 16777215;
    secret = ((secret << 11) ^ secret) & 16777215;
    secret
}

unsafe fn part1_inner(inp:&[u8]) -> u64 {
    let mut prev = 0;
    let mut iter = memchr_iter(b'\n', inp);
    let mut cnt = 0;
    while prev < inp.len() {
        let next = iter.next().unwrap_or(inp.len());
        let mut inp = parse_u8(&inp[prev..next]);
        for _ in 0..2000 {
            inp = evolve_secret(inp)
        }
        cnt += inp as u64;
        prev = next + 1;
    }
    cnt
}

unsafe fn uslc_to_ind(slc:[usize;4]) -> usize {
    slc[0] + slc[1] * 19 + slc[2] * 361 + slc[3] * 6859
}

unsafe fn part2_inner_o(inp:&[u8]) -> u16 {
    let mut out = [0u16; 19 * 19 * 19 * 19];
    let mut prev = 0;
    let mut iter = memchr_iter(b'\n', inp);
    let mut cnt = 0;
    let mut change = [0usize; 2000];
    let mut bananas = [0u16; 2000];
    let w_ptr = change.as_ptr();
    while prev < inp.len() {
        let mut seen = [false; 19 * 19 * 19 * 19];
        let next = iter.next().unwrap_or(inp.len());
        let mut inp = parse_u8(&inp[prev..next]);
        prev = next + 1;
        let _ = change.iter_mut().zip(bananas.iter_mut()).fold(inp, |state, (c, b)| {
            let evolved = evolve_secret(state);
            *c = (evolved % 10 + 9) as usize - (state % 10) as usize;
            *b = (evolved % 10) as u16;
            evolved
        });
        // println!("{:?}", change);
        for i in 0..1997 {
            let w = (w_ptr.add(i) as *const [usize; 4]).read();
            let ind = uslc_to_ind(w);
            if !*seen.get_unchecked(ind) {
                *seen.get_unchecked_mut(ind) = true;
                *out.get_unchecked_mut(ind) += bananas.get_unchecked(i + 3);
            }
        }
    }
    out.into_iter().max().unwrap()
}

#[aoc(day22, part1)]
pub fn part1(inp: &str) -> u64 {
    unsafe { part1_inner(inp.as_bytes()) }
}

#[aoc(day22, part2)]
pub fn part2(inp: &str) -> u16 {
    unsafe { part2_inner_o(inp.as_bytes()) }
}

#[cfg(test)]
mod tests{
    use super::{part2,part1};

    #[test]
    fn part1_test() {
        let inp = "1
10
100
2024
";
        assert_eq!(part1(inp), 37327623)
    }

    #[test]
    fn part2_test() {
        let inp = "1
2
3
2024
";
        assert_eq!(part2(inp), 23)
    }
}