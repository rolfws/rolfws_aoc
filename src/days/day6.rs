use aoc_runner_derive::aoc;
use memchr::{memchr, memchr_iter};

#[derive(Clone, Copy, Debug)]
enum Dir {
    Up,
    RI,
    DO,
    LE,
}

const RL: usize = 131;
const RN: usize = 130;

unsafe fn read_obstacles(inp:&[u8]) -> (usize, usize, [bool; RL * RN]) {
    let init_pos = memchr(b'^', inp).expect("input");

    let cur_row = init_pos / RL;
    let cur_col = init_pos % RL;

    let mut obstacles = [false; RL * RN];
    let o_ptr = obstacles.as_mut_ptr();

    for ind in memchr_iter(b'#', inp) {
        o_ptr.add(ind).write(true)
    }

    (cur_row, cur_col, obstacles)
}

unsafe fn path_map(mut cur_row:usize, mut cur_col:usize, obs: [bool; RL * RN]) -> [bool; RL * RN] {
    let o_ptr = obs.as_ptr();
    let mut visited = [false; RL * RN];
    let v_ptr = visited.as_mut_ptr();
    v_ptr.add(cur_row * RL + cur_col).write(true);
    let mut dir = Dir::Up;
    let mut in_map = true;
    while in_map {
        match dir {
            Dir::Up => {
                if let Some(obstacle_row) =
                    (0..cur_row).rfind(|ri| o_ptr.add(ri * RL + cur_col).read())
                {
                    for r in obstacle_row + 1..cur_row {
                        v_ptr.add(r * RL + cur_col).write(true);
                    }
                    cur_row = obstacle_row + 1;
                    dir = Dir::RI;
                } else {
                    for r in 0..cur_row {
                        v_ptr.add(r * RL + cur_col).write(true);
                    }
                    in_map = false;
                }
            }
            Dir::RI => {
                if let Some(obstacle_col) =
                    (cur_col + 1..RN).find(|ci| o_ptr.add(cur_row * RL + ci).read())
                {
                    for c in cur_col + 1..obstacle_col {
                        v_ptr.add(cur_row * RL + c).write(true);
                    }
                    cur_col = obstacle_col - 1;
                    dir = Dir::DO;
                } else {
                    for c in cur_col + 1..RN {
                        v_ptr.add(cur_row * RL + c).write(true);
                    }
                    in_map = false
                }
            }
            Dir::DO => {
                if let Some(obstacle_row) =
                    (cur_row + 1..RN).find(|ri| o_ptr.add(ri * RL + cur_col).read())
                {
                    for r in cur_row + 1..obstacle_row {
                        v_ptr.add(r * RL + cur_col).write(true);
                    }
                    cur_row = obstacle_row - 1;
                    dir = Dir::LE;
                } else {
                    for r in cur_row..RN {
                        v_ptr.add(r * RL + cur_col).write(true);
                    }
                    in_map = false
                }
            }
            Dir::LE => {
                if let Some(obstacle_col) =
                    (0..cur_col).rfind(|ci| o_ptr.add(cur_row * RL + ci).read())
                {
                    for c in obstacle_col + 1..cur_col {
                        v_ptr.add(cur_row * RL + c).write(true);
                    }
                    cur_col = obstacle_col + 1;
                    dir = Dir::Up;
                } else {
                    for c in 0..cur_col {
                        v_ptr.add(cur_row * RL + c).write(true);
                    }
                    in_map = false
                }
            }
        };
    }
    visited
}

unsafe fn part1_inner(inp: &[u8]) -> u32 {
    let (cur_row, cur_col, obstacles) = read_obstacles(inp);
    let visited = path_map(cur_row, cur_col, obstacles);
    visited
        .into_iter()
        .fold(0u32, |acc, b| if b { acc + 1 } else { acc })
}

