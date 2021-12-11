use crate::Solution;
use std::error::Error;

fn parse_line(line: &str) -> Result<[u8; 10], Box<dyn Error>> {
    let mut chars = line.chars();
    let mut next = || -> Result<u8, Box<dyn Error>> {
        Ok(chars
            .next()
            .ok_or("Missing character")?
            .to_digit(16)
            .ok_or("Not a digit")?
            .try_into()?)
    };
    Ok([
        next()?,
        next()?,
        next()?,
        next()?,
        next()?,
        next()?,
        next()?,
        next()?,
        next()?,
        next()?,
    ])
}

fn parse_octopuses(input: &str) -> Result<[[u8; 10]; 10], Box<dyn Error>> {
    let mut lines = input.lines();
    let mut next = || parse_line(lines.next().ok_or("Missing line")?);
    Ok([
        next()?,
        next()?,
        next()?,
        next()?,
        next()?,
        next()?,
        next()?,
        next()?,
        next()?,
        next()?,
    ])
}

fn neighbours(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    [
        (!0, !0),
        (!0, 0),
        (!0, 1),
        (0, !0),
        (0, 1),
        (1, !0),
        (1, 0),
        (1, 1),
    ]
    .into_iter()
    .map(move |(mod_x, mod_y)| (x.wrapping_add(mod_x), y.wrapping_add(mod_y)))
}

fn flash(octopuses: &mut [[u8; 10]; 10], x: usize, y: usize) {
    for (x, y) in neighbours(x, y) {
        if let Some(octopus) = octopuses.get_mut(y).and_then(|row| row.get_mut(x)) {
            if *octopus != 0 {
                *octopus += 1;
                if *octopus > 9 {
                    *octopus = 0;
                    flash(octopuses, x, y);
                }
            }
        }
    }
}

fn run_step(octopuses: &mut [[u8; 10]; 10]) {
    for octopus in octopuses.iter_mut().flatten() {
        *octopus += 1;
    }
    for y in 0..10 {
        for x in 0..10 {
            if octopuses[y][x] > 9 {
                octopuses[y][x] = 0;
                flash(octopuses, x, y);
            }
        }
    }
}

pub(super) const DAY11: Solution = Solution {
    part1: |input| {
        let mut octopuses = parse_octopuses(input)?;
        let mut flashes = 0;
        for _ in 0..100 {
            run_step(&mut octopuses);
            flashes += octopuses
                .iter()
                .flatten()
                .filter(|&&octopus| octopus == 0)
                .count();
        }
        Ok(flashes.to_string())
    },
    part2: |input| {
        let mut octopuses = parse_octopuses(input)?;
        for i in 1.. {
            run_step(&mut octopuses);
            if octopuses.iter().flatten().all(|&octopus| octopus == 0) {
                return Ok(i.to_string());
            }
        }
        unreachable!()
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY11.part1,
        example1: lines!(
            "5483143223"
            "2745854711"
            "5264556173"
            "6141336146"
            "6357385478"
            "4167524645"
            "2176841721"
            "6882881134"
            "4846848554"
            "5283751526"
        ) => 1656,
        input: 1755,
    );
    test!(
        DAY11.part2,
        example1: lines!(
            "5483143223"
            "2745854711"
            "5264556173"
            "6141336146"
            "6357385478"
            "4167524645"
            "2176841721"
            "6882881134"
            "4846848554"
            "5283751526"
        ) => 195,
        input: 212,
    );
}
