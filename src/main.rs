#![allow(unused)]

use std::time::Instant;
fn main() {
    // let inp = aoc24::load_day_bytes(3, false);
    // let runs = 100000;
    // // let t = Instant::now();
    // // for _ in 0..runs {
    // //     let r = unsafe{aoc24::day3::part1(&inp)};
    // // }
    // // println!("{:?}", t.elapsed() / runs);

    // let t = Instant::now();
    // for _ in 0..runs {
    //     aoc24::day3::part2(&inp);
    // }
    // println!("{:?}", t.elapsed() / runs);
    // let r = aoc24::day3::part2(&inp);
    let inp = aoc24::load_day(2, false);
    let r = aoc24::day2::part1(&inp);
    println!("{r}");

    let r = aoc24::day2::part2(&inp);
    println!("{r}");
}