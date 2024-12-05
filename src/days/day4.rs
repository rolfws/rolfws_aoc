// #![allow(unused)]
use core::str;

use aoc_runner_derive::aoc;

#[inline(always)]
unsafe fn part1_xmas_count(x: u64, m: u64, a: u64, s: u64) -> u32 {
    (x & m & a & s).count_ones()
}

unsafe fn part1_vector(inp: &str) -> u32 {
    let line_len: usize = inp.find('\n').unwrap() + 1;
    let mut line_num = inp.len() / line_len;
    if inp.len() % line_len != 0 {
        line_num += 1; // Some times the last \n gets stripped, then the last line is incomplete
    }

    let mut xb: Vec<bool> = Vec::with_capacity(inp.len() + 8);
    xb.extend(
        inp.bytes()
            .map(|b| b == b'X')
            .chain([false; 8]),
    );
    let mut mb: Vec<bool> = Vec::with_capacity(inp.len() + 8);
    mb.extend(
        inp.bytes()
            .map(|b| b == b'M')
            .chain([false; 8]),
    );
    let mut ab: Vec<bool> = Vec::with_capacity(inp.len() + 8);
    ab.extend(
        inp.bytes()
            .map(|b| b == b'A')
            .chain([false; 8]),
    );
    //inp.bytes().map(|b| b == b'X').collect();
    let mut sb: Vec<bool> = Vec::with_capacity(inp.len() + 8);
    sb.extend(
        inp.bytes()
            .map(|b| b == b'S')
            .chain([false; 8]),
    );
    let mut x_cnt = 0;

    let x_ptr = xb.as_ptr();
    let m_ptr = mb.as_ptr();
    let a_ptr = ab.as_ptr();
    let s_ptr = sb.as_ptr();

    let mut reps = (line_num - 3) * line_len / 8;
    if (line_num - 3) * line_len % 8 != 0 {
        reps += 1;
    }
    let mut count = 0;
    for _ in 0..reps {
        // Horizontal
        x_cnt += part1_xmas_count(
            (x_ptr.add(count) as *const u64).read(),
            (m_ptr.add(count + 1) as *const u64).read(),
            (a_ptr.add(count + 2) as *const u64).read(),
            (s_ptr.add(count + 3) as *const u64).read(),
        );
        x_cnt += part1_xmas_count(
            (x_ptr.add(count + 3) as *const u64).read(),
            (m_ptr.add(count + 2) as *const u64).read(),
            (a_ptr.add(count + 1) as *const u64).read(),
            (s_ptr.add(count) as *const u64).read(),
        );

        // Vertical
        x_cnt += part1_xmas_count(
            (x_ptr.add(count) as *const u64).read(),
            (m_ptr.add(count + line_len) as *const u64).read(),
            (a_ptr.add(count + 2 * line_len) as *const u64).read(),
            (s_ptr.add(count + 3 * line_len) as *const u64).read(),
        );
        x_cnt += part1_xmas_count(
            (x_ptr.add(count + 3 * line_len) as *const u64).read(),
            (m_ptr.add(count + 2 * line_len) as *const u64).read(),
            (a_ptr.add(count + line_len) as *const u64).read(),
            (s_ptr.add(count) as *const u64).read(),
        );
        
        // Bottom right
        x_cnt += part1_xmas_count(
            (x_ptr.add(count) as *const u64).read(),
            (m_ptr.add(count + line_len + 1) as *const u64).read(),
            (a_ptr.add(count + 2 * line_len + 2) as *const u64).read(),
            (s_ptr.add(count + 3 * line_len + 3) as *const u64).read(),
        );
        x_cnt += part1_xmas_count(
            (x_ptr.add(count + 3 * line_len + 3) as *const u64).read(),
            (m_ptr.add(count + 2 * line_len + 2) as *const u64).read(),
            (a_ptr.add(count + line_len + 1) as *const u64).read(),
            (s_ptr.add(count) as *const u64).read(),
        );

        // Bottom left
        x_cnt += part1_xmas_count(
            (x_ptr.add(count) as *const u64).read(),
            (m_ptr.add(count + line_len - 1) as *const u64).read(),
            (a_ptr.add(count + 2 * line_len - 2) as *const u64).read(),
            (s_ptr.add(count + 3 * line_len - 3) as *const u64).read(),
        );
        x_cnt += part1_xmas_count(
            (x_ptr.add(count + 3 * line_len - 3) as *const u64).read(),
            (m_ptr.add(count + 2 * line_len - 2) as *const u64).read(),
            (a_ptr.add(count + line_len - 1) as *const u64).read(),
            (s_ptr.add(count) as *const u64).read(),
        );
        count += 8;
    }

    for _ in reps..(xb.len() / 8) {
        // Horizontal
        x_cnt += part1_xmas_count(
            (x_ptr.add(count) as *const u64).read(),
            (m_ptr.add(count + 1) as *const u64).read(),
            (a_ptr.add(count + 2) as *const u64).read(),
            (s_ptr.add(count + 3) as *const u64).read(),
        );
        x_cnt += part1_xmas_count(
            (x_ptr.add(count + 3) as *const u64).read(),
            (m_ptr.add(count + 2) as *const u64).read(),
            (a_ptr.add(count + 1) as *const u64).read(),
            (s_ptr.add(count) as *const u64).read(),
        );
        count += 8;
    }
    x_cnt
}


