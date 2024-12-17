#![allow(unused)]

use std::num::NonZero;

use aoc_runner_derive::aoc;
use fxhash::FxHashSet;

#[cfg(test)]
const S: usize = 15;
#[cfg(not(test))]
const S: usize = 141;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    fn offset(&self) -> usize {
        match self {
            Dir::N => 0,
            Dir::E => 1,
            Dir::S => 2,
            Dir::W => 3,
        }
    }

    fn frwdoff(&self, cur: usize) -> usize {
        match self {
            Dir::N => cur - S,
            Dir::E => cur + 1,
            Dir::S => cur + S,
            Dir::W => cur - 1,
        }
    }

    fn turnleft(&self) -> Self {
        match self {
            Dir::N => Dir::W,
            Dir::E => Dir::N,
            Dir::S => Dir::E,
            Dir::W => Dir::S,
        }
    }

    fn turnright(&self) -> Self {
        match self {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::S => Dir::W,
            Dir::W => Dir::N,
        }
    }

    fn leftoff(&self, cur: usize) -> usize {
        self.turnleft().frwdoff(cur)
    }

    fn rightoff(&self, cur: usize) -> usize {
        self.turnright().frwdoff(cur)
    }
}
unsafe fn part1_move(
    cur: usize,
    cur_dir: Dir,
    cur_cost: u32,
    visited: &mut [[u32; 4]; S * S],
    walls: &[bool; S * S],
) -> Option<NonZero<u32>> {
    // println!("{cur}, {cur_dir:?}");
    if cur == S * 2 - 2 {
        // println!("end");
        return Some(cur_cost.try_into().unwrap());
    };
    if *visited.get_unchecked(cur).get_unchecked(cur_dir.offset()) < cur_cost {
        // println!("useless");
        return None; // Been here before and it was a cheaper route, So this route is useless
    }
    // Maybe been here but this route was cheaper
    *visited
        .get_unchecked_mut(cur)
        .get_unchecked_mut(cur_dir.offset()) = cur_cost;
    // Now we go either forward, left or right
    // let fwrd = part1_move(cur, cur_dir, cur_cost, visited)

    let fwrd = cur_dir.frwdoff(cur);
    let fwrd_cost = if !*walls.get_unchecked(fwrd) {
        part1_move(fwrd, cur_dir, cur_cost + 1, visited, walls)
    } else {
        None
    };
   
    let left = cur_dir.leftoff(cur);
    let left_cost = if !*walls.get_unchecked(left) {
        part1_move(left, cur_dir.turnleft(), cur_cost + 1001, visited, walls)
    } else {
        None
    };

    let right = cur_dir.rightoff(cur);
    let right_cost = if !*walls.get_unchecked(right) {
        part1_move(right, cur_dir.turnright(), cur_cost + 1001, visited, walls)
    } else {
        None
    };

    if let Some(Some(r)) = [fwrd_cost, left_cost, right_cost]
        .into_iter()
        .min_by_key(|x| x.unwrap_or(NonZero::new(u32::MAX).unwrap())) 
    {
        Some(r)
    } else {
        None
    }
}

unsafe fn part1_inner(inp: &[u8]) -> u32 {
    let mut walls = [true; S * S];
    for r in 1..S -1 {
        for c in 1..S - 1 {
            if *inp.get_unchecked(r * (S + 1) + c) != b'#' {
                *walls.get_unchecked_mut(r * S + c) = false;
            }
        }
        // println!("{}", walls[r * S..(r+1) * S].iter().map(|b| if *b {'#'} else {'.'}).collect::<String>())
    }
    
    let mut cur =  S * (S - 2) + 1;
    let mut visited = [[u32::MAX; 4]; S * S];
    visited[ S * (S - 2) + 1] = [0,0,0,0];
    let mut cur_dir = Dir::E;
    part1_move(cur, cur_dir, 0, &mut visited, &walls).unwrap().into()
}

