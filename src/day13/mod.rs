use crate::Solution;
use std::collections::HashSet;
use std::error::Error;

fn parse_points<'a>(
    lines: impl Iterator<Item = &'a str>,
) -> Result<HashSet<(u16, u16)>, Box<dyn Error>> {
    let mut points = HashSet::new();
    for line in lines {
        if line.is_empty() {
            break;
        }
        let (x, y) = line.split_once(',').ok_or("Missing comma")?;
        points.insert((x.parse()?, y.parse()?));
    }
    Ok(points)
}

fn fold(points: HashSet<(u16, u16)>, fold: &str) -> Result<HashSet<(u16, u16)>, Box<dyn Error>> {
    let fold = fold
        .strip_prefix("fold along ")
        .ok_or("Fold not prefixed")?;
    let (coordinate, value) = fold.split_once('=').ok_or("No = in fold")?;
    let value = value.parse()?;
    let use_x = match coordinate {
        "x" => true,
        "y" => false,
        _ => return Err("Unrecognized coordinate type".into()),
    };
    let points = points
        .into_iter()
        .map(|(mut x, mut y)| {
            let affected = if use_x { &mut x } else { &mut y };
            if *affected > value {
                *affected = 2 * value - *affected;
            }
            (x, y)
        })
        .collect();
    Ok(points)
}

pub(super) const DAY13: Solution = Solution {
    part1: |input| {
        let mut lines = input.lines();
        let points = parse_points(&mut lines)?;
        let points = fold(points, lines.next().ok_or("Expected a fold")?)?;
        Ok(points.len().to_string())
    },
    part2: |input| {
        let mut lines = input.lines();
        let points = parse_points(&mut lines)?;
        let points = lines.try_fold(points, fold)?;
        let max_x = points.iter().map(|p| p.0).max().unwrap_or(0);
        let max_y = points.iter().map(|p| p.1).max().unwrap_or(0);
        let mut output = String::new();
        for y in 0..=max_y {
            output.push('\n');
            for x in 0..=max_x {
                output.push(if points.contains(&(x, y)) { '█' } else { ' ' })
            }
        }
        Ok(output)
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY13.part1,
        example: lines!(
            "6,10"
            "0,14"
            "9,10"
            "0,3"
            "10,4"
            "4,11"
            "6,0"
            "6,12"
            "4,1"
            "0,13"
            "10,12"
            "3,4"
            "3,0"
            "8,4"
            "1,10"
            "2,14"
            "8,10"
            "9,0"
            ""
            "fold along y=7"
            "fold along x=5"
        ) => 17,
        input: 827,
    );
}
