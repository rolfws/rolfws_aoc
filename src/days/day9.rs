use std::{cmp::Reverse, collections::BinaryHeap};

use aoc_runner_derive::aoc;

#[allow(unused)]
unsafe fn load_u64(inp: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let mut nums_blocks: Vec<u8> = Vec::with_capacity(inp.len() / 2 + 1);
    let mut empty_block: Vec<u8> = Vec::with_capacity(inp.len() / 2 + 1);
    let t_num: u64 = u64::from_ne_bytes([b'0'; 8]);
    let inp_ptr = inp.as_ptr() as *const u64;

    let skip_last = (inp.len() % 8 == 0) as usize;
    for i in 0..inp.len() / 8 - skip_last {
        let nums = (inp_ptr.add(i).read_unaligned() - t_num).to_ne_bytes();
        nums_blocks.extend([nums[0], nums[2], nums[4], nums[6]]);
        empty_block.extend([nums[1], nums[3], nums[5], nums[7]]);
    }
    let offset = inp.len() - (inp.len() % 8 + skip_last * 8);
    let additional = (inp.len() % 2 == 0) as usize;

    for i in 0..(inp.len() % 8 + skip_last * 8) / 2 - additional {
        nums_blocks.push(inp[offset + i * 2] - b'0');
        empty_block.push(inp[offset + i * 2 + 1] - b'0');
    }

    nums_blocks.push(inp[inp.len() - 1 - additional] - b'0');
    empty_block.push(0);
    (nums_blocks, empty_block)
}

unsafe fn load_u128(inp: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let mut nums_blocks: Vec<u8> = Vec::with_capacity(inp.len() / 2 + 1);
    let mut empty_block: Vec<u8> = Vec::with_capacity(inp.len() / 2 + 1);
    let t_num: u128 = u128::from_ne_bytes([b'0'; 16]);
    let inp_ptr = inp.as_ptr() as *const u128;

    let skip_last = (inp.len() % 16 == 0) as usize;
    for i in 0..inp.len() / 16 - skip_last {
        let nums = (inp_ptr.add(i).read_unaligned() - t_num).to_ne_bytes();
        nums_blocks.extend([
            nums[0], nums[2], nums[4], nums[6], nums[8], nums[10], nums[12], nums[14],
        ]);
        empty_block.extend([
            nums[1], nums[3], nums[5], nums[7], nums[9], nums[11], nums[13], nums[15],
        ]);
    }
    let offset = inp.len() - (inp.len() % 16 + skip_last * 16);
    let additional = (inp.len() % 2 == 0) as usize;

    for i in 0..(inp.len() % 16 + skip_last * 16) / 2 - additional {
        nums_blocks.push(inp[offset + i * 2] - b'0');
        empty_block.push(inp[offset + i * 2 + 1] - b'0');
    }

    nums_blocks.push(inp[inp.len() - 1 - additional] - b'0');
    empty_block.push(0);
    (nums_blocks, empty_block)
}

unsafe fn part1_inner(inp: &[u8]) -> u64 {
    let (mut nums, mut empty) = load_u128(inp);
    let mut n_gauss: u64;
    let mut prev_gauss = 0u64;
    let mut ans = 0u64;
    let mut num_cnt = 0u64;
    let mut b_ind = 0;
    let mut e_ind = nums.len() - 1;

    while b_ind < e_ind {
        num_cnt += *nums.get_unchecked(b_ind) as u64;
        n_gauss = (num_cnt - 1) * num_cnt / 2;
        ans += (n_gauss - prev_gauss) * b_ind as u64;
        prev_gauss = n_gauss;
        *nums.get_unchecked_mut(b_ind) = 0;
        loop {
            if empty.get_unchecked(b_ind) > nums.get_unchecked(e_ind) {
                *empty.get_unchecked_mut(b_ind) -= nums.get_unchecked(e_ind);
                num_cnt += *nums.get_unchecked(e_ind) as u64;
                n_gauss = (num_cnt - 1) * num_cnt / 2;
                ans += (n_gauss - prev_gauss) * e_ind as u64;
                prev_gauss = n_gauss;
                if e_ind == b_ind {
                    break;
                }
                e_ind -= 1;
            } else {
                num_cnt += *empty.get_unchecked(b_ind) as u64;
                n_gauss = (num_cnt - 1) * num_cnt / 2;
                ans += (n_gauss - prev_gauss) * e_ind as u64;
                prev_gauss = n_gauss;
                *nums.get_unchecked_mut(e_ind) -= *empty.get_unchecked(b_ind);
                break;
            }
        }
        b_ind += 1;
    }
    num_cnt += *nums.get_unchecked(b_ind) as u64;
    n_gauss = (num_cnt - 1) * num_cnt / 2;
    ans += (n_gauss - prev_gauss) * b_ind as u64;

    ans
}

