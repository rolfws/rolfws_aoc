
use aoc_runner_derive::aoc;
use fxhash::FxHashSet;

#[cfg(test)]
const S: usize = 10;
#[cfg(not(test))]
const S: usize = 50;

unsafe fn part1_inner(inp: &[u8]) -> usize {
    let mut walls = [false; S * S];
    let mut boxs = [false; S * S];
    let mut cur = 0usize;
    walls[0..S].iter_mut().for_each(|c| *c = true);
    walls[S * (S - 1)..].iter_mut().for_each(|c| *c = true);
    walls.iter_mut().step_by(S).for_each(|c| *c = true);
    walls[S - 1..].iter_mut().step_by(S).for_each(|c| *c = true);
    for r in 1..S - 1 {
        for c in 1..S - 1 {
            match inp.get_unchecked(r * (S + 1) + c) {
                b'#' => *walls.get_unchecked_mut(r * S + c) = true,
                b'O' => *boxs.get_unchecked_mut(r * S + c) = true,
                b'@' => cur = r * S + c,
                _ => {}
            }
        }
    }

    for &step in inp[S * (S + 1) + 1..].iter() {
        match step {
            b'<' => {
                if let Some(p) = (1..cur % S)
                    .take_while(|o| !walls.get_unchecked(cur - o))
                    .find(|o| !boxs.get_unchecked(cur - o))
                {
                    // All boxes can be moved to p
                    *boxs.get_unchecked_mut(cur - p) = true;
                    *boxs.get_unchecked_mut(cur - 1) = false;
                    cur -= 1;
                }
            }
            b'>' => {
                if let Some(p) = (1..S - cur % S)
                    .take_while(|o| !walls.get_unchecked(cur + o))
                    .find(|o| !boxs.get_unchecked(cur + o))
                {
                    // All boxes can be moved to p
                    *boxs.get_unchecked_mut(cur + p) = true;
                    *boxs.get_unchecked_mut(cur + 1) = false;
                    cur += 1;
                }
            }
            b'v' => {
                if let Some(p) = (1..S - cur / S)
                    .take_while(|o| !walls.get_unchecked(cur + o * S))
                    .find(|o| !boxs.get_unchecked(cur + o * S))
                {
                    // All boxes can be moved to p
                    *boxs.get_unchecked_mut(cur + p * S) = true;
                    *boxs.get_unchecked_mut(cur + S) = false;
                    cur += S;
                }
            }
            b'^' => {
                if let Some(p) = (1..cur / S)
                    .take_while(|o| !walls.get_unchecked(cur - o * S))
                    .find(|o| !boxs.get_unchecked(cur - o * S))
                {
                    // All boxes can be moved to p
                    *boxs.get_unchecked_mut(cur - p * S) = true;
                    *boxs.get_unchecked_mut(cur - S) = false;
                    cur -= S;
                }
            }
            _ => {unreachable!()} // Inputs are limited to these 4 cases
        }
    }
    boxs.into_iter().enumerate().fold(
        0,
        |acc, (i, b)| if b { acc + i / S * 100 + i % S } else { acc },
    )
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum BoxPart {
    L,
    R,
    #[default]
    N,
}

impl BoxPart {
    fn flip(&mut self) {
        match self {
            BoxPart::L => *self = BoxPart::R,
            BoxPart::R => *self = BoxPart::L,
            BoxPart::N => {}
        }
    }
}

unsafe fn part2_inner(inp: &[u8]) -> usize {
    let mut walls = [false; S * S * 2];
    let mut boxs = [BoxPart::default(); S * S * 2];
    let mut cur = 0usize;
    walls[0..2 * S].iter_mut().for_each(|c| *c = true);
    walls[2 * S * (S - 1)..].iter_mut().for_each(|c| *c = true);
    walls[1..].iter_mut().step_by(2 * S).for_each(|c| *c = true); // Only care about the inner walls
    walls[2 * S - 2..]
        .iter_mut()
        .step_by(2 * S)
        .for_each(|c| *c = true);
    for r in 1..S - 1 {
        for c in 1..S - 1 {
            match inp.get_unchecked(r * (S + 1) + c) {
                b'#' => {
                    *walls.get_unchecked_mut(r * S * 2 + c * 2) = true;
                    *walls.get_unchecked_mut(r * S * 2 + c * 2 + 1) = true
                }
                b'O' => {
                    *boxs.get_unchecked_mut(r * S * 2 + c * 2) = BoxPart::L;
                    *boxs.get_unchecked_mut(r * S * 2 + c * 2 + 1) = BoxPart::R;
                }
                b'@' => cur = r * S * 2 + c * 2,
                _ => {}
            }
        }
    }

    let mut indbuffer: [FxHashSet<usize>; 15] = core::array::from_fn(|_| {
        FxHashSet::with_capacity_and_hasher(30, fxhash::FxBuildHasher::default())
    });
    for &step in inp[S * (S + 1) + 1..].iter() {
        // println!("{}")
        match step {
            b'<' => {
                // println!("left");
                if let Some(p) = (1..cur % (2 * S))
                    .step_by(2) // If there is a right side it is followed by a left side. So we skip that one
                    .take_while(|o| !walls.get_unchecked(cur - o))
                    .find(|o| *boxs.get_unchecked(cur - o) == BoxPart::N)
                {
                    *boxs.get_unchecked_mut(cur - p) = BoxPart::L;
                    for o in 2..p {
                        boxs.get_unchecked_mut(cur - o).flip();
                    }
                    *boxs.get_unchecked_mut(cur - 1) = BoxPart::N;
                    cur -= 1;
                }
            }
            b'>' => {
                // println!("right");
                if let Some(p) = (1..(2 * S) - cur % (2 * S))
                    .step_by(2) // if there is a left side then it is followed by a right side. So no wall or empty slot anyway
                    .take_while(|o| !walls.get_unchecked(cur + o))
                    .find(|o| *boxs.get_unchecked(cur + o) == BoxPart::N)
                {
                    // All boxes can be moved to p'
                    *boxs.get_unchecked_mut(cur + p) = BoxPart::R;
                    for o in 2..p {
                        boxs.get_unchecked_mut(cur + o).flip();
                    }
                    *boxs.get_unchecked_mut(cur + 1) = BoxPart::N;
                    cur += 1;
                }
            }
            b'v' => {
                // println!("down");
                let mut wallblocked = false;
                let mut stack_height = 0;
                indbuffer[0].insert(cur);
                loop {
                    let (splitl, splitr) = indbuffer.split_at_mut(stack_height + 1);
                    for i in splitl[stack_height].iter() {
                        if *walls.get_unchecked(i + 2 * S) {
                            wallblocked = true;
                            splitr[0].clear();
                            break;
                        }
                        match *boxs.get_unchecked(i + 2 * S) {
                            BoxPart::L => {
                                splitr[0].extend([i + 2 * S, i + 2 * S + 1]);
                            }
                            BoxPart::R => {
                                splitr[0].extend([i + 2 * S, i + 2 * S - 1]);
                            }
                            BoxPart::N => {}
                        }
                    }
                    if splitr[0].is_empty() {
                        break;
                    }
                    stack_height += 1;
                }
                for inds in indbuffer[1..=stack_height].iter_mut().rev() {
                    if !wallblocked {
                        // Here we know the row below is fully empty, so we move each element down
                        for &i in inds.iter() {
                            *boxs.get_unchecked_mut(i + 2 * S) = *boxs.get_unchecked(i);
                            *boxs.get_unchecked_mut(i) = BoxPart::N;
                        }
                    }
                    inds.clear();
                }
                indbuffer[0].clear();
                if !wallblocked {
                    cur += 2 * S;
                }
            }
            b'^' => {
                // println!("up");
                let mut wallblocked = false;
                let mut stack_height = 0;
                indbuffer[0].insert(cur);
                loop {
                    let (splitl, splitr) = indbuffer.split_at_mut(stack_height + 1);
                    for i in splitl[stack_height].iter() {
                        if *walls.get_unchecked(i - 2 * S) {
                            wallblocked = true;
                            splitr[0].clear();
                            break;
                        }
                        match *boxs.get_unchecked(i - 2 * S) {
                            BoxPart::L => {
                                splitr[0].extend([i - 2 * S, i - 2 * S + 1]);
                            }
                            BoxPart::R => {
                                splitr[0].extend([i - 2 * S, i - 2 * S - 1]);
                            }
                            BoxPart::N => {}
                        }
                    }
                    if splitr[0].is_empty() {
                        break;
                    }
                    stack_height += 1;
                }
                for inds in indbuffer[1..=stack_height].iter_mut().rev() {
                    if !wallblocked {
                        // Here we know the row below is fully empty, so we move each element down
                        for &i in inds.iter() {
                            *boxs.get_unchecked_mut(i - 2 * S) = *boxs.get_unchecked(i);
                            *boxs.get_unchecked_mut(i) = BoxPart::N;
                        }
                    }
                    inds.clear();
                }
                indbuffer[0].clear();
                if !wallblocked {
                    cur -= 2 * S;
                }
            }
            _ => {unreachable!()}
        }
    }
    boxs.into_iter().enumerate().fold(0, |acc, (i, b)| {
        if b == BoxPart::L {
            acc + i / (S * 2) * 100 + i % (S * 2)
        } else {
            acc
        }
    })
}

#[aoc(day15, part1)]
pub fn part1(inp: &str) -> usize {
    unsafe { part1_inner(inp.as_bytes()) }
}

#[aoc(day15, part2)]
pub fn part2(inp: &str) -> usize {
    unsafe { part2_inner(inp.as_bytes()) }
}

// #[cfg(test)]
// mod tests {
//     use super::{part1, part2};

//     #[test]
//     fn part1_test() {
//         let inp = "##########
// #..O..O.O#
// #......O.#
// #.OO..O.O#
// #..O@..O.#
// #O#..O...#
// #O..O..O.#
// #.OO.O.OO#
// #....O...#
// ##########

// <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
// vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
// ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
// <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
// ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
// ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
// >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
// <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
// ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
// v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
// ";
//         assert_eq!(part1(inp), 10092)
//     }

//     #[test]
//     fn part2_test() {
//         let inp = "##########
// #..O..O.O#
// #......O.#
// #.OO..O.O#
// #..O@..O.#
// #O#..O...#
// #O..O..O.#
// #.OO.O.OO#
// #....O...#
// ##########

// <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
// vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
// ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
// <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
// ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
// ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
// >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
// <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
// ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
// v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
// ";
//         assert_eq!(part2(inp), 9021)
//     }
// }
