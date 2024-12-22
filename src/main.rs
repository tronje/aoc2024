mod puzzle;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

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
    todo!()
}

fn day02() -> Result<()> {
    todo!()
}

fn day03() -> Result<()> {
    todo!()
}

fn day04() -> Result<()> {
    todo!()
}

fn day05() -> Result<()> {
    todo!()
}

fn day06() -> Result<()> {
    todo!()
}

fn day07() -> Result<()> {
    todo!()
}

fn day08() -> Result<()> {
    todo!()
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