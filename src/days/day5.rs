use aoc_runner_derive::aoc;
use memchr::memchr_iter;

// use fxhash::FxHashMap;

fn u8_from_2b(inp: [u8; 2]) -> u8 {
    (inp[0] - b'0') * 10 + (inp[1] - b'0')
}

// type U8map = FxHashMap<u8, Vec<u8>>;
unsafe fn read_rule_maps(inp: &str) -> ([bool; 10_000], &str) {
    let mut before_map = [false; 10_000];
    let mut i = 0;
    let ptr = inp.as_ptr();
    let wptr = before_map.as_mut_ptr();
    loop {
        if ptr.add(i).read() == b'\n' {
            i += 1;
            break;
        }
        let l = u8_from_2b((ptr.add(i) as *const [u8; 2]).read());
        let r = u8_from_2b((ptr.add(i + 3) as *const [u8; 2]).read());

        wptr.add((l as usize) * 100 + r as usize).write(true);
        i += 6;
    }
    let r_str = &inp[i..];
    (before_map, r_str)
}

unsafe fn part1_inner(inp: &str) -> u32 {
    let (mapb, rstr) = read_rule_maps(inp);
    let map_ptr = mapb.as_ptr();
    let mut prev: usize = 0;
    let ptr = rstr.as_ptr();
    let mut line_buf: [u8; 100] = [0; 100];
    let buf_ptr = line_buf.as_mut_ptr();
    let mut nums: usize;
    let mut acc: u32 = 0;
    'lines: for next in memchr_iter(b'\n', rstr.as_bytes()) {
        nums = (next - prev + 1) / 3;
        for j in 0..nums {
            // Fill and check back
            let new = u8_from_2b((ptr.add(prev + j * 3) as *const [u8; 2]).read());
            buf_ptr.add(j).write(new);
            for i in 0..j {
                // Check if earlier numbers violate that the new number needs to be before
                if map_ptr
                    .add((new as usize) * 100 + buf_ptr.add(i).read() as usize)
                    .read()
                {
                    // println!("{}, {}", new, buf_ptr.add(i).read());
                    prev = next + 1;
                    continue 'lines;
                }
            }
        }
        prev = next + 1;
        acc += buf_ptr.add(nums / 2).read() as u32
    }
    acc
}

unsafe fn part2_inner(inp: &str) -> u32 {
    let (mapb, rstr) = read_rule_maps(inp);
    let map_ptr = mapb.as_ptr();
    let mut prev: usize = 0;
    let ptr = rstr.as_ptr();
    let mut line_buf: [u8; 100] = [0; 100];
    let buf_ptr = line_buf.as_mut_ptr();
    let mut nums: usize;
    let mut acc: u32 = 0;
    let mut cor:bool;
    for next in memchr_iter(b'\n', rstr.as_bytes()) {
        nums = (next - prev + 1) / 3;
        cor = true;
        for j in 0..nums {
            // Fill and check back
            let new = u8_from_2b((ptr.add(prev + j * 3) as *const [u8; 2]).read());
            buf_ptr.add(j).write(new);
            for i in 0..j {
                // Check if earlier numbers violate that the new number needs to be before
                if map_ptr
                    .add((new as usize) * 100 + buf_ptr.add(i).read() as usize)
                    .read()
                {
                    cor = false;
                }
            }
        }
        prev = next + 1;

        if cor {
            continue;
        }

        line_buf[0..nums].sort_unstable_by(
            |l, r| if map_ptr.add((*l as usize) * 100 + *r as usize).read() {
                std::cmp::Ordering::Less
            } else if map_ptr.add((*r as usize) * 100 + *l as usize).read() {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        );

        acc += buf_ptr.add(nums / 2).read() as u32
    }
    acc
}

#[aoc(day5, part1)]
pub fn part1(inp: &str) -> u32 {
    unsafe { part1_inner(inp) }
}

#[aoc(day5, part2)]
pub fn part2(inp: &str) -> u32 {
    unsafe { part2_inner(inp) }
}