unsafe fn part2_inner_opt(inp: &[u8]) -> u64 {
    let (nums, empty) = load_u128(inp);
    // a_cnt gives elements before the gap, c_cnt before the numbers themself.
    let (c_cnt, mut a_cnt) = nums
        .iter()
        .zip(empty.iter())
        .scan(0u64, |acc, (&n, &e)| {
            let r = Some((*acc, *acc + n as u64 - 1));
            *acc += (n + e) as u64;
            r
        })
        .collect::<(Vec<_>, Vec<_>)>(); // Count of numbers before.

    let mut gaps: [BinaryHeap<Reverse<usize>>; 9] =
        core::array::from_fn(|_| BinaryHeap::with_capacity(2500));
    for (i, g_size) in empty.into_iter().enumerate() {
        if g_size == 0 {
            continue;
        }
        gaps[g_size as usize - 1].push(Reverse(i));
    }

    let mut ans = 0u64;

    'outer: for e_ind in (1..=nums.len() - 1).rev() {
        let e_num = *nums.get_unchecked(e_ind) as usize;
        if let Some((gap_idx, _)) = gaps[e_num - 1..]
            .iter()
            .enumerate()
            .filter_map(|(i, g)| g.peek().map(|ind| (i, ind)))
            .max_by_key(|(_, &g)| g)
        // Max because of the Reverse
        {
            let Reverse(gap_ind) = gaps[e_num + gap_idx - 1].pop().expect("We just peeked");
            if gap_ind >= e_ind {
                // So the min idx of all gaps >= e_num are not used anymore so we clear them and then not move
                gaps[e_num - 1..].iter_mut().for_each(|g| g.clear());
            } else {
                let num_b = a_cnt.get_unchecked_mut(gap_ind);
                let m = e_num as u64;
                ans += (m * (2 * *num_b + m + 1) / 2) * e_ind as u64;
                *num_b += e_num as u64;
                if gap_idx > 0 {
                    gaps[gap_idx - 1].push(Reverse(gap_ind));
                }
                continue 'outer;
            }
        }
        let num_b = c_cnt.get_unchecked(e_ind) - 1; // zeroes are skipped, so all have something in front. i.e. not wrapping.
        let m = e_num as u64;
        ans += (m * (2 * num_b + m + 1) / 2) * e_ind as u64;
    }
    ans
}

#[aoc(day9, part1)]
pub fn part1(inp: &str) -> u64 {
    unsafe { part1_inner(inp.as_bytes()) }
}

#[aoc(day9, part2)]
pub fn part2(inp: &str) -> u64 {
    unsafe { part2_inner_opt(inp.as_bytes()) }
}

#[cfg(test)]
mod test {
    use super::part2;

    use super::part1;

    #[test]
    fn test_part1() {
        let inp = "2333133121414131402\n";
        assert_eq!(part1(inp), 1928);
    }

    #[test]
    fn test_part2() {
        let inp = "2333133121414131402\n";
        assert_eq!(part2(inp), 2858);
    }
}
