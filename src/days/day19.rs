// #![allow(unused)]

use aoc_runner_derive::aoc;
use fxhash::FxHashMap;
use memchr::memchr_iter;

const MAP: [usize; 256] = {
    let mut a = [0; 256];
    a[b'w' as usize] = 1;
    a[b'b' as usize] = 2;
    a[b'u' as usize] = 3;
    a[b'r' as usize] = 5;
    a[b'g' as usize] = 4;
    a
};
unsafe fn slc_to_idx(inp: &[u8]) -> usize {
    let mut out = 0usize;
    for (i, &v) in inp.iter().enumerate() {
        out += MAP[v as usize] * 5usize.pow(i as u32)
    }
    out
}
unsafe fn read_towels(inp: &[u8]) -> ([bool; 488280], Vec<&[u8]>) {
    let mut towels = [false; 488280];
    let mut niter = memchr_iter(b'\n', inp);
    let mut start = 0;
    for cind in memchr_iter(b',', inp).chain(std::iter::once(niter.next().expect("first line"))) {
        let ind = slc_to_idx(&inp[start..cind]);
        // println!("{ind}");
        *towels.get_unchecked_mut(ind) = true;
        start = cind + 2;
    }

    let mut start = niter.next().expect("2nd line") + 1;
    let mut patterns = Vec::with_capacity(400);
    for nind in niter {
        patterns.push(&inp[start..nind]);
        start = nind + 1;
    }
    if *inp.last().expect("") != b'\n' {
        patterns.push(&inp[start..]);
    }

    (towels, patterns)
}

unsafe fn make_pattern<'a: 'b, 'b>(
    pat: &'a [u8],
    twls: &[bool; 488280],
    cache: &'b mut FxHashMap<&'a [u8], bool>,
) -> bool {
    if pat.is_empty() {
        return true;
    } else if let Some(r) = cache.get(pat) {
        return *r;
    }
    for i in 1..=std::cmp::min(8, pat.len()) {
        if *twls.get_unchecked(slc_to_idx(&pat[..i])) && make_pattern(&pat[i..], twls, cache) {
            cache.insert(pat, true);
            return true;
        }
    }
    cache.insert(pat, false);
    false
}

unsafe fn part1_inner(inp: &[u8]) -> u32 {
    let (towels, patterns) = read_towels(inp);
    let mut cache: FxHashMap<&[u8], bool> = FxHashMap::default();
    cache.reserve(15000);
    let mut cnt = 0;
    for pat in patterns {
        if make_pattern(pat, &towels, &mut cache) {
            cnt += 1;
        }
    }
    cnt
}

unsafe fn make_pattern_cnt<'a: 'b, 'b>(
    pat: &'a [u8],
    twls: &[bool; 488280],
    cache: &'b mut FxHashMap<&'a [u8], u64>,
) -> u64 {
    if pat.is_empty() {
        return 1;
    } else if let Some(r) = cache.get(pat) {
        return *r;
    }
    let mut cnt = 0;
    for i in 1..=std::cmp::min(8, pat.len()) {
        if *twls.get_unchecked(slc_to_idx(&pat[..i])) {
            cnt += make_pattern_cnt(&pat[i..], twls, cache);
        }
    }
    cache.insert(pat, cnt);
    cnt
}
unsafe fn part2_inner(inp: &[u8]) -> u64 {
    let (towels, patterns) = read_towels(inp);
    let mut cache: FxHashMap<&[u8], u64> = FxHashMap::default();
    cache.reserve(20000);
    let mut cnt = 0;
    for pat in patterns {
        cnt += make_pattern_cnt(pat, &towels, &mut cache);
    }
    cnt
}

#[aoc(day19, part1)]
pub fn part1(inp: &str) -> u32 {
    unsafe { part1_inner(inp.as_bytes()) }
}

#[aoc(day19, part2)]
pub fn part2(inp: &str) -> u64 {
    unsafe { part2_inner(inp.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use super::part1;

    const INP: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";
    #[test]
    fn part1_test() {
        assert_eq!(part1(INP), 6)
    }
}
