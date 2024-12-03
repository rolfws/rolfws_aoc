#![allow(unused)]

use std::time::Instant;
fn main() {
    let inp = aoc24::load_day_bytes(2, false);
    let runs = 20000;
    let t = Instant::now();
    for _ in 0..runs {
        aoc24::day2::part2(&inp);
    }
    println!("{:?}", t.elapsed() / runs);
    // let r = aoc24::day2::part1(&inp);
    // let r = aoc24::day2::part2(&inp);
    // println!("{r}");
}