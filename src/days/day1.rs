#![allow(unused)]

use super::file_loader::load_day;
use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};
use fxhash::FxHashMap;
// use nohash::IntMap;

fn unpack_inp(inp: String) -> (Vec<usize>, Vec<usize>) {
    inp.lines()
        .map(str::split_whitespace)
        .map(|mut spl| {
            (
                spl.next()
                    .expect("atleast 2")
                    .parse::<usize>()
                    .expect("fault input"),
                spl.next().unwrap().parse::<usize>().expect("fault input"),
            )
        })
        .collect()
}


#[aoc_generator(day1)]
fn unpack_inp_opt(inp: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left = Vec::<u32>::with_capacity(1000);
    let mut right = Vec::<u32>::with_capacity(1000);
    for s in 0..1000 {
        let p = s * 14;
        left.push(inp[p..p+5].parse().unwrap());
        right.push(inp[p+8..p+13].parse().unwrap());
    }
    (left, right)
}

#[aoc(day1, part1)]
fn part_1_solve(lsts:&(Vec<u32>, Vec<u32>)) -> u32 {
    let mut l_list = lsts.0.clone();
    let mut r_list = lsts.1.clone();

    l_list.sort_unstable();
    r_list.sort_unstable();

    l_list
        .into_iter()
        .zip(r_list)
        .fold(0, |acc, (l, r)| acc + l.abs_diff(r))
}


pub fn part_1_old(inp: String) -> usize {
    let (mut l_list, mut r_list) = unpack_inp(inp);
    l_list.sort_unstable();
    r_list.sort_unstable();

    l_list
        .into_iter()
        .zip(r_list)
        .fold(0usize, |acc, (l, r)| acc + l.abs_diff(r))
}

fn part_1(inp: &str) -> u32 {
    let (mut l_list, mut r_list) = unpack_inp_opt(inp);
    l_list.sort_unstable();
    r_list.sort_unstable();

    l_list
        .into_iter()
        .zip(r_list)
        .fold(0, |acc, (l, r)| acc + l.abs_diff(r))
}

fn freq_map(lst: Vec<usize>) -> HashMap<usize, usize> {
    let mut freq_map = HashMap::new();
    for num in lst {
        match freq_map.get_mut(&num) {
            Some(cnt) => *cnt += 1,
            None => {
                freq_map.insert(num, 1usize);
            }
        }
    }
    freq_map
}

fn day_12_work(inp: String) -> usize {
    let (l_list, r_list) = unpack_inp(inp);
    let (l_freq, r_freq) = (freq_map(l_list), freq_map(r_list));
    l_freq.iter().fold(0usize, |acc, (val, cnt_l)| {
        if let Some(cnt_r) = r_freq.get(val) {
            acc + val * cnt_l * cnt_r
        } else {
            acc
        }
    })
}

fn freq_map_opt(lst: &[u32]) -> FxHashMap<u32, u32> {
    let mut freq_map = FxHashMap::default();
    freq_map.reserve(1000);
    for &num in lst {
        match freq_map.get_mut(&num) {
            Some(cnt) => *cnt += 1,
            None => {
                freq_map.insert(num, 1u32);
            }
        }
    }
    freq_map
}

#[aoc(day1, part2)]
fn day_1_part_2_solve(lsts:&(Vec<u32>, Vec<u32>)) -> u32 {
    let (l_freq, r_freq) = (freq_map_opt(&lsts.0), freq_map_opt(&lsts.1));
    l_freq.iter().fold(0, |acc, (val, cnt_l)| {
        if let Some(cnt_r) = r_freq.get(val) {
            acc + val * cnt_l * cnt_r
        } else {
            acc
        }
    })
}

pub fn part_2(inp: &str) -> u32 {
    let (l_list, r_list) = unpack_inp_opt(inp);
    let (l_freq, r_freq) = (freq_map_opt(&l_list), freq_map_opt(&r_list));
    l_freq.iter().fold(0, |acc, (val, cnt_l)| {
        if let Some(cnt_r) = r_freq.get(val) {
            acc + val * cnt_l * cnt_r
        } else {
            acc
        }
    })
}


fn part_1_test() -> u32 {
    let inp = load_day(1, true);
    part_1(&inp)
}