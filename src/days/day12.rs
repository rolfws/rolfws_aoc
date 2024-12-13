#![allow(unused)]

use aoc_runner_derive::aoc;
use fxhash::FxHashSet;

#[cfg(test)]
const SL: usize = 5;

#[cfg(not(test))]
const SL: usize = 140;

unsafe fn part1_inner(inp: &[u8]) -> u32 {
    let mut visited = [false; SL * SL];
    let mut needle: u8;
    let mut tot: u32 = 0;
    let mut que = FxHashSet::<usize>::default();
    let mut fence: u32;
    let mut area: u32;
    que.reserve(SL * 2);
    for i in 0..SL * SL {
        if *visited.get_unchecked(i) {
            continue;
        }
        needle = *inp.get_unchecked(i / SL * (SL + 1) + i % SL); // \n
        que.insert(i);
        fence = 0;
        area = 0;

        while !que.is_empty() {
            let j = *que.iter().next().expect("not empty");
            *visited.get_unchecked_mut(j) = true;
            area += 1;
            que.remove(&j);

            if j >= SL {
                if *inp.get_unchecked((j - SL) / SL * (SL + 1) + (j - SL) % SL) == needle {
                    if !*visited.get_unchecked(j - SL) {
                        que.insert(j - SL);
                    };
                } else {
                    fence += 1; // Other needle above
                }
            } else {
                fence += 1; // Top row fence
            }

            if j < SL * (SL - 1) {
                if *inp.get_unchecked((j + SL) / SL * (SL + 1) + (j + SL) % SL) == needle {
                    if !*visited.get_unchecked(j + SL) {
                        que.insert(j + SL);
                    };
                } else {
                    fence += 1; // Other needle below, so fence
                }
            } else {
                fence += 1; // Bottom row, so bottom fence
            }

            if j % SL >= 1 {
                if *inp.get_unchecked((j - 1) / SL * (SL + 1) + (j - 1) % SL) == needle {
                    if !*visited.get_unchecked(j - 1) {
                        que.insert(j - 1);
                    };
                } else {
                    fence += 1; // Other needle left, so fence
                }
            } else {
                fence += 1; // Left column
            }

            if j % SL < SL - 1 {
                if *inp.get_unchecked((j + 1) / SL * (SL + 1) + (j + 1) % SL) == needle {
                    if !*visited.get_unchecked(j + 1) {
                        que.insert(j + 1);
                    };
                } else {
                    fence += 1; // Other needle right, so fence
                }
            } else {
                fence += 1; // right column
            }
        }
        tot += fence * area;
        // break;
    }
    tot
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Dir {
    U,
    D,
    R,
    L,
}

impl Dir {
    fn rotate_right(&self) -> Dir {
        match self {
            Dir::U => Dir::R,
            Dir::D => Dir::L,
            Dir::R => Dir::D,
            Dir::L => Dir::U,
        }
    }

    fn rotate_left(&self) -> Dir {
        match self {
            Dir::U => Dir::L,
            Dir::D => Dir::R,
            Dir::R => Dir::U,
            Dir::L => Dir::D,
        }
    }

    fn check_rside(&self, i: usize) -> Option<usize> {
        match self {
            Dir::U => {
                if i % SL < SL - 1 {
                    Some(i + 1)
                } else {
                    None
                }
            }
            Dir::D => {
                if i % SL > 0 {
                    Some(i - 1)
                } else {
                    None
                }
            }
            Dir::R => {
                if i < SL * (SL - 1) {
                    Some(i + SL)
                } else {
                    None
                }
            }
            Dir::L => {
                if i >= SL {
                    Some(i - SL)
                } else {
                    None
                }
            }
        }
    }

    fn check_front(&self, i: usize) -> Option<usize> {
        match self {
            Dir::R => {
                if i % SL < SL - 1 {
                    Some(i + 1)
                } else {
                    None
                }
            }
            Dir::L => {
                if i % SL > 0 {
                    Some(i - 1)
                } else {
                    None
                }
            }
            Dir::D => {
                if i < SL * (SL - 1) {
                    Some(i + SL)
                } else {
                    None
                }
            }
            Dir::U => {
                if i >= SL {
                    Some(i - SL)
                } else {
                    None
                }
            }
        }
    }

    fn corner_offset(&self, other: &Self) -> (usize, usize) {
        match (self, other) {
            (Dir::D, Dir::L) => (0, 0),
            (Dir::D, Dir::R) => (1, 0),

            (Dir::U, Dir::L) => (0, 1),
            (Dir::U, Dir::R) => (1, 1),

            (Dir::R, Dir::U) => (1, 1),
            (Dir::R, Dir::D) => (1, 0),

            (Dir::L, Dir::U) => (0, 1),
            (Dir::L, Dir::D) => (0, 0),
            _ => unreachable!("Not possible combination"),
        }
    }
}

// unsafe fn part2_edge_walk(inp: &[u8], start: usize) -> u32 {
    // let mut vertices: Vec<(usize, usize)> = Vec::with_capacity(100);
    // let needle = *inp.get_unchecked(start / SL * (SL + 1) + start % SL); // \n
    // let mut dir = Dir::D; // We always start at the most top left element
    // vertices.clear();
    // vertices.push((start / SL, start % SL));
    // let mut cur = start;
    // let mut k = 0;
    // while cur != start || dir != Dir::L {
    //     if let Some(j) = dir.check_rside(cur) {
    //         if *inp.get_unchecked(j / SL * (SL + 1) + j % SL) == needle {
    //             let n_dir = dir.rotate_right();
    //             let off = dir.corner_offset(&n_dir);
    //             vertices.push((cur / SL + off.0, cur % SL + off.1));
    //             cur = j;
    //             dir = n_dir;
    //             continue;
    //         }
    //     }
    //     if let Some(j) = dir.check_front(cur) {
    //         if *inp.get_unchecked(j / SL * (SL + 1) + j % SL) == needle {
    //             cur = j;
    //             continue;
    //         }
    //     }

    //     let n_dir = dir.rotate_left();
    //     let off = dir.corner_offset(&n_dir);
    //     vertices.push((cur / SL + off.0, cur % SL + off.1));
    //     dir = n_dir;
    // }
    // vertices.len() as u32
// }

unsafe fn part2_inner(inp: &[u8]) -> u32 {
    // let mut line_map 
    0
}

#[aoc(day12, part1)]
pub fn part1(inp: &str) -> u32 {
    unsafe { part1_inner(inp.as_bytes()) }
}

#[aoc(day12, part2)]
pub fn part2(inp: &str) -> u32 {
    unsafe { part2_inner(inp.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    #[test]
    fn part1_test() {
        let inp = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";
        assert_eq!(part1(inp), 1930)
    }

    #[test]
    fn part2_test() {
        let inp = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";
        assert_eq!(part2(inp), 1206);
        //         let inp = "RRRRIICCFF
        // RRRRIICCCF
        // VVRRRCCFFF
        // VVRCCCJFFF
        // VVVVCJJCFE
        // VVIVCCJJEE
        // VVIIICJJEE
        // MIIIIIJJEE
        // MIIISIJEEE
        // MMMISSJEEE
        // ";
        //         assert_eq!(part2(inp), 1206)
    }
}
