use std::fs;

pub fn load_day(day:usize, ex:bool) -> String {
    let fp = if ex {
        format!("./inputs/examples/day{day}.txt")
    } else {
        format!("./inputs/full/day{day}.txt")
    };

    fs::read_to_string(fp).unwrap()
}

pub fn load_day_bytes(day:usize, ex:bool) -> Vec<u8> {
    let fp = if ex {
        format!("./inputs/examples/day{day}.txt")
    } else {
        format!("./inputs/full/day{day}.txt")
    };

    fs::read(fp).unwrap()
}

// pub(crate) fn line_num_len(inp:&[u8]) -> (usize, usize) {
//     let line_len = memchr(b'\n', inp).expect("input") + 1; // len is including
//     let mut line_num = inp.len() / line_len;
//     if inp.len() % line_len != 0 {
//         line_num += 1;
//     }
//     (line_num, line_len)
// }