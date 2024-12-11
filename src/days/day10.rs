// #![allow(unused)]s

use aoc_runner_derive::aoc;
#[cfg(test)]
const SIZE: usize = 8;

#[cfg(not(test))]
const SIZE: usize = 40;

unsafe fn load_data(inp: &[u8]) -> ([u8; SIZE * SIZE], Vec<usize>) {
    let ptr = inp.as_ptr();
    let offset = u64::from_ne_bytes([b'0'; 8]);
    let mut out = [0; SIZE * SIZE];
    let mut start = Vec::<usize>::new();
    let out_ptr = out.as_mut_ptr();
    for r in 0..SIZE {
        for b in 0..SIZE / 8 {
            let block = (ptr.add(r * (SIZE + 1) + b * 8) as *const u64).read_unaligned();
            let parsed = (block - offset).to_ne_bytes();
            out_ptr
                .add(r * SIZE + b * 8)
                .copy_from_nonoverlapping(parsed.as_ptr(), 8);
            for (i, _) in parsed.into_iter().enumerate().filter(|(_, h)| *h == 0) {
                start.push(r * SIZE + b * 8 + i)
            }
        }
    }
    (out, start)
}

unsafe fn part_1_step(cur_pos: usize, cur_h: u8, hmap: &[u8], vmap: &mut [bool]) {
    if cur_pos > SIZE && *hmap.get_unchecked(cur_pos - SIZE) == cur_h + 1 {
        if cur_h == 8 {
            *vmap.get_unchecked_mut(cur_pos - SIZE) = true;
        } else {
            part_1_step(cur_pos - SIZE, cur_h + 1, hmap, vmap);
        }
    }

    if cur_pos < SIZE * (SIZE - 1) && *hmap.get_unchecked(cur_pos + SIZE) == cur_h + 1 {
        if cur_h == 8 {
            *vmap.get_unchecked_mut(cur_pos + SIZE) = true;
        } else {
            part_1_step(cur_pos + SIZE, cur_h + 1, hmap, vmap);
        }
    }

    if cur_pos % SIZE > 0 && *hmap.get_unchecked(cur_pos - 1) == cur_h + 1 {
        if cur_h == 8 {
            *vmap.get_unchecked_mut(cur_pos - 1) = true;
        } else {
            part_1_step(cur_pos - 1, cur_h + 1, hmap, vmap);
        }
    }

    if cur_pos % SIZE < SIZE - 1 && *hmap.get_unchecked(cur_pos + 1) == cur_h + 1 {
        if cur_h == 8 {
            *vmap.get_unchecked_mut(cur_pos + 1) = true;
        } else {
            part_1_step(cur_pos + 1, cur_h + 1, hmap, vmap);
        }
    }
}

unsafe fn part1_inner(inp: &[u8]) -> usize {
    let (hmap, starts) = load_data(inp);
    let nine_inds: Vec<usize> = hmap
        .iter()
        .enumerate()
        .filter_map(|(i, h)| if *h == 9 { Some(i) } else { None })
        .collect();
    let mut tot_cnt = 0usize;
    let mut v_map = [false; SIZE * SIZE];
    for s in starts {
        part_1_step(s, 0, &hmap, &mut v_map);
        tot_cnt += nine_inds.iter().fold(0, |acc, &ind| {
            if *v_map.get_unchecked(ind) {
                *v_map.get_unchecked_mut(ind) = false;
                acc + 1
            } else {
                acc
            }
        })
    }
    tot_cnt
}

unsafe fn part_2_step(cur_pos: usize, cur_h: u8, hmap: &[u8], vmap: &mut [(bool, u16)]) -> u16 {
    if let (true, a) = *vmap.get_unchecked(cur_pos) {
        return a;
    }
    let mut cnt = 0u16;
    if cur_pos > SIZE && *hmap.get_unchecked(cur_pos - SIZE) == cur_h + 1 {
        if cur_h == 8 {
            cnt += 1;
        } else {
            cnt += part_2_step(cur_pos - SIZE, cur_h + 1, hmap, vmap)
        }
    }

    if cur_pos < SIZE * (SIZE - 1) && *hmap.get_unchecked(cur_pos + SIZE) == cur_h + 1 {
        if cur_h == 8 {
            cnt += 1;
        } else {
            cnt += part_2_step(cur_pos + SIZE, cur_h + 1, hmap, vmap)
        }
    }

    if cur_pos % SIZE > 0 && *hmap.get_unchecked(cur_pos - 1) == cur_h + 1 {
        if cur_h == 8 {
            cnt += 1;
        } else {
            cnt += part_2_step(cur_pos - 1, cur_h + 1, hmap, vmap)
        }
    }

    if cur_pos % SIZE < SIZE - 1 && *hmap.get_unchecked(cur_pos + 1) == cur_h + 1 {
        if cur_h == 8 {
            cnt += 1;
        } else {
            cnt += part_2_step(cur_pos + 1, cur_h + 1, hmap, vmap)
        }
    }

    *vmap.get_unchecked_mut(cur_pos) = (true, cnt);

    cnt
}

unsafe fn part2_inner(inp: &[u8]) -> u16 {
    let (hmap, starts) = load_data(inp);
    let mut tot_cnt = 0u16;
    let mut vmap = [(false, 0u16); SIZE * SIZE];
    // // part_1_step(1479, 0, &hmap);
    for s in starts {
        tot_cnt += part_2_step(s, 0, &hmap, &mut vmap);
    }
    tot_cnt
}

#[aoc(day10, part1)]
pub fn part1(inp: &str) -> usize {
    unsafe { part1_inner(inp.as_bytes()) }
}

#[aoc(day10, part2)]
pub fn part2(inp: &str) -> u16 {
    unsafe { part2_inner(inp.as_bytes()) }
}

// #[cfg(test)]
// mod tests {
//     use super::{part1, part2};

//     #[test]
//     fn part1_test() {
//         let inp = "89010123
// 78121874
// 87430965
// 96549874
// 45678903
// 32019012
// 01329801
// 10456732
// ";
//         assert_eq!(part1(inp), 36);
//     }

//     #[test]
//     fn part2_test() {
//         let inp = "89010123
// 78121874
// 87430965
// 96549874
// 45678903
// 32019012
// 01329801
// 10456732
// ";
//         assert_eq!(part2(inp), 81);
//     }
// }
