#![allow(unused)]

use std::time::Instant;
fn main() {
    let inp = aoc24::load_day(15, false);
    let runs = 5000;
    let t = Instant::now();
    for _ in 0..runs {
        let r = aoc24::day15::part1(&inp);
    }
    println!("{:?}", t.elapsed() / runs);

    // let inp = aoc24::load_day(14, false);
    // let r = aoc24::day14::part2(&inp);
    // println!("{r}");

    // let inp = aoc24::load_day(4, false);
    // let r = aoc24::day4::part1(&inp);
    // println!("{r}");

    // let r = aoc24::day2::part2(&inp);
    // println!("{r}");
}