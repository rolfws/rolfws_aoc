#![allow(unused)]

use std::time::Instant;
fn main() {
    // let inp = aoc24::load_day(6, false);
    // let runs = 300;
    // let t = Instant::now();
    // for _ in 0..runs {
    //     let r = aoc24::day6::part2(&inp);
    // }
    // println!("{:?}", t.elapsed() / runs);

    let inp = aoc24::load_day(7, true);
    let r = aoc24::day7::part2(&inp);
    println!("{r}");

    // let inp = aoc24::load_day(4, false);
    // let r = aoc24::day4::part1(&inp);
    // println!("{r}");

    // let r = aoc24::day2::part2(&inp);
    // println!("{r}");
}