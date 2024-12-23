mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod puzzle;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
pub use puzzle::Puzzle;

pub mod prelude {
    pub use super::Result;
    pub use crate::puzzle::Puzzle;
    pub use std::fs::File;
    pub use std::io::BufRead;
}

fn main() -> Result<()> {
    let day = std::env::args()
        .nth(1)
        .ok_or("specify a day!")?
        .parse::<u32>()?;

    match day {
        1 => day01(),
        2 => day02(),
        3 => day03(),
        4 => day04(),
        5 => day05(),
        6 => day06(),
        7 => day07(),
        8 => day08(),
        9 => day09(),
        10 => day10(),
        11 => day11(),
        12 => day12(),
        13 => day13(),
        14 => day14(),
        15 => day15(),
        16 => day16(),
        17 => day17(),
        18 => day18(),
        19 => day19(),
        20 => day20(),
        21 => day21(),
        22 => day22(),
        23 => day23(),
        24 => day24(),
        25 => day25(),
        _ => Err("invalid day".into()),
    }
}

fn day01() -> Result<()> {
    let mut a = day01::A;
    println!("part A: {}", a.solution()?);

    let mut b = day01::B;
    println!("part B: {}", b.solution()?);

    Ok(())
}

fn day02() -> Result<()> {
    let mut a = day02::A;
    println!("part A: {}", a.solution()?);

    let mut b = day02::B;
    println!("part B: {}", b.solution()?);

    Ok(())
}

fn day03() -> Result<()> {
    let mut a = day03::A;
    println!("part A: {}", a.solution()?);

    let mut b = day03::B;
    println!("part B: {}", b.solution()?);

    Ok(())
}

fn day04() -> Result<()> {
    let mut a = day04::A;
    println!("part A: {}", a.solution()?);

    let mut b = day04::B;
    println!("part B: {}", b.solution()?);

    Ok(())
}

fn day05() -> Result<()> {
    let mut a = day05::A;
    println!("part A: {}", a.solution()?);

    let mut b = day05::B;
    println!("part B: {}", b.solution()?);

    Ok(())
}

fn day06() -> Result<()> {
    let mut a = day06::A;
    println!("part A: {}", a.solution()?);

    let mut b = day06::B;
    println!("part B: {}", b.solution()?);

    Ok(())
}

fn day07() -> Result<()> {
    let mut a = day07::A;
    println!("part A: {}", a.solution()?);

    let mut b = day07::B;
    println!("part B: {}", b.solution()?);

    Ok(())
}

fn day08() -> Result<()> {
    let mut a = day08::A;
    println!("part A: {}", a.solution()?);

    let mut b = day08::B;
    println!("part B: {}", b.solution()?);

    Ok(())
}

fn day09() -> Result<()> {
    todo!()
}

fn day10() -> Result<()> {
    todo!()
}

fn day11() -> Result<()> {
    todo!()
}

fn day12() -> Result<()> {
    todo!()
}

fn day13() -> Result<()> {
    todo!()
}

fn day14() -> Result<()> {
    todo!()
}

fn day15() -> Result<()> {
    todo!()
}

fn day16() -> Result<()> {
    todo!()
}

fn day17() -> Result<()> {
    todo!()
}

fn day18() -> Result<()> {
    todo!()
}

fn day19() -> Result<()> {
    todo!()
}

fn day20() -> Result<()> {
    todo!()
}

fn day21() -> Result<()> {
    todo!()
}

fn day22() -> Result<()> {
    todo!()
}

fn day23() -> Result<()> {
    todo!()
}

fn day24() -> Result<()> {
    todo!()
}

fn day25() -> Result<()> {
    todo!()
}
