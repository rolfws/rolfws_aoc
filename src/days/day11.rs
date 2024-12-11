#![allow(unused)]

use std::num::NonZero;

use aoc_runner_derive::aoc;
use fxhash::FxHashMap;
use memchr::memchr_iter;

fn fast_parseu64(slc: &[u8]) -> u64 {
    let mut out = 0u64;
    for b in slc {
        out *= 10;
        out += (b - b'0') as u64
    }
    out
}

fn blink_stone(blinks_rem: usize, stone_num: u64, cache: &mut FxHashMap<(usize, u64), u64>) -> u64 {
    if let Some(a) = cache.get(&(blinks_rem, stone_num)) {
        return *a;
    }
    // println!("{blinks_rem}, {stone_num}");
    if blinks_rem == 0 {
        return 1;
    } else if stone_num == 0 {
        let ret = blink_stone(blinks_rem - 1, 1, cache);
        cache.insert((blinks_rem, stone_num), ret);
        return ret;
    }
    let ints = (stone_num.ilog10() + 1);
    if ints % 2 == 0 {
        let div = 10u64.pow(ints / 2);
        let ret = blink_stone(blinks_rem - 1, stone_num / div, cache)
            + blink_stone(blinks_rem - 1, stone_num % div, cache);
        cache.insert((blinks_rem, stone_num), ret);
        ret
    } else {
        let ret = blink_stone(blinks_rem - 1, stone_num * 2024, cache);
        cache.insert((blinks_rem, stone_num), ret);
        ret
    }
}

unsafe fn part1_inner(inp: &[u8]) -> u64 {
    let mut prev: usize = 0;
    let addit = (inp[inp.len() - 1] == b'\n') as usize;
    let mut cnt = 0u64;
    let mut cache = FxHashMap::default();
    cache.reserve(5000);
    for i in memchr_iter(b' ', inp).chain([inp.len() - addit]) {
        let stone_num = fast_parseu64(&inp[prev..i]);
        prev = i + 1;
        cnt += blink_stone(25, stone_num, &mut cache)
    }
    cnt
}

unsafe fn part2_inner(inp: &[u8]) -> u64 {
    let mut prev: usize = 0;
    let addit = (inp[inp.len() - 1] == b'\n') as usize;
    let mut cnt = 0u64;
    let mut cache = FxHashMap::default();
    cache.reserve(150_000);
    for i in memchr_iter(b' ', inp).chain([inp.len() - addit]) {
        let stone_num = fast_parseu64(&inp[prev..i]);
        prev = i + 1;
        cnt += blink_stone(75, stone_num, &mut cache)
    }
    cnt
}

#[aoc(day11, part1)]
pub fn part1(inp: &str) -> u64 {
    unsafe { part1_inner(inp.as_bytes()) }
}

#[aoc(day11, part2)]
pub fn part2(inp: &str) -> u64 {
    unsafe { part2_inner(inp.as_bytes()) }
}

// #[cfg(test)]
// mod tests {
//     use super::{part1, part2};

//     #[test]
//     fn part1_test() {
//         let inp = "125 17\n";
//         assert_eq!(part1(inp), 55312);
//     }
// }
