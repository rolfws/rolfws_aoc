// #![allow(unused)]

use aoc_runner_derive::aoc;
use fxhash::FxHashSet;

#[cfg(test)]
const SL: usize = 10;

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
    fn map_offset(&self) -> usize {
        match self {
            Dir::U => 0,
            Dir::R => 1,
            Dir::D => 2,
            Dir::L => 3,
        }
    }
}

unsafe fn part2_inner(inp: &[u8]) -> u32 {
    let mut line_map = [[0u16; 4]; SL * SL];
    let mut line_ind = 0;
    // First line and last line only effect first and last row
    let mut prev: u8;
    for r in 0..SL {
        prev = 0u8;
        for c in 0..SL {
            // top lines
            if *inp.get_unchecked(r * (SL + 1) + c) != prev {
                line_ind += 1;
            }
            prev = *inp.get_unchecked(r * (SL + 1) + c);
            if r == 0 || prev != *inp.get_unchecked((r - 1) * (SL + 1) + c) {
                *line_map
                    .get_unchecked_mut(r * SL + c)
                    .get_unchecked_mut(Dir::U.map_offset()) = line_ind;
            } else {
                line_ind += 1;
            }
        }
        // bottom lines
        prev = 0u8;
        for c in 0..SL {
            // top lines
            if *inp.get_unchecked(r * (SL + 1) + c) != prev {
                line_ind += 1;
            }
            prev = *inp.get_unchecked(r * (SL + 1) + c);
            if r == SL - 1 || prev != *inp.get_unchecked((r + 1) * (SL + 1) + c) {
                *line_map
                    .get_unchecked_mut(r * SL + c)
                    .get_unchecked_mut(Dir::D.map_offset()) = line_ind;
            } else {
                line_ind += 1;
            }
        }
    }

    for c in 0..SL {
        prev = 0u8;
        for r in 0..SL {
            // top lines
            if *inp.get_unchecked(r * (SL + 1) + c) != prev {
                line_ind += 1;
            }
            prev = *inp.get_unchecked(r * (SL + 1) + c);
            if c == 0 || prev != *inp.get_unchecked(r * (SL + 1) + c - 1) {
                *line_map
                    .get_unchecked_mut(r * SL + c)
                    .get_unchecked_mut(Dir::L.map_offset()) = line_ind;
            } else {
                line_ind += 1;
            }
        }

        prev = 0u8;
        for r in 0..SL {
            // top lines
            if *inp.get_unchecked(r * (SL + 1) + c) != prev {
                line_ind += 1;
            }
            prev = *inp.get_unchecked(r * (SL + 1) + c);
            if c == SL - 1 || prev != *inp.get_unchecked(r * (SL + 1) + c + 1) {
                *line_map
                    .get_unchecked_mut(r * SL + c)
                    .get_unchecked_mut(Dir::R.map_offset()) = line_ind;
            }else {
                line_ind += 1;
            }
        }
    }

    let mut visited = [false; SL * SL];
    let mut needle: u8;
    let mut tot: u32 = 0;
    let mut que = FxHashSet::<usize>::default();
    que.reserve(SL * 2);

    let mut sides = FxHashSet::<u16>::default();
    sides.reserve(150);

    let mut area: u32;

    for i in 0..SL * SL {
        if *visited.get_unchecked(i) {
            continue;
        }
        needle = *inp.get_unchecked(i / SL * (SL + 1) + i % SL); // \n
        que.insert(i);
        sides.clear();
        area = 0;

        while !que.is_empty() {
            let j = *que.iter().next().expect("not empty");
            *visited.get_unchecked_mut(j) = true;
            area += 1;
            for v in line_map[j] {
                if v != 0 {
                    sides.insert(v);
                }
            }
            que.remove(&j);

            if j >= SL
                && *inp.get_unchecked((j - SL) / SL * (SL + 1) + (j - SL) % SL) == needle
                && !*visited.get_unchecked(j - SL)
            {
                que.insert(j - SL);
            };

            if j < SL * (SL - 1)
                && *inp.get_unchecked((j + SL) / SL * (SL + 1) + (j + SL) % SL) == needle
                && !*visited.get_unchecked(j + SL)
            {
                que.insert(j + SL);
            };

            if j % SL >= 1
                && *inp.get_unchecked((j - 1) / SL * (SL + 1) + (j - 1) % SL) == needle
                && !*visited.get_unchecked(j - 1)
            {
                que.insert(j - 1);
            };

            if j % SL < SL - 1
                && *inp.get_unchecked((j + 1) / SL * (SL + 1) + (j + 1) % SL) == needle
                && !*visited.get_unchecked(j + 1)
            {
                que.insert(j + 1);
            };
        }
        tot += sides.len() as u32 * area;
        // break;
    }
    tot
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
        let inp = "
RRRRIICCFF
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
        assert_eq!(part2(inp), 1206)
    }
}
