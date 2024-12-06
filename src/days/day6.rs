use aoc_runner_derive::aoc;
use memchr::{memchr, memchr_iter};

#[derive(Clone, Copy, Debug)]
enum Dir {
    Up,
    RI,
    DO,
    LE,
}

unsafe fn part1_inner(inp: &[u8]) -> u32 {
    let init_pos = memchr(b'^', inp).expect("input");

    let mut cur_row = init_pos / 131;
    let mut cur_col = init_pos % 131;
    let mut visited = [false; 130 * 131];
    let v_ptr = visited.as_mut_ptr();
    v_ptr.add(init_pos).write(true);

    let mut obstacles = [false; 130 * 131];
    let o_ptr = obstacles.as_mut_ptr();
    let mut dir = Dir::Up;

    for ind in memchr_iter(b'#', inp) {
        o_ptr.add(ind).write(true)
    }

    let mut in_map = true;
    while in_map {
        match dir {
            Dir::Up => {
                if let Some(obstacle_row) =
                    (0..cur_row).rfind(|ri| o_ptr.add(ri * 131 + cur_col).read())
                {
                    for r in obstacle_row + 1..cur_row {
                        v_ptr.add(r * 131 + cur_col).write(true);
                    }
                    cur_row = obstacle_row + 1;
                    dir = Dir::RI;
                } else {
                    for r in 0..cur_row {
                        v_ptr.add(r * 131 + cur_col).write(true);
                    }
                    in_map = false;
                }
            }
            Dir::RI => {
                if let Some(obstacle_col) =
                    (cur_col + 1..130).find(|ci| o_ptr.add(cur_row * 131 + ci).read())
                {
                    for c in cur_col + 1..obstacle_col {
                        v_ptr.add(cur_row * 131 + c).write(true);
                    }
                    cur_col = obstacle_col - 1;
                    dir = Dir::DO;
                } else {
                    for c in cur_col + 1..130 {
                        v_ptr.add(cur_row * 131 + c).write(true);
                    }
                    in_map = false
                }
            }
            Dir::DO => {
                if let Some(obstacle_row) =
                    (cur_row + 1..130).find(|ri| o_ptr.add(ri * 131 + cur_col).read())
                {
                    for r in cur_row + 1..obstacle_row {
                        v_ptr.add(r * 131 + cur_col).write(true);
                    }
                    cur_row = obstacle_row - 1;
                    dir = Dir::LE;
                } else {
                    for r in cur_row..130 {
                        v_ptr.add(r * 131 + cur_col).write(true);
                    }
                    in_map = false
                }
            }
            Dir::LE => {
                if let Some(obstacle_col) =
                    (0..cur_col).rfind(|ci| o_ptr.add(cur_row * 131 + ci).read())
                {
                    for c in obstacle_col + 1..cur_col {
                        v_ptr.add(cur_row * 131 + c).write(true);
                    }
                    cur_col = obstacle_col + 1;
                    dir = Dir::Up;
                } else {
                    for c in 0..cur_col {
                        v_ptr.add(cur_row * 131 + c).write(true);
                    }
                    in_map = false
                }
            }
        };
    }
    visited
        .into_iter()
        .fold(0u32, |acc, b| if b { acc + 1 } else { acc })
}

const RL: usize = 131;
const RN: usize = 130;

