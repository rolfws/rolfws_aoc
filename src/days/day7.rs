use aoc_runner_derive::aoc;
use memchr::memchr_iter;

fn fast_parseu64(slc: &[u8]) -> u64 {
    let mut out = 0u64;
    for b in slc {
        out *= 10;
        out += (b - b'0') as u64
    }
    out
}

fn consume_test_mul(test: u64, buf: &[u64]) -> bool {
    if buf.len() == 1 {
        test == buf[0]
    } else if test % buf[buf.len() - 1] != 0 {
        false
    } else {
        let last = buf.len() - 1;
        let rem = test / buf[last];
        consume_test_mul(rem, &buf[..last]) || consume_test_add(rem, &buf[..last])
    }
}

fn consume_test_add(test: u64, buf: &[u64]) -> bool {
    if buf.len() == 1 {
        test == buf[0]
    } else if test < buf[buf.len() - 1] {
        false
    } else {
        let last = buf.len() - 1;
        let rem = test - buf[last];
        consume_test_mul(rem, &buf[..last]) || consume_test_add(rem, &buf[..last])
    }
}

unsafe fn part1_inner_opt(inp: &[u8]) -> u64 {
    let mut prev: usize = 0;
    let mut num_start;
    let mut buf: [u64; 20] = [0; 20];
    let mut len: usize;
    let mut ok_cnt = 0u64;
    let buf_ptr = buf.as_mut_ptr();
    for (new_row, new_split) in memchr_iter(b'\n', inp).zip(memchr_iter(b':', inp)) {
        let test = fast_parseu64(&inp[prev..new_split]);
        num_start = new_split + 2;
        len = 0;
        for (i, split) in memchr_iter(b' ', &inp[new_split + 2..new_row])
            .chain(std::iter::once(new_row - new_split - 2))
            .enumerate()
        {
            buf_ptr
                .add(i)
                .write(fast_parseu64(&inp[num_start..new_split + 2 + split]));
            num_start = new_split + split + 3;
            len += 1;
        }
        prev = new_row + 1;

        if consume_test_mul(test, &buf[..len]) || consume_test_add(test, &buf[..len]) {
            ok_cnt += test;
        }
    }

    ok_cnt
}

fn consume_test_mul2(test: u64, buf: &[u64]) -> bool {
    if buf.len() == 1 {
        test == buf[0]
    } else if test % buf[buf.len() - 1] != 0 {
        false
    } else {
        let last = buf.len() - 1;
        let rem = test / buf[last];
        consume_test_mul2(rem, &buf[..last])
            || consume_test_add2(rem, &buf[..last])
            || consume_test_con(rem, &buf[..last])
    }
}

fn consume_test_add2(test: u64, buf: &[u64]) -> bool {
    if buf.len() == 1 {
        test == buf[0]
    } else if test < buf[buf.len() - 1] {
        false
    } else {
        let last = buf.len() - 1;
        let rem = test - buf[last];
        consume_test_mul2(rem, &buf[..last])
            || consume_test_add2(rem, &buf[..last])
            || consume_test_con(rem, &buf[..last])
    }
}

fn consume_test_con(test: u64, buf: &[u64]) -> bool {
    let ints = 10u64.pow(buf[buf.len() - 1].ilog10() + 1);
    if buf.len() == 1 {
        test == buf[0]
    } else if test % ints != buf[buf.len() - 1] {
        false
    } else {
        let last = buf.len() - 1;
        let rem = test / ints;
        consume_test_mul2(rem, &buf[..last])
            || consume_test_add2(rem, &buf[..last])
            || consume_test_con(rem, &buf[..last])
    }
}

unsafe fn part2_inner_opt(inp: &[u8]) -> u64 {
    let mut prev: usize = 0;
    let mut num_start;
    let mut buf: [u64; 20] = [0; 20];
    let mut len: usize;
    let mut ok_cnt = 0u64;
    let buf_ptr = buf.as_mut_ptr();
    for (new_row, new_split) in memchr_iter(b'\n', inp).zip(memchr_iter(b':', inp)) {
        let test = fast_parseu64(&inp[prev..new_split]);
        num_start = new_split + 2;
        len = 0;
        for (i, split) in memchr_iter(b' ', &inp[new_split + 2..new_row])
            .chain(std::iter::once(new_row - new_split - 2))
            .enumerate()
        {
            buf_ptr
                .add(i)
                .write(fast_parseu64(&inp[num_start..new_split + 2 + split]));
            num_start = new_split + split + 3;
            len += 1;
        }
        prev = new_row + 1;

        if consume_test_mul2(test, &buf[..len])
        || consume_test_add2(test, &buf[..len])
        || consume_test_con(test, &buf[..len])
        {
            ok_cnt += test;
        }
    }

    ok_cnt
}

#[aoc(day7, part1, opt)]
pub fn part1(inp: &str) -> u64 {
    unsafe { part1_inner_opt(inp.as_bytes()) }
}

#[aoc(day7, part2, opt)]
pub fn part2(inp: &str) -> u64 {
    unsafe { part2_inner_opt(inp.as_bytes()) }
}

