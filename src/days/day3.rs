use aoc_runner_derive::aoc;

fn two_u8_to_u32(in1: u8, in2: u8) -> u32 {
    (in1 - b'0') as u32 * 10 + (in2 - b'0') as u32
}

fn three_u8_to_u32(in1: u8, in2: u8, in3: u8) -> u32 {
    (in1 - b'0') as u32 * 100 + (in2 - b'0') as u32 * 10 + (in3 - b'0') as u32
}

unsafe fn part_1_wrap(inp: &str) -> u32 {
    let mut acc = 0;
    let mut i: usize = 0;
    let mut left_reg: u32;
    let mut right_reg: u32;
    // Max we read is from i..i+12, so we extend the &[u8] with 4 elements that will be faulty
    // 12 is the max length of mul(xyz,abc)
    let inp_s = [inp, "...."].concat();
    let inp = inp_s.as_bytes();
    let ptr = inp.as_ptr();

    while i < inp.len() - 11 {
        // Slices len is atleast 12 elements so all 3 sets of 4 reads are all safe.
        match [
            ptr.add(i).read(),
            ptr.add(i + 1).read(),
            ptr.add(i + 2).read(),
            ptr.add(i + 3).read(),
        ] {
            [b'm', b'u', b'l', b'('] => i += 4,
            [_, b'm', b'u', b'l'] => {
                i += 1;
                continue;
            }
            [_, _, b'm', b'u'] => {
                i += 2;
                continue;
            }
            [_, _, _, b'm'] => {
                i += 3;
                continue;
            }
            _ => {
                i += 4;
                continue;
            } // IF slice is not 4 elements we do not go into this by while.
        }

        // Yay we got mul(
        match [
            ptr.add(i).read(),
            ptr.add(i + 1).read(),
            ptr.add(i + 2).read(),
            ptr.add(i + 3).read(),
        ] {
            [b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', b','] => {
                left_reg = three_u8_to_u32(inp[i], inp[i + 1], inp[i + 2]);
                i += 4
            }
            [b'0'..=b'9', b'0'..=b'9', b',', _] => {
                left_reg = two_u8_to_u32(inp[i], inp[i + 1]);
                i += 3
            }
            [b'0'..=b'9', b',', _, _] => {
                left_reg = (inp[i] - b'0') as u32;
                i += 2
            }

            // No numbers with comma, but we read any way so we check for mul(
            [b'm', b'u', b'l', b'('] => {
                continue;
            } // This could be done a little more efficient
            [_, b'm', b'u', b'l'] => {
                i += 1;
                continue;
            }
            [_, _, b'm', b'u'] => {
                i += 2;
                continue;
            }
            [_, _, _, b'm'] => {
                i += 3;
                continue;
            }
            _ => {
                i += 4;
                continue;
            }
        }

        match [
            ptr.add(i).read(),
            ptr.add(i + 1).read(),
            ptr.add(i + 2).read(),
            ptr.add(i + 3).read(),
        ] {
            [b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', b')'] => {
                right_reg = three_u8_to_u32(inp[i], inp[i + 1], inp[i + 2]);
                i += 4
            }
            [b'0'..=b'9', b'0'..=b'9', b')', _] => {
                right_reg = two_u8_to_u32(inp[i], inp[i + 1]);
                i += 3
            }
            [b'0'..=b'9', b')', _, _] => {
                right_reg = (inp[i] - b'0') as u32;
                i += 2
            }

            [b'm', b'u', b'l', b'('] => {
                continue;
            } // This could be done a little more efficient, as the first check is now done twice.
            [_, b'm', b'u', b'l'] => {
                i += 1;
                continue;
            }
            [_, _, b'm', b'u'] => {
                i += 2;
                continue;
            }
            [_, _, _, b'm'] => {
                i += 3;
                continue;
            }
            _ => {
                i += 4;
                continue;
            }
        }
        acc += left_reg * right_reg
    }
    acc
}