unsafe fn check_loop(start_row: usize, start_col: usize, obs: [bool; RL * RN]) -> bool {
    // Loop when we reach a point + direction we have done before
    let mut dir = Dir::Up;
    let mut cur_row = start_row;
    let mut cur_col = start_col;

    let o_ptr = obs.as_ptr();

    let mut visited_up = [false; RN * RL]; // To go to next row incr by 1, next col by RN Last row is \n
    let vu_ptr = visited_up.as_mut_ptr();
    let mut visited_ri = [false; RN * RL]; // To go to next c0l incr by 1, next row by RL last col is \n
    let vr_ptr = visited_ri.as_mut_ptr();
    let mut visited_do = [false; RN * RL];
    let vd_ptr = visited_do.as_mut_ptr();
    let mut visited_le = [false; RN * RL];
    let vl_ptr = visited_le.as_mut_ptr();

    // println!("{dir:?}");
    let mut in_map = true;
    // We loop till we hit an obstacle and go in a direction we have done before, 
    // only keep track as this means at most 1 walk direction till loop is detected.
    while in_map {
        match dir {
            Dir::Up => {
                if let Some(obstacle_row) =
                    (0..cur_row).rfind(|ri| o_ptr.add(ri * RL + cur_col).read())
                {
                    if vr_ptr.add((obstacle_row + 1) * RL + cur_col).read() {
                        return true;
                    }
                    vu_ptr.add(cur_row + cur_col * RN).write(true);
                    vu_ptr.add(obstacle_row + 1 + cur_col * RN).write(true);
                    // for r in obstacle_row + 1..=cur_row {
                    //     vu_ptr.add(r + cur_col * RN).write(true);
                    // }
                    cur_row = obstacle_row + 1;
                    dir = Dir::RI;
                } else {
                    in_map = false;
                }
            }
            Dir::RI => {
                if let Some(obstacle_col) =
                    (cur_col + 1..RL - 1).find(|ci| o_ptr.add(cur_row * RL + ci).read())
                {
                    if vd_ptr.add(cur_row + (obstacle_col - 1) * RN).read() {
                        return true;
                    }

                    vr_ptr.add(cur_row * RL + cur_col).write(true);
                    vr_ptr.add(cur_row * RL + obstacle_col - 1).write(true);
                    // for c in cur_col..obstacle_col {
                    //     vr_ptr.add(cur_row * RL + c).write(true);
                    // }
                    cur_col = obstacle_col - 1;
                    dir = Dir::DO;
                } else {
                    in_map = false
                }
            }
            Dir::DO => {
                if let Some(obstacle_row) =
                    (cur_row + 1..RN).find(|ri| o_ptr.add(ri * RL + cur_col).read())
                {
                    if vl_ptr.add((obstacle_row - 1) * RL + cur_col).read() {
                        return true;
                    }
                    vd_ptr.add(cur_row + cur_col * RN).write(true);
                    vd_ptr.add(obstacle_row - 1 + cur_col * RN).write(true);
                    // for r in cur_row..obstacle_row {
                    //     vd_ptr.add(r + cur_col * RN).write(true);
                    // }
                    cur_row = obstacle_row - 1;
                    dir = Dir::LE;
                } else {
                    in_map = false
                }
            }
            Dir::LE => {
                if let Some(obstacle_col) =
                    (0..cur_col).rfind(|ci| o_ptr.add(cur_row * RL + ci).read())
                {
                    if vu_ptr.add(cur_row + (obstacle_col + 1) * RN).read() {
                        return true;
                    }
                    vl_ptr.add(cur_row * RL + obstacle_col + 1).write(true);
                    vl_ptr.add(cur_row * RL + cur_col).write(true);
                    // for c in obstacle_col + 1..=cur_col {
                    //     vl_ptr.add(cur_row * RL + c).write(true);
                    // }
                    cur_col = obstacle_col + 1;
                    dir = Dir::Up;
                } else {
                    in_map = false
                }
            }
        };
    }
    false
}

unsafe fn part2_inner(inp: &[u8]) -> u32 {
    let (cur_row, cur_col, mut obstacles) = read_obstacles(inp);
    let mut visited = path_map(cur_row, cur_col, obstacles);
    visited[cur_row * RL + cur_col] = false;
    let o_ptr = obstacles.as_mut_ptr();
    let mut lp_cnt:u32 = 0;
    for (i, _) in visited.into_iter().enumerate().filter(|(_, b)| *b) {
        o_ptr.add(i).write(true);
        if check_loop(cur_row, cur_col, obstacles) {
            lp_cnt += 1;
        };
        o_ptr.add(i).write(false);
    };
    lp_cnt
}

#[aoc(day6, part1)]
pub fn part1(inp: &str) -> u32 {
    unsafe { part1_inner(inp.as_bytes()) }
}

#[aoc(day6, part2)]
pub fn part2(inp: &str) -> u32 {
    unsafe { part2_inner(inp.as_bytes()) }
}