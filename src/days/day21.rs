#![allow(unused)]

use aoc_runner_derive::aoc;
use fxhash::FxHashMap;
use memchr::memchr_iter;


unsafe fn parse_u8(ps:&[u8]) -> usize {
    let mut out = 0usize;
    for b in ps {
        out *= 10;
        out += (b - b'0') as usize
    }
    out
}

unsafe fn match_num_dir(from: u8, to:u8, steps: (i8, i8)) -> Vec<u8> {
    let mut r_part = if steps.0 >= 0 {
        vec![b'v'; steps.0 as usize]
    } else {
        vec![b'^'; (-steps.0) as usize]
    };
    let mut c_part = if steps.1 >= 0 {
        vec![b'>'; steps.1 as usize]
    } else {
        vec![b'<'; (-steps.1) as usize]
    };
    match (from, to) {
        (b'0', b'1') | (b'0', b'4') | (b'0', b'7') | (b'A', b'1') | (b'A', b'4') | (b'A', b'7') => {
            r_part.extend(c_part);
            r_part.push(b'A');
            r_part
        }
        (b'7', b'0') | (b'7', b'A') | (b'4', b'0') | (b'4', b'A') | (b'1', b'0') | (b'1', b'A') => {
            c_part.extend(r_part);
            c_part.push(b'A');
            c_part
        }
        _ if steps.1 < 0 => {
            c_part.extend(r_part);
            c_part.push(b'A');
            c_part
        }
        _ => {
            r_part.extend(c_part);
            r_part.push(b'A');
            r_part 
        }
    }
}


#[inline(always)]
fn match_num_crd(b:u8) -> (i8, i8) {
    match b {
        b'0' => {(3,1)},
        b'1' => {(2,0)},
        b'2' => {(2,1)},
        b'3' => {(2,2)},
        b'4' => {(1,0)},
        b'5' => {(1,1)},
        b'6' => {(1,2)},
        b'7' => {(0,0)},
        b'8' => {(0,1)},
        b'9' => {(0,2)},
        b'A' => {(3,2)},
        _ => unreachable!("faulty input")
    }
}

fn match_dir_crd(b:u8) -> (i8, i8) {
    match b {
        b'^' => {(0,1)},
        b'A' => {(0,2)},
        b'v' => {(1,1)},
        b'<' => {(1,0)},
        b'>' => {(1,2)},
        _ => unreachable!("faulty input")
    }
}

unsafe fn match_dir_dir(from: u8, to:u8, steps: (i8, i8)) -> Vec<u8> {
    let mut r_part = if steps.0 >= 0 {
        vec![b'v'; steps.0 as usize]
    } else {
        vec![b'^'; (-steps.0) as usize]
    };
    let mut c_part = if steps.1 >= 0 {
        vec![b'>'; steps.1 as usize]
    } else {
        vec![b'<'; (-steps.1) as usize]
    };
    match (from, to) {
        (b'<', b'^') | (b'<', b'A')  => {
            c_part.extend(r_part);
            c_part.push(b'A');
            c_part
        }
        (b'^', b'<') | (b'A', b'<') => {
            r_part.extend(c_part);
            r_part.push(b'A');
            r_part 
        }
        _ if steps.1 < 0 => {
            c_part.extend(r_part);
            c_part.push(b'A');
            c_part
        }
        _ => {
            r_part.extend(c_part);
            r_part.push(b'A');
            r_part 
        }
    }
}

