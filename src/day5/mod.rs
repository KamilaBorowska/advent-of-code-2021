use crate::Solution;
use std::collections::HashMap;
use std::error::Error;
use std::ops::RangeInclusive;

type Point = (u16, u16);

fn from_point(a: &str) -> Result<Point, Box<dyn Error>> {
    let (a, b) = a.split_once(',').ok_or("No comma found")?;
    Ok((a.parse()?, b.parse()?))
}

fn get_line(line: &str) -> Result<[Point; 2], Box<dyn Error>> {
    let (a, b) = line.split_once(" -> ").ok_or("No arrow found")?;
    let p1 = from_point(a)?;
    let p2 = from_point(b)?;
    Ok([p1, p2])
}

fn range(a: u16, b: u16) -> RangeInclusive<u16> {
    a.min(b)..=a.max(b)
}

fn abs_sub(a: u16, b: u16) -> u16 {
    a.max(b) - a.min(b)
}

fn insert_range(map: &mut HashMap<Point, bool>, iter: impl IntoIterator<Item = Point>) {
    for p in iter {
        map.entry(p).and_modify(|v| *v = true).or_insert(false);
    }
}

pub(super) const DAY5: Solution = Solution {
    part1: |input| {
        let mut map = HashMap::new();
        for line in input.lines() {
            let [(x1, y1), (x2, y2)] = get_line(line)?;
            if x1 == x2 {
                insert_range(&mut map, range(y1, y2).map(|y| (x1, y)));
            } else if y1 == y2 {
                insert_range(&mut map, range(x1, x2).map(|x| (x, y1)));
            }
        }
        Ok(map
            .values()
            .filter(|&&overlaps| overlaps)
            .count()
            .to_string())
    },
    part2: |input| {
        let mut map = HashMap::new();
        for line in input.lines() {
            let [(x1, y1), (x2, y2)] = get_line(line)?;
            if x1 == x2 {
                insert_range(&mut map, range(y1, y2).map(|y| (x1, y)));
            } else if y1 == y2 {
                insert_range(&mut map, range(x1, x2).map(|x| (x, y1)));
            } else if abs_sub(x1, x2) == abs_sub(y1, y2) {
                let range = 0..=abs_sub(x1, x2);
                if (x1 > x2) ^ (y1 > y2) {
                    insert_range(&mut map, range.map(|i| (x1.max(x2) - i, y1.min(y2) + i)));
                } else {
                    insert_range(&mut map, range.map(|i| (x1.min(x2) + i, y1.min(y2) + i)));
                }
            }
        }
        Ok(map
            .values()
            .filter(|&&overlaps| overlaps)
            .count()
            .to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY5.part1,
        example: lines!(
            "0,9 -> 5,9"
            "8,0 -> 0,8"
            "9,4 -> 3,4"
            "2,2 -> 2,1"
            "7,0 -> 7,4"
            "6,4 -> 2,0"
            "0,9 -> 2,9"
            "3,4 -> 1,4"
            "0,0 -> 8,8"
            "5,5 -> 8,2"
        ) => 5,
        input: 5690,
    );
    test!(
        DAY5.part2,
        example: lines!(
            "0,9 -> 5,9"
            "8,0 -> 0,8"
            "9,4 -> 3,4"
            "2,2 -> 2,1"
            "7,0 -> 7,4"
            "6,4 -> 2,0"
            "0,9 -> 2,9"
            "3,4 -> 1,4"
            "0,0 -> 8,8"
            "5,5 -> 8,2"
        ) => 12,
        input: 17741,
    );
}
