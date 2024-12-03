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