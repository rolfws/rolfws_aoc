use aoc_runner_derive::aoc;

const RNI:isize = 50;
const RLI:isize = 50;
const RN: usize = 50;
const RL: usize = 50;

// const RNI:isize = 12;
// const RLI:isize = 12;
// const RN: usize = 12;
// const RL: usize = 12;

unsafe fn read_antenna_map(inp: &[u8]) -> ([[(isize, isize);5]; 80], [usize;80]) {
    let mut inp_map: [[(isize, isize);5]; 80] = [Default::default(); 80];
    let mut lens = [0usize;80];
    let inp_ptr = inp.as_ptr();
    let len_ptr = lens.as_mut_ptr();
    for i in 0..RNI {
        for j in 0..RLI {
            let c = inp_ptr.offset(i * (RNI + 1) + j).read();
            if c != b'.' {
                let l = len_ptr.add((c - b'0') as usize);
                *inp_map.get_unchecked_mut((c - b'0') as usize).get_unchecked_mut(*l) = (i, j);
                *l += 1;
            }
        }
    }

    (inp_map, lens)
}


unsafe fn part1_inner(inp:&[u8]) -> u32 {
    let (inp_map, lens) = read_antenna_map(inp);
    let mut node_map = [false; RN * RL];
    let nd_ptr = node_map.as_mut_ptr();
    let mut node_count: u32 = 0;
    for (i, l) in lens.into_iter().enumerate().filter(|(_,l)| *l > 0 ){
        let v = inp_map.get_unchecked(i);
        for i in 0..l-1 {
            for j in i + 1..l {
                let left = v.get_unchecked(i);
                let right = v.get_unchecked(j);
                let distance = (right.0 - left.0, right.1 - left.1);
                let node1 = (right.0 + distance.0, right.1 + distance.1);
                match node1 {
                    (0..RNI, 0..RLI) if !nd_ptr.offset(node1.0 * RLI + node1.1).read() => {
                        nd_ptr.offset(node1.0 * RLI + node1.1).write(true);
                        node_count += 1;
                    }
                    _ => {}
                }

                let node2 = (left.0 - distance.0, left.1 - distance.1);
                match node2 {
                    (0..RNI, 0..RLI) if !nd_ptr.offset(node2.0 * RLI + node2.1).read() => {
                        nd_ptr.offset(node2.0 * RLI + node2.1).write(true);
                        node_count += 1;
                    }
                    _ => {}
                }
            }
        }
    }
    node_count
}

unsafe fn part2_inner(inp:&[u8]) -> u32 {
    let (inp_map, lens) = read_antenna_map(inp);
    let mut node_map = [false; RN * RL];
    let nd_ptr = node_map.as_mut_ptr();
    let mut node_count: u32 = 0;
    for (i, l) in lens.into_iter().enumerate().filter(|(_,l)| *l > 0 ){
        let v = inp_map.get_unchecked(i);
        for i in 0..l-1 {
            for j in i + 1..l {
                let mut left = *v.get_unchecked(i);
                let mut right = *v.get_unchecked(j);
                let distance = (right.0 - left.0, right.1 - left.1);
                while let (0..RNI, 0..RLI) = right {
                    if !nd_ptr.offset(right.0 * RLI + right.1).read() {
                        nd_ptr.offset(right.0 * RLI + right.1).write(true);
                        node_count += 1;
                    }
                    right.0 += distance.0;
                    right.1 += distance.1;
                }

                while let (0..RNI, 0..RLI) = left {
                    if !nd_ptr.offset(left.0 * RLI + left.1).read() {
                        nd_ptr.offset(left.0 * RLI + left.1).write(true);
                        node_count += 1;
                    }
                    left.0 -= distance.0;
                    left.1 -= distance.1;
                }
            }
        }
    }

    node_count
}

#[aoc(day8, part1)]
pub fn part1(inp: &str) -> u32 {
    unsafe { part1_inner(inp.as_bytes()) }
}

#[aoc(day8, part2)]
pub fn part2(inp: &str) -> u32 {
    unsafe { part2_inner(inp.as_bytes()) }
}


#[cfg(test)]
mod tests {
    use crate::day8::{part1, part2};

    #[test]
    fn test_part1() {
        let inp = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
        assert_eq!(14,part1(inp))
    }

    #[test]
    fn test_part2() {
        let inp = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
        assert_eq!(34,part2(inp))
    }
}