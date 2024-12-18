#![allow(unused)]

use aoc_runner_derive::aoc;
use memchr::memchr_iter;
use itertools::{enumerate, Itertools};

fn fast_parse(slc: &[u8]) -> u64 {
    let mut out = 0u64;
    for b in slc {
        out *= 10;
        out += (b - b'0') as u64
    }
    out
}

unsafe fn load_instructions(inp:&[u8]) -> (u64, u64, u64, Vec<u8>) {
    let mut nriter = memchr_iter(b'\n', inp);
    let nr = nriter.next().expect("atleast one");
    let mut ar = fast_parse(&inp[12..nr]);
    let nr2 = nriter.next().expect("atleast 2");
    let mut br = fast_parse(&inp[nr + 13..nr2]);
    let nr3 = nriter.next().expect("atleast 3");
    let mut cr = fast_parse(&inp[nr2 + 13..nr3]);
    let instruction = inp[nr3+11..].iter().step_by(2).map(|i| i - b'0').collect();
    (ar, br, cr, instruction)
}

fn load_combo(comb: u8, ar: u64, br: u64, cr: u64) -> u64 {
    match comb {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => ar,
        5 => br,
        6 => cr,
        _ => unreachable!("faulty combo input"),
    }
}

unsafe fn do_instructions(mut ar:u64, mut br:u64, mut cr:u64, instr:&[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(20);
    let mut cur_instr = 0;
    while cur_instr < instr.len() {
        // println!("{cur_instr}, {ar}, {br}, {cr}");
        match instr.get_unchecked(cur_instr) {
            0 => {
                ar >>= load_combo(*instr.get_unchecked(cur_instr + 1), ar, br, cr);
                cur_instr += 2;
            }
            1 => {
                br ^= *instr.get_unchecked(cur_instr + 1) as u64;
                cur_instr += 2;
            },
            2 => {
                br = load_combo(*instr.get_unchecked(cur_instr + 1), ar, br, cr) % 8;
                cur_instr += 2;
            },
            3 => {
                if ar != 0 {
                    cur_instr = *instr.get_unchecked(cur_instr + 1) as usize;
                } else {
                    cur_instr += 2;
                }
            },
            4 => {
                br ^= cr;
                cur_instr += 2;
            },
            5 => {
                // println!("out");
                out.push((load_combo(*instr.get_unchecked(cur_instr + 1), ar, br, cr) % 8 )as u8);
                cur_instr += 2;
            },
            6 => {
                br = ar >> load_combo(*instr.get_unchecked(cur_instr + 1), ar, br, cr);
                cur_instr += 2;
            },
            7 => {
                cr = ar >> load_combo(*instr.get_unchecked(cur_instr + 1), ar, br, cr);
                cur_instr += 2;
            },
            _ => unreachable!("faulty input"),
        }
    }
    out
}


unsafe fn part1_inner(inp: &[u8]) -> String {
    let (ar, br, cr, instructions) = load_instructions(inp);
    let out = do_instructions(ar, br, cr, &instructions);
    out.iter().join(",")
}

unsafe fn part2_inner(inp: &[u8]) -> u64 {
    let (_, _, _, instructions) = load_instructions(inp);
    let mut start_as = vec![0];
    for (exp, g) in instructions.iter().enumerate().rev() {
        let mut next_as = vec![];
        for a in start_as {
            for i in 0..8 {
                let out = do_instructions(a + i * 8u64.pow(exp as u32), 0, 0, &instructions);
                if out.len() > exp && *out.get_unchecked(exp) == *g {
                    next_as.push(a + i * 8u64.pow(exp as u32));
                }
            }
        }
        start_as = next_as;
    }
    start_as[0]
}

#[aoc(day17, part1)]
pub fn part1(inp: &str) -> String {
    unsafe { part1_inner(inp.as_bytes()) }
}

#[aoc(day17, part2)]
pub fn part2(inp: &str) -> u64 {
    unsafe { part2_inner(inp.as_bytes()) }
}