unsafe fn check_loop(start_row: usize, start_col: usize, obs: [bool; RL * RN]) -> bool {
    // Loop when we reach a point + direction we have done before
    let mut dir = Dir::Up;
    let mut cur_row = start_row;
    let mut cur_col = start_col;

    let o_ptr = obs.as_ptr();
    let mut visited_up = [false; RN * RL];
    let vu_ptr = visited_up.as_mut_ptr();
    let mut visited_ri = [false; RN * RL];
    let vr_ptr = visited_ri.as_mut_ptr();
    let mut visited_do = [false; RN * RL];
    let vd_ptr = visited_do.as_mut_ptr();
    let mut visited_le = [false; RN * RL];
    let vl_ptr = visited_le.as_mut_ptr();

    // println!("{dir:?}");
    let mut in_map = true;
    while in_map {
        match dir {
            Dir::Up => {
                if let Some(obstacle_row) =
                    (0..cur_row).rfind(|ri| o_ptr.add(ri * RL + cur_col).read())
                {
                    if vr_ptr.add((obstacle_row + 1) * RL + cur_col).read() {
                        return true;
                    }

                    for r in obstacle_row + 1..=cur_row {
                        vu_ptr.add(r * RL + cur_col).write(true);
                    }
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
                    if vd_ptr.add(cur_row * RL + obstacle_col - 1).read() {
                        return true;
                    }

                    for c in cur_col..obstacle_col {
                        vr_ptr.add(cur_row * RL + c).write(true);
                    }
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
                    for r in cur_row..obstacle_row {
                        vd_ptr.add(r * RL + cur_col).write(true);
                    }
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
                    if vu_ptr.add(cur_row * RL + obstacle_col + 1).read() {
                        return true;
                    }
                    for c in obstacle_col + 1..=cur_col {
                        vl_ptr.add(cur_row * RL + c).write(true);
                    }
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
    let init_pos = memchr(b'^', inp).expect("input");

    let start_row = init_pos / RL;
    let start_col = init_pos % RL;

    let mut cur_row = start_row;
    let mut cur_col = start_col;

    let mut obstacles = [false; RN * RL];
    let o_ptr = obstacles.as_mut_ptr();
    let mut dir = Dir::Up;

    for ind in memchr_iter(b'#', inp) {
        o_ptr.add(ind).write(true)
    }

    let mut lp = [false; RN * RL];
    let lp_ptr = lp.as_mut_ptr();
    loop {
        match dir {
            Dir::Up => {
                let next_add = (cur_row - 1) * RL + cur_col;
                if cur_row == 0 {
                    break; // We leave the map
                } else if o_ptr.add(next_add).read() {
                    // obs
                    dir = Dir::RI;
                } else {
                    // no obs

                    if !lp_ptr.add(next_add).read() {
                        o_ptr.add(next_add).write(true);
                        if check_loop(start_row, start_col, obstacles) {
                            lp_ptr.add(next_add).write(true);
                        };
                        o_ptr.add(next_add).write(false);
                    }

                    cur_row -= 1;
                }
            }
            Dir::RI => {
                let next_add = cur_row * RL + cur_col + 1;
                if cur_col == RL - 2 {
                    break; // We leave the map
                } else if o_ptr.add(next_add).read() {
                    // obs
                    dir = Dir::DO;
                } else {
                    // no obs
                    if !lp_ptr.add(next_add).read() {
                        o_ptr.add(next_add).write(true);
                        if check_loop(start_row, start_col, obstacles) {
                            lp_ptr.add(next_add).write(true);
                        };
                        o_ptr.add(next_add).write(false);
                    }
                    cur_col += 1;
                }
            }
            Dir::DO => {
                let next_add = (cur_row + 1) * RL + cur_col;
                if cur_row == RN - 1 {
                    break; // We leave the map
                } else if o_ptr.add(next_add).read() {
                    // obs
                    dir = Dir::LE;
                } else {
                    // no obs
                    if !lp_ptr.add(next_add).read() {
                        o_ptr.add(next_add).write(true);
                        if check_loop(start_row, start_col, obstacles) {
                            lp_ptr.add(next_add).write(true);
                        };
                        o_ptr.add(next_add).write(false);
                    }
                    cur_row += 1;
                }
            }
            Dir::LE => {
                let next_add = cur_row * RL + cur_col - 1;
                if cur_col == 0 {
                    break; // We leave the map
                } else if o_ptr.add(next_add).read() {
                    // obs
                    dir = Dir::Up;
                } else {
                    // no obs
                    if !lp_ptr.add(next_add).read() {
                        o_ptr.add(next_add).write(true);
                        if check_loop(start_row, start_col, obstacles) {
                            lp_ptr.add(next_add).write(true);
                        };
                        o_ptr.add(next_add).write(false);
                    }
                    cur_col -= 1;
                }
            }
        }
    }
    lp.into_iter()
        .fold(0u32, |acc, b| if b { acc + 1 } else { acc })
}

#[aoc(day6, part1)]
pub fn part1(inp: &str) -> u32 {
    unsafe { part1_inner(inp.as_bytes()) }
}

#[aoc(day6, part2)]
pub fn part2(inp: &str) -> u32 {
    unsafe { part2_inner(inp.as_bytes()) }
}
