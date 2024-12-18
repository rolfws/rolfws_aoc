// #![allow(unused)]

use std::collections::VecDeque;

use aoc_runner_derive::aoc;
use memchr::memchr_iter;

#[cfg(test)]
const S: usize = 7;
#[cfg(not(test))]
const S: usize = 71;

fn fast_parse(slc: &[u8]) -> usize {
    let mut out = 0usize;
    for b in slc {
        out *= 10;
        out += (b - b'0') as usize
    }
    out
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct CellPrio(usize, usize);

impl PartialOrd for CellPrio {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CellPrio {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0) // Reverse as lower value needs to give higer proio
    }
}

unsafe fn load_blocks(inp: &[u8], take: usize) -> [bool; S * S] {
    let mut blocked = [false; S * S];
    let mut prev = 0;
    for (c, end) in memchr_iter(b',', inp)
        .take(take)
        .zip(memchr_iter(b'\n', inp).chain(std::iter::once(inp.len() - 1)))
    {
        *blocked.get_unchecked_mut(fast_parse(&inp[prev..c]) + fast_parse(&inp[c + 1..end]) * S) =
            true;
        prev = end + 1;
    }
    blocked
}

unsafe fn a_star_que(blocked: [bool; S * S]) -> usize {
    let mut closed = [false; S * S];
    let mut gs = [usize::MAX; S * S];
    let mut fs = [usize::MAX; S * S];

    gs[0] = 0;
    fs[0] = 2 * S - 2;

    // let mut open: BinaryHeap<CellPrio> = BinaryHeap::with_capacity(S);
    let mut open: VecDeque<CellPrio> = VecDeque::with_capacity(S);
    open.push_back(CellPrio(0, 0));
    let end = S * S - 1;
    let mut out: usize = 0;
    'outer: while !open.is_empty() {
        let CellPrio(_, node_idx) = open.pop_front().expect("not empty");
        *closed.get_unchecked_mut(node_idx) = true;
        // for i in dirs.iter().filter_map(|d| d.offset(node_idx)) {
        if node_idx >= S {
            let i = node_idx - S;
            if !*blocked.get_unchecked(i) && !*closed.get_unchecked(i) {
                if i == end {
                    out = gs.get_unchecked(node_idx) + 1;
                    break 'outer;
                } else {
                    let g_new = gs.get_unchecked(node_idx) + 1;
                    let h_new = (S - 1 - i % S) + (S - 1 - i / S);
                    let f_new = g_new + h_new;
                    if *fs.get_unchecked(i) > f_new {
                        open.push_back(CellPrio(f_new, i));
                        *fs.get_unchecked_mut(i) = f_new;
                        *gs.get_unchecked_mut(i) = g_new;
                    }
                }
            }
        }
        if node_idx % S < S - 1 {
            let i = node_idx + 1;
            if !*blocked.get_unchecked(i) && !*closed.get_unchecked(i) {
                if i == end {
                    out = gs.get_unchecked(node_idx) + 1;
                    break 'outer;
                } else {
                    let g_new = gs.get_unchecked(node_idx) + 1;
                    let h_new = (S - 1 - i % S) + (S - 1 - i / S);
                    let f_new = g_new + h_new;
                    if *fs.get_unchecked(i) > f_new {
                        open.push_back(CellPrio(f_new, i));
                        *fs.get_unchecked_mut(i) = f_new;
                        *gs.get_unchecked_mut(i) = g_new;
                    }
                }
            }
        }
        if node_idx < S * (S - 1) {
            let i = node_idx + S;
            if !*blocked.get_unchecked(i) && !*closed.get_unchecked(i) {
                if i == end {
                    out = gs.get_unchecked(node_idx) + 1;
                    break 'outer;
                } else {
                    let g_new = gs.get_unchecked(node_idx) + 1;
                    let h_new = (S - 1 - i % S) + (S - 1 - i / S);
                    let f_new = g_new + h_new;
                    if *fs.get_unchecked(i) > f_new {
                        open.push_back(CellPrio(f_new, i));
                        *fs.get_unchecked_mut(i) = f_new;
                        *gs.get_unchecked_mut(i) = g_new;
                    }
                }
            }
        }
        if node_idx % S > 0 {
            let i = node_idx - 1;
            if !*blocked.get_unchecked(i) && !*closed.get_unchecked(i) {
                if i == end {
                    out = gs.get_unchecked(node_idx) + 1;
                    break 'outer;
                } else {
                    let g_new = gs.get_unchecked(node_idx) + 1;
                    let h_new = (S - 1 - i % S) + (S - 1 - i / S);
                    let f_new = g_new + h_new;
                    if *fs.get_unchecked(i) > f_new {
                        open.push_back(CellPrio(f_new, i));
                        *fs.get_unchecked_mut(i) = f_new;
                        *gs.get_unchecked_mut(i) = g_new;
                    }
                }
            }
        }
    }
    out
}

#[cfg(test)]
const T: usize = 12;
#[cfg(not(test))]
const T: usize = 1024;

unsafe fn part1_inner(inp: &[u8]) -> usize {
    let blocked = load_blocks(inp, T);
    a_star_que(blocked)
}

#[cfg(test)]
const U: usize = 25;
#[cfg(not(test))]
const U: usize = 3450;

unsafe fn load_block_locs(inp: &[u8]) -> (Vec<(usize, usize)>, Vec<usize>) {
    let mut blocked = Vec::with_capacity(U);
    let mut locs = Vec::with_capacity(U);
    let mut prev = 0;
    for (c, end) in
        memchr_iter(b',', inp).zip(memchr_iter(b'\n', inp).chain(std::iter::once(inp.len() - 1)))
    {
        let x = fast_parse(&inp[prev..c]);
        let y = fast_parse(&inp[c + 1..end]);
        locs.push((x, y));
        blocked.push(x + y * S);
        prev = end + 1;
    }
    (locs, blocked)
}

unsafe fn part2_inner(inp: &[u8]) -> String {
    let mut lower_bound = T;
    let (locs_prety, locs) = load_block_locs(inp);
    let mut upper_bound = locs.len();
    // println!("{locs:?}");
    let mut blocked = [false; S * S];
    let mut prev_test = T;
    // let (blocked, prev) = load_blocks(inp, T);
    locs[..lower_bound]
        .iter()
        .for_each(|&i| *blocked.get_unchecked_mut(i) = true);
    loop {
        let t = (upper_bound - lower_bound) / 2 + lower_bound;
        // locs[..t].iter().for_each(|&i| *blocked.get_unchecked_mut(i) = true);

        if t > prev_test {
            locs[prev_test..t]
                .iter()
                .for_each(|&i| *blocked.get_unchecked_mut(i) = true);
        } else {
            locs[t..prev_test]
                .iter()
                .for_each(|&i| *blocked.get_unchecked_mut(i) = false);
        }
        prev_test = t;
        let len = a_star_que(blocked);
        if len == 0 {
            upper_bound = t
        } else {
            lower_bound = t
        }
        if upper_bound == lower_bound + 1 {
            break;
        }
    }
    let l = *locs_prety.get_unchecked(lower_bound);

    format!("{},{}", l.0, l.1)
}

#[aoc(day18, part1)]
pub fn part1(inp: &str) -> usize {
    unsafe { part1_inner(inp.as_bytes()) }
}

#[aoc(day18, part2)]
pub fn part2(inp: &str) -> String {
    unsafe { part2_inner(inp.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn part1_test() {
        let inp = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";
        assert_eq!(part1(inp), 22);

        assert_eq!(part2(inp), "6,1")
    }
}
