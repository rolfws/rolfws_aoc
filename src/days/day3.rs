use aoc_runner_derive::aoc;

fn two_u8_to_u32(in1: u8, in2: u8) -> u32 {
    (in1 - b'0') as u32 * 10 + (in2 - b'0') as u32
}

fn three_u8_to_u32(in1: u8, in2: u8, in3: u8) -> u32 {
    (in1 - b'0') as u32 * 100 + (in2 - b'0') as u32 * 10 + (in3 - b'0') as u32
}

#[allow(clippy::missing_safety_doc)]
unsafe fn part_1_wrap(inp: &str) -> u32 {
    let mut acc = 0;
    let mut i: usize = 0;
    let mut left_reg: u32;
    let mut right_reg: u32;
    // Max we read is from i..i+12, so we extend the &[u8] with 4 elements that will be faulty
    let inp_s = [inp, "...."].concat();
    let inp = inp_s.as_bytes();
    let ptr = inp.as_ptr();
    
    while i < inp.len() - 12 {
        // println!("{:?}", std::str::from_utf8(&inp[i..i+4]));
        match [
            ptr.add(i).read(),
            ptr.add(i + 1).read(),
            ptr.add(i + 2).read(),
            ptr.add(i + 3).read(),
        ] {
            [b'm', b'u', b'l', b'('] => i += 4,
            [b'm', b'u', b'l', _] => {
                i += 3;
                continue;
            }
            [b'm', b'u', _, _] => {
                i += 2;
                continue;
            }
            _ => {
                i += 1;
                continue;
            } // IF slice is not 4 elements we do not go into this by while.
        }

        // Yay we got mul(
        // println!("{:?}", std::str::from_utf8(&inp[i..i+4]));
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
            [b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', _] => {
                i += 3;
                continue;
            }
            [b'0'..=b'9', b'0'..=b'9', b',', _] => {
                left_reg = two_u8_to_u32(inp[i], inp[i + 1]);
                i += 3
            }
            [b'0'..=b'9', b'0'..=b'9', _, _] => {
                i += 2;
                continue;
            }
            [b'0'..=b'9', b',', _, _] => {
                left_reg = (inp[i] - b'0') as u32;
                i += 2
            }
            _ => {
                i += 1;
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
            [b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', _] => {
                i += 3;
                continue;
            }
            [b'0'..=b'9', b'0'..=b'9', b')', _] => {
                right_reg = two_u8_to_u32(inp[i], inp[i + 1]);
                i += 3
            }
            [b'0'..=b'9', b'0'..=b'9', _, _] => {
                i += 2;
                continue;
            }
            [b'0'..=b'9', b')', _, _] => {
                right_reg = (inp[i] - b'0') as u32;
                i += 2
            }
            _ => {
                i += 1;
                continue;
            }
        }
        acc += left_reg * right_reg
    }
    acc
}

pub fn part1_safe(inp: &[u8]) -> u32 {
    let mut acc = 0;
    let mut i: usize = 0;
    let mut left_reg: u32;
    let mut right_reg: u32;
    while i < inp.len() - 8 {
        // println!("{:?}", std::str::from_utf8(&inp[i..i+4]));
        match inp[i..i + 4] {
            [b'm', b'u', b'l', b'('] => i += 4,
            [b'm', b'u', b'l', _] => {
                i += 3;
                continue;
            }
            [b'm', b'u', _, _] => {
                i += 2;
                continue;
            }
            _ => {
                i += 1;
                continue;
            } // IF slice is not 4 elements we do not go into this by while.
        }

        // Yay we got mul(
        // Now we need to check if up to next 3 are digits
        // Here we still can not go out of bounds as 8 - 4 = 4
        // println!("{:?}", std::str::from_utf8(&inp[i..i+4]));
        match inp[i..i + 4] {
            [b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', b','] => {
                left_reg = three_u8_to_u32(inp[i], inp[i + 1], inp[i + 2]);
                i += 4
            }
            [b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', _] => {
                i += 3;
                continue;
            }
            [b'0'..=b'9', b'0'..=b'9', b',', _] => {
                left_reg = two_u8_to_u32(inp[i], inp[i + 1]);
                i += 3
            }
            [b'0'..=b'9', b'0'..=b'9', _, _] => {
                i += 2;
                continue;
            }
            [b'0'..=b'9', b',', _, _] => {
                left_reg = (inp[i] - b'0') as u32;
                i += 2
            }
            _ => {
                i += 1;
                continue;
            }
        }

        match inp[i..std::cmp::min(i + 4, inp.len() - 1)] {
            [b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', b')'] => {
                right_reg = three_u8_to_u32(inp[i], inp[i + 1], inp[i + 2]);
                i += 4
            } // Valid 3
            [b'0'..=b'9', b'0'..=b'9', b')', _] | [b'0'..=b'9', b'0'..=b'9', b')'] => {
                right_reg = two_u8_to_u32(inp[i], inp[i + 1]);
                i += 3
            } // Valid 2
            [b'0'..=b'9', b')', _, _] | [b'0'..=b'9', b')', _] | [b'0'..=b'9', b')'] => {
                right_reg = (inp[i] - b'0') as u32;
                i += 2
            } // Valid 1
            [b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', _] => {
                i += 3;
                continue;
            }
            [b'0'..=b'9', b'0'..=b'9', _, _] => {
                i += 2;
                continue;
            }
            _ => {
                i += 1;
                continue;
            }
        }
        // Yay we got mul(x,
        // Remains to check for y). Now we can go out of range.
        acc += left_reg * right_reg
    }
    acc
}