unsafe fn instructions(inp:&[u8]) -> usize {
    let mut cur = (3, 2);
    let mut cur_b = b'A';
    let mut cnts:FxHashMap<Vec<u8>, usize> = FxHashMap::default();
    for &c in inp.iter() {
        let to = match_num_crd(c);
        let d = (to.0 - cur.0, to.1 - cur.1);
        // out.extend();
        let b = match_num_dir(cur_b, c, d);
        if let Some(v) = cnts.get_mut(&b) {
            *v += 1;
        } else {
            cnts.insert(b, 1);
        }
        cur_b = c;
        cur = to;
    }
    

    let mut cache: FxHashMap<Vec<u8>, Vec<u8>> = FxHashMap::default();
    for _ in 0..2 {
        let mut out_cnt:FxHashMap<Vec<u8>, usize> = FxHashMap::default();
        let mut prev = 0usize;
        for (p, c) in cnts.iter() {
            let ins = instructions_inner(p, &mut cache);
            let mut prev = 0usize;
            for n in memchr_iter(b'A', &ins) {
                if let Some(v) = out_cnt.get_mut(&ins[prev..=n]) {
                    *v += c;
                } else {
                    out_cnt.insert(ins[prev..=n].to_vec(), *c);
                }
                prev = n + 1;
            }
        }
        cnts = out_cnt;
    }
    cnts.into_iter().fold(0,|acc,(k,v)| acc + k.len() * v)
}

unsafe fn part1_inner(inp: &[u8]) -> usize {
    let mut cnt = 0usize;
    for i in 0..5 {
        // println!("{}, {}", ins.len(), parse_u8(&inp[i*5..i*5+3]));
        cnt += instructions(&inp[i*5..i*5+4]) * parse_u8(&inp[i*5..i*5+3]);
    }
    cnt
}

// A to A, So we chunk it up after initial
unsafe fn instructions_inner(inp:&[u8], cache: &mut FxHashMap<Vec<u8>, Vec<u8>>) -> Vec<u8> {
    if let Some(r) = cache.get(inp) {
        return r.clone()
    }
    let mut cur = (0, 2);
    let mut cur_b = b'A';
    let mut out: Vec<u8> = Vec::with_capacity(inp.len() * 2);
    for &c in inp.iter() {
        let to = match_dir_crd(c);
        let d = (to.0 - cur.0, to.1 - cur.1);
        out.extend(match_dir_dir(cur_b, c, d));
        cur_b = c;
        cur = to;
    }
    cache.insert(inp.to_vec(), out.clone());
    out
}

unsafe fn instructions2(inp:&[u8]) -> usize {
    let mut cur = (3, 2);
    let mut cur_b = b'A';
    let mut cnts:FxHashMap<Vec<u8>, usize> = FxHashMap::default();
    for &c in inp.iter() {
        let to = match_num_crd(c);
        let d = (to.0 - cur.0, to.1 - cur.1);
        // out.extend();
        let b = match_num_dir(cur_b, c, d);
        if let Some(v) = cnts.get_mut(&b) {
            *v += 1;
        } else {
            cnts.insert(b, 1);
        }
        cur_b = c;
        cur = to;
    }
    

    let mut cache: FxHashMap<Vec<u8>, Vec<u8>> = FxHashMap::default();
    for _ in 0..25 {
        let mut out_cnt:FxHashMap<Vec<u8>, usize> = FxHashMap::default();
        let mut prev = 0usize;
        for (p, c) in cnts.iter() {
            let ins = instructions_inner(p, &mut cache);
            let mut prev = 0usize;
            for n in memchr_iter(b'A', &ins) {
                if let Some(v) = out_cnt.get_mut(&ins[prev..=n]) {
                    *v += c;
                } else {
                    out_cnt.insert(ins[prev..=n].to_vec(), *c);
                }
                prev = n + 1;
            }
        }
        cnts = out_cnt;
    }
    cnts.into_iter().fold(0,|acc,(k,v)| acc + k.len() * v)
}

unsafe fn part2_inner(inp: &[u8]) -> usize {
    let mut cnt = 0usize;
    for i in 0..5 {
        // println!("{}, {}", ins.len(), parse_u8(&inp[i*5..i*5+3]));
        cnt += instructions2(&inp[i*5..i*5+4]) * parse_u8(&inp[i*5..i*5+3]);
    }
    cnt
}

#[aoc(day21, part1)]
pub fn part1(inp: &str) -> usize {
    unsafe { part1_inner(inp.as_bytes()) }
}

#[aoc(day21, part2)]
pub fn part2(inp: &str) -> usize {
    unsafe { part2_inner(inp.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use crate::day21::instructions;

    use super::{part1, part2};

    #[test]
    fn part1_test() {
        let inp = "029A
980A
179A
456A
379A
";
        assert_eq!(part1(inp), 126384)
    }
}
