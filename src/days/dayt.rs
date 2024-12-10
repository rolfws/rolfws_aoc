#![allow(unused)]

use aoc_runner_derive::aoc;

unsafe fn part1_inner(inp:&[u8]) -> u32 {
    0
}

unsafe fn part2_inner(_inp:&[u8]) -> u32 {
    0
}

#[aoc(day10, part1)]
pub fn part1(inp: &str) -> u32 {
    unsafe { part1_inner(inp.as_bytes()) }
}

#[aoc(day10, part2)]
pub fn part2(inp: &str) -> u32 {
    unsafe { part2_inner(inp.as_bytes()) }
}

#[cfg(test)]
mod tests{
    use super::{part2,part1};

    #[test]
    fn part1_test() {
        let inp = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
        assert_eq!(part1(inp), 36)
    }
}