#[allow(clippy::comparison_chain)]
unsafe fn part2_move(
    cur: usize,
    cur_dir: Dir,
    cur_cost: u32,
    visited: &mut [[u32; 4]; S * S],
    walls: &[bool; S * S],
) -> Option<(u32, FxHashSet<usize>)> {
    // println!("{cur}, {cur_dir:?}");
    if cur == S * 2 - 2 {
        // println!("end");
        let mut path_set = FxHashSet::default();
        path_set.reserve(4 * S);
        path_set.insert(cur);
        return Some((cur_cost, path_set));
    };
    if *visited.get_unchecked(cur).get_unchecked(cur_dir.offset()) < cur_cost {
        // println!("useless");
        return None; // Been here before and it was a cheaper route, So this route is useless
    }
    // Maybe been here but this route was cheaper
    *visited
        .get_unchecked_mut(cur)
        .get_unchecked_mut(cur_dir.offset()) = cur_cost;
    // Now we go either forward, left or right
    // let fwrd = part1_move(cur, cur_dir, cur_cost, visited)

    let fwrd = cur_dir.frwdoff(cur);
    let mut min_cost = u32::MAX;
    let (mut min_cost, mut set) = if !*walls.get_unchecked(fwrd) {
        part2_move(fwrd, cur_dir, cur_cost + 1, visited, walls).unwrap_or((u32::MAX, FxHashSet::default()))
    } else {
        (u32::MAX, FxHashSet::default())
    };
   
    let left = cur_dir.leftoff(cur);
    if !*walls.get_unchecked(left) {
        if let Some((c, s)) = part2_move(left, cur_dir.turnleft(), cur_cost + 1001, visited, walls) {
            if c < min_cost {
                min_cost = c;
                set = s
            } else if  c == min_cost {
                set.extend(s);
            }
        }
    }

    let right = cur_dir.rightoff(cur);
    if !*walls.get_unchecked(right) {
        if let Some((c,s)) = part2_move(right, cur_dir.turnright(), cur_cost + 1001, visited, walls) {
            if c < min_cost {
                min_cost = c;
                set = s;
            } else if  c == min_cost {
                set.extend(s);
            }
        }
    }

    if min_cost < u32::MAX {
        set.insert(cur);
        Some((min_cost, set))
    } else {
        None
    }
}

unsafe fn part2_inner(inp: &[u8]) -> usize {
    let mut walls = [true; S * S];
    for r in 1..S -1 {
        for c in 1..S - 1 {
            if *inp.get_unchecked(r * (S + 1) + c) != b'#' {
                *walls.get_unchecked_mut(r * S + c) = false;
            }
        }
        // println!("{}", walls[r * S..(r+1) * S].iter().map(|b| if *b {'#'} else {'.'}).collect::<String>())
    }
    
    let mut cur =  S * (S - 2) + 1;
    let mut visited = [[u32::MAX; 4]; S * S];
    visited[ S * (S - 2) + 1] = [0,0,0,0];
    let mut cur_dir = Dir::E;
    let (l, s) = part2_move(cur, cur_dir, 0, &mut visited, &walls).unwrap();
    println!("{l}");
    s.len()
}

#[aoc(day16, part1)]
pub fn part1(inp: &str) -> u32 {
    unsafe { part1_inner(inp.as_bytes()) }
}

#[aoc(day16, part2)]
pub fn part2(inp: &str) -> usize {
    unsafe { part2_inner(inp.as_bytes()) }
}

// #[cfg(test)]
// mod tests {
//     use super::{part1, part2};


//     // #[test]
//     // fn part1_test() {
// //         let inp = "#######
// // #.#.#E#
// // #...#.#
// // #.#...#
// // #.#.#.#
// // #S#...#
// // #######
// // ";
//     //     assert_eq!(part1(inp), 7036)
//     // }
//     #[test]
//     fn part1_test() {
//         let inp = "###############
// #.......#....E#
// #.#.###.#.###.#
// #.....#.#...#.#
// #.###.#####.#.#
// #.#.#.......#.#
// #.#.#####.###.#
// #...........#.#
// ###.#.#####.#.#
// #...#.....#.#.#
// #.#.#.###.#.#.#
// #.....#...#.#.#
// #.###.#.#.#.#.#
// #S..#.....#...#
// ###############
// ";
//         assert_eq!(part1(inp), 7036)
//     }
// }