#[allow(clippy::missing_safety_doc)]
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
    
    while i < inp.len() - 12 {
        if !doing {
            // Check if the next 4 are do(), else we cont
            match [ptr.add(i).read(),
            ptr.add(i + 1).read(),
            ptr.add(i + 2).read(),
            ptr.add(i + 3).read()
            ] {
                [b'd', b'o', b'(', b')'] => {i+=4; doing=true},
                [b'd', b'o', b'(', _] => {i+=3},
                [b'd', b'o', _, _] => {i+=2},
                _ => {i+=1}
            }
            continue;
        }
        
        // Check for mul( 4 or don't() 7
        match [
            ptr.add(i).read(),
            ptr.add(i + 1).read(),
            ptr.add(i + 2).read(),
            ptr.add(i + 3).read(),
            ptr.add(i + 4).read(),
            ptr.add(i + 5).read(),
            ptr.add(i + 6).read(),
        ] {
            [b'd', b'o', b'n', b'\'', b't', b'(', b')'] => {i += 7; doing=false},
            [b'm', b'u', b'l', b'(', _, _, _] => i += 4,
            [b'm', b'u', b'l', _, _, _, _] => {
                i += 3;
                continue;
            }
            [b'm', b'u', _, _, _, _, _] => {
                i += 2;
                continue;
            }
            _ => {
                i += 1;
                continue;
            }
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
            [b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', _] => {
                i += 3;
                continue;
            }
            [b'0'..=b'9', b'0'..=b'9', b',', _] => {
                left_reg = two_u8_to_u32(inp[i], inp[i + 1]);
                i += 3
            }
            [b'0'..=b'9', b'0'..=b'9', _, _] => {
                i += 2;
                continue;
            }
            [b'0'..=b'9', b',', _, _] => {
                left_reg = (inp[i] - b'0') as u32;
                i += 2
            }
            _ => {
                i += 1;
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
            [b'0'..=b'9', b'0'..=b'9', b'0'..=b'9', _] => {
                i += 3;
                continue;
            }
            [b'0'..=b'9', b'0'..=b'9', b')', _] => {
                right_reg = two_u8_to_u32(inp[i], inp[i + 1]);
                i += 3
            }
            [b'0'..=b'9', b'0'..=b'9', _, _] => {
                i += 2;
                continue;
            }
            [b'0'..=b'9', b')', _, _] => {
                right_reg = (inp[i] - b'0') as u32;
                i += 2
            }
            _ => {
                i += 1;
                continue;
            }
        }
        acc += left_reg * right_reg
    }
    acc
}

// use regex::Regex;

// #[aoc(day3, part1, regex)]
// pub fn part1_reg(inp:&str) -> u32 {
//     let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
//     re.captures_iter(inp).map(|c| c.extract()).fold(0u32, |acc, (_, [left, right])| 
//         acc + left.parse::<u32>().unwrap() * right.parse::<u32>().unwrap()
//     )
// }   

#[aoc(day3, part1)]
pub fn part1(inp:&str) -> u32 {
    unsafe {part_1_wrap(inp)}
}

#[aoc(day3, part1, safe)]
pub fn part1_s(inp:&[u8]) -> u32 {
    part1_safe(inp)
}

#[aoc(day3, part2)]
pub fn part2(inp:&str) -> u32 {
    unsafe {part_2_core(inp)}
}