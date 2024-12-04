#![allow(unused)]
use core::str;

use aoc_runner_derive::aoc;

fn xmax_match(inp: [u8; 4]) -> bool {
    matches!(inp, [b'X', b'M', b'A', b'S'] | [b'S', b'A', b'M', b'X'])
}

unsafe fn part1_core(inp: &str) -> u32 {
    let line_len: usize = inp.lines().next().unwrap().len() + 1; // Without \n
    let line_num: usize = inp.lines().count();

    let ptr = inp.as_ptr();
    let mut x_cnt = 0u32;
    let mut j: usize;

    // Horizontal (flipped or unflipped)

    for i in (0..line_num) {
        let line_start = ptr.add(i * line_len);
        j = 0;
        while j < line_len - 4 {
            // Another one because the \n
            if xmax_match([
                line_start.add(j).read(),
                line_start.add(j + 1).read(),
                line_start.add(j + 2).read(),
                line_start.add(j + 3).read(),
            ]) {
                x_cnt += 1;
                j += 3 // Last letter is an S/X so we we need to check for flip aswell
            } else {
                j += 1;
            }
        }
    }

    // Vertical (flipped or unflipped) (column, to skip more)
    for i in (0..line_len - 1) {
        let col_start = ptr.add(i);
        j = 0;
        while j < line_num - 3 {
            if xmax_match([
                col_start.add(j * line_len).read(),
                col_start.add((j + 1) * line_len).read(),
                col_start.add((j + 2) * line_len).read(),
                col_start.add((j + 3) * line_len).read(),
            ]) {
                x_cnt += 1;
                j += 3 // Last letter is an S/X so we we need to check for flip aswell
            } else {
                j += 1;
            }
        }
    }

    // S(.|\n){11}A(.|\n){11}M(.|\n){11}X
    // X(.|\n){11}M(.|\n){11}A(.|\n){11}S

    // Left top to right bottom
    for i in (0..line_num - 3) {
        let line_start = ptr.add(i * line_len);
        j = 0;
        while j < line_len - 4 {
            // Another one because the \n
            if xmax_match([
                line_start.add(j).read(),
                line_start.add(j + line_len + 1).read(),
                line_start.add(j + 2 * line_len + 2).read(),
                line_start.add(j + 3 * line_len + 3).read(),
            ]) {
                x_cnt += 1
            }
            j += 1
        }
    }

    // right top to left bottom
    for i in (0..line_num - 3) {
        let line_start = ptr.add(i * line_len);
        j = 3;
        while j < line_len - 1 {
            // Another one because the \n
            if xmax_match([
                line_start.add(j).read(),
                line_start.add(j + line_len - 1).read(),
                line_start.add(j + 2 * line_len - 2).read(),
                line_start.add(j + 3 * line_len - 3).read(),
            ]) {
                x_cnt += 1
            }
            j += 1
        }
    }
    x_cnt
}

unsafe fn part1_vector(inp: &str) -> u32 {
    let line_len: usize = inp.lines().next().unwrap().len() + 1; // Without \n
    let xb: Vec<bool> = inp.bytes().map(|b| b == b'X').collect();
    let mb: Vec<bool> = inp.bytes().map(|b| b == b'X').collect();
    let ab: Vec<bool> = inp.bytes().map(|b| b == b'X').collect();
    let sb: Vec<bool> = inp.bytes().map(|b| b == b'X').collect();
    let mut x_cnt = 0;
    let mut i = 0;

    // If any wrapping is hit a \n is encoutered, which is false anyway
    for i in 0..xb.len() - 3 {
        if (xb[i] && mb[i + 1] && ab[i + 2] && sb[i + 3])
            || (xb[i + 3] && mb[i + 2] && ab[i + 1] && sb[i + 1])
        {
            x_cnt += 1;
        }

        if (i < xb.len() - line_len * 3)
            && ((xb[i] && mb[i + line_len] && ab[i + 2 * line_len] && sb[i + 3 * line_len])
                || (sb[i] && ab[i + line_len] && mb[i + 2 * line_len] && xb[i + 3 * line_len]))
        {
            x_cnt += 1;
        }
        if (i < xb.len() - line_len * 3 - 3)
            && ((xb[i] && mb[i + line_len + 1] && ab[i + 2 * line_len + 2] && sb[i + 3 * line_len + 3])
                || (sb[i] && ab[i + line_len + 1] && mb[i + 2 * line_len + 2] && xb[i + 3 * line_len + 3]))
        {
            x_cnt += 1;
        }
        if (i < xb.len() - line_len * 3) 
            && ((xb[i] && mb[i + line_len - 1] && ab[i + 2 * line_len - 2] && sb[i + 3 * line_len - 3])
            || (sb[i] && ab[i + line_len - 1] && mb[i + 2 * line_len - 2] && xb[i + 3 * line_len - 3])) {
                x_cnt += 1;
            }
    }
    x_cnt
}

fn check_xmas(inp: [u8; 4]) -> bool {
    matches!(inp[..2], [b'M', b'S'] | [b'S', b'M'])
        & matches!(inp[2..], [b'M', b'S'] | [b'S', b'M'])
}

unsafe fn part2_core(inp: &str) -> u32 {
    let line_len: usize = inp.lines().next().unwrap().len() + 1; // Without \n
    let line_num: usize = inp.lines().count();

    let ptr = inp.as_ptr();
    let mut x_cnt = 0u32;
    let mut j: usize;
    // We walk the inner box, then if we find an A, we check for X - MAS
    for i in (1..line_num - 1) {
        for j in (1..line_len - 2) {
            let middle_ptr = ptr.add(i * line_len + j);
            if (middle_ptr.read() == b'A')
                && check_xmas([
                    middle_ptr.sub(line_len + 1).read(),
                    middle_ptr.add(line_len + 1).read(),
                    middle_ptr.sub(line_len - 1).read(),
                    middle_ptr.add(line_len - 1).read(),
                ])
            {
                x_cnt += 1;
            }
        }
    }
    x_cnt
}

#[aoc(day4, part1)]
pub fn part1(inp: &str) -> u32 {
    unsafe { part1_core(inp) }
}

#[aoc(day4, part1, vec)]
pub fn part1_v(inp: &str) -> u32 {
    unsafe { part1_vector(inp) }
}

#[aoc(day4, part2)]
pub fn part2(inp: &str) -> u32 {
    unsafe { part2_core(inp) }
}