unsafe fn part_2_core(inp: &str) -> u32 {
    let mut doing = true;
    let mut acc = 0;
    let mut i: usize = 0;
    let mut left_reg: u32;
    let mut right_reg: u32;
    // Max we read is from i..i+12, so we extend the &[u8] with 4 elements that will be faulty
    let inp_s = [inp, "...."].concat();
    let inp = inp_s.as_bytes();
    let ptr = inp.as_ptr();

    while i < inp.len() - 11 {
        // Len is 12 so this part is safe
        if !doing {
            // Check if the next 4 are do(), else we cont
            match [
                ptr.add(i).read(),
                ptr.add(i + 1).read(),
                ptr.add(i + 2).read(),
                ptr.add(i + 3).read(),
            ] {
                [b'd', b'o', b'(', b')'] => {
                    i += 4;
                    doing = true
                }
                [_, b'd', b'o', b'('] => i += 1,
                [_, _, b'd', b'o'] => i += 2,
                [_, _, _, b'd'] => i += 3,
                _ => i += 4,
            }
            continue;
        }

        // Check for mul( 4 or don't() 7
        // Len is 12 so this part is safe
        match [
            ptr.add(i).read(),
            ptr.add(i + 1).read(),
            ptr.add(i + 2).read(),
            ptr.add(i + 3).read(),
            ptr.add(i + 4).read(),
            ptr.add(i + 5).read(),
            ptr.add(i + 6).read(),
        ] {
            [b'd', b'o', b'n', b'\'', b't', b'(', b')'] => {
                i += 7;
                doing = false;
                continue;
            }
            [b'm', b'u', b'l', b'(', _, _, _] => i += 4,
            [_, b'm', b'u', b'l', b'(', _, _] => i += 5,
            [_, _, b'm', b'u', b'l', b'(', _] => i += 6,
            [_, _, _, b'm', b'u', b'l', b'('] => i += 7,

            [_, b'd', b'o', b'n', b'\'', b't', b'('] => {
                i += 1;
                continue;
            }

            [_, _, b'd', b'o', b'n', b'\'', b't'] => {
                i += 2;
                continue;
            }

            [_, _, _, b'd', b'o', b'n', b'\''] => {
                i += 3;
                continue;
            }

            [_, _, _, _, b'd', b'o', b'n'] | [_, _, _, _, b'm', b'u', b'l'] => {
                i += 4;
                continue;
            }

            [_, _, _, _, _, b'd', b'o'] | [_, _, _, _, _, b'm', b'u'] => {
                i += 5;
                continue;
            }

            [_, _, _, _, _, _, b'd'] | [_, _, _, _, _, _, b'm'] => {
                i += 6;
                continue;
            }
            _ => {
                i += 7;
                continue;
            }
        }
        // From the previous there are atleast 12 - 7 = 5 elements left,
        // if we reached the end we will hit the '....'. So we wont match and terminate.
        // This last part is essential for the read after this one.
        match [
            ptr.add(i).read(),
            ptr.add(i + 1).read(),
            ptr.add(i + 2).read(),
            ptr.add(i + 3).read(),
        ] {
            [b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', b','] => {
                left_reg = three_u8_to_u32(inp[i], inp[i + 1], inp[i + 2]);
                i += 4
            }
            [b'0'..=b'9', b'0'..=b'9', b',', _] => {
                left_reg = two_u8_to_u32(inp[i], inp[i + 1]);
                i += 3
            }
            [b'0'..=b'9', b',', _, _] => {
                left_reg = (inp[i] - b'0') as u32;
                i += 2
            }

            // No numbers, read any way so we check for m / d (Start of maybe next)
            // do() is not an interesting match, as we already are doing.
            [b'm', b'u', b'l', b'('] | [b'd', b'o', b'n', b'\''] => {
                continue;
            } // This could be done a little more efficient
            [_, b'm', b'u', b'l'] | [_, b'd', b'o', b'n'] => {
                i += 1;
                continue;
            }
            [_, _, b'm', b'u'] | [_, _, b'd', b'o'] => {
                i += 2;
                continue;
            }
            [_, _, _, b'm'] | [_, _, _, b'd'] => {
                i += 3;
                continue;
            }
            _ => {
                i += 4;
                continue;
            }
        }

        // At this point if there was a . we would have continued, and terminated, so next reads are surely in bound.
        match [
            ptr.add(i).read(),
            ptr.add(i + 1).read(),
            ptr.add(i + 2).read(),
            ptr.add(i + 3).read(),
        ] {
            [b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', b')'] => {
                right_reg = three_u8_to_u32(inp[i], inp[i + 1], inp[i + 2]);
                i += 4
            }
            [b'0'..=b'9', b'0'..=b'9', b')', _] => {
                right_reg = two_u8_to_u32(inp[i], inp[i + 1]);
                i += 3
            }
            [b'0'..=b'9', b')', _, _] => {
                right_reg = (inp[i] - b'0') as u32;
                i += 2
            }

            // No numbers, read any way so we check for m / d (Start of maybe next)
            // do() is not an interesting match, as we already are doing.
            [b'm', b'u', b'l', b'('] | [b'd', b'o', b'n', b'\''] => {
                continue;
            } // This could be done a little more efficient
            [_, b'm', b'u', b'l'] | [_, b'd', b'o', b'n'] => {
                i += 1;
                continue;
            }
            [_, _, b'm', b'u'] | [_, _, b'd', b'o'] => {
                i += 2;
                continue;
            }
            [_, _, _, b'm'] | [_, _, _, b'd'] => {
                i += 3;
                continue;
            }
            _ => {
                i += 4;
                continue;
            }
        }
        acc += left_reg * right_reg
    }
    acc
}

#[aoc(day3, part1)]
pub fn part1(inp: &str) -> u32 {
    unsafe { part_1_wrap(inp) }
}
// #[aoc(day3, part1, safe)]
// pub fn part1_s(inp:&[u8]) -> u32 {
//     part1_safe(inp)
// }

#[aoc(day3, part2)]
pub fn part2(inp: &str) -> u32 {
    unsafe { part_2_core(inp) }
}
