#![allow(unused)]

use std::time::Instant;
fn main() {
    let mut out = Vec::<(isize, isize, u16)>::new();
    for r in 1..21 {
        let mut cur = (-r, 0, r as u16);
        // (-x, 0)
        while cur.0 < 0 {
            out.push(cur);
            cur.0 += 1;
            cur.1 += 1;
        }
        // (0, x)
        while cur.1 > 0 {
            out.push(cur);
            cur.0 += 1;
            cur.1 -= 1;
        }
        // (x, 0)
        // while cur.0 > 0 {
        //     out.push(cur);
        //     cur.0 -= 1;
        //     cur.1 -= 1;
        // }
        // // (0, -x)
        // while cur.1 < 0 {
        //     out.push(cur);
        //     cur.0 -= 1;
        //     cur.1 += 1;
        // }
    }
    println!("{:?}", out)
    // let inp = aoc24::load_day(19, false);
    // let runs = 1000;
    // let t = Instant::now();
    // for _ in 0..runs {
    //     let r = aoc24::day19::part2(&inp);
    // }
    // println!("{:?}", t.elapsed() / runs);

    // let inp = aoc24::load_day(14, false);
    // let r = aoc24::day14::part2(&inp);
    // println!("{r}");

    // let inp = aoc24::load_day(4, false);
    // let r = aoc24::day4::part1(&inp);
    // println!("{r}");

    // let r = aoc24::day2::part2(&inp);
    // println!("{r}");
}