#[inline(always)]
fn check_mas(m: u64, a:u64, s:u64) -> u64 {
    m & a & s
}


unsafe fn part2_vec(inp: &str) -> u32 {
    let line_len: usize = inp.find('\n').unwrap() + 1;
    let mut line_num = inp.len() / line_len;
    if inp.len() % line_len != 0 {
        line_num += 1; // Some times the last \n gets stripped, then the last line is incomplete
    }
    let mut mb: Vec<bool> = Vec::with_capacity(inp.len() + 8);
    mb.extend(
        inp.bytes()
            .map(|b| b == b'M')
            .chain([false; 8]),
    );
    let mut ab: Vec<bool> = Vec::with_capacity(inp.len() + 8);
    ab.extend(
        inp.bytes()
            .map(|b| b == b'A')
            .chain([false; 8]),
    );
    let mut sb: Vec<bool> = Vec::with_capacity(inp.len() + 8);
    sb.extend(
        inp.bytes()
            .map(|b| b == b'S')
            .chain([false; 8]),
    );
    let mut x_cnt = 0;

    let m_ptr = mb.as_ptr();
    let a_ptr = ab.as_ptr();
    let s_ptr = sb.as_ptr();
    let mut count = line_len + 1;
    let reps = (line_num - 2) * line_len / 8;
    for _ in 0..reps {
        let mid_a = (a_ptr.add(count) as *const u64).read();
        let mas_1 = check_mas(
            (m_ptr.add(count - line_len - 1) as *const u64).read(),
            mid_a,
            (s_ptr.add(count + line_len + 1) as *const u64).read()
        ) | check_mas(
            (m_ptr.add(count + line_len + 1) as *const u64).read(),
            mid_a,
            (s_ptr.add(count - line_len - 1) as *const u64).read()
        );

        let mas_2 = check_mas(
            (m_ptr.add(count - line_len + 1) as *const u64).read(),
            mid_a,
            (s_ptr.add(count + line_len - 1) as *const u64).read()
        ) | check_mas(
            (m_ptr.add(count + line_len - 1) as *const u64).read(),
            mid_a,
            (s_ptr.add(count - line_len + 1) as *const u64).read()
        );

        x_cnt += (mas_2 & mas_1).count_ones();
        count += 8;
    }
    x_cnt
}

#[aoc(day4, part1)]
pub fn part1(inp: &str) -> u32 {
    unsafe { part1_vector(inp) }
}

#[aoc(day4, part2)]
pub fn part2(inp: &str) -> u32 {
    unsafe { part2_vec(inp) }
}