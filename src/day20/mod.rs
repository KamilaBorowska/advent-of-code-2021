use crate::Solution;
use std::collections::HashSet;

const CHECKED_PIXELS: [(i16, i16); 9] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn enhance(input: &str, steps: u32) -> String {
    let (pattern, image) = input.split_once("\n\n").unwrap();
    let pattern = pattern.as_bytes();
    let mut pixels = HashSet::new();
    for (y, row) in (0..).zip(image.lines()) {
        for (x, cell) in (0..).zip(row.chars()) {
            if cell == '#' {
                pixels.insert((x, y));
            }
        }
    }
    let mut oob = false;
    let mut min_x = pixels.iter().map(|p| p.0).min().unwrap();
    let mut max_x = pixels.iter().map(|p| p.0).max().unwrap();
    let mut min_y = pixels.iter().map(|p| p.1).min().unwrap();
    let mut max_y = pixels.iter().map(|p| p.1).max().unwrap();
    for _ in 0..steps {
        let mut new_pixels = HashSet::new();
        for x in min_x - 1..max_x + 2 {
            for y in min_y - 1..max_y + 2 {
                let mut index = 0;
                for (mx, my) in CHECKED_PIXELS {
                    index <<= 1;
                    let x = x + mx;
                    let y = y + my;
                    index |= usize::from(
                        if (min_x..=max_x).contains(&x) && (min_y..=max_y).contains(&y) {
                            pixels.contains(&(x, y))
                        } else {
                            oob
                        },
                    );
                }
                if pattern[index] == b'#' {
                    new_pixels.insert((x, y));
                }
            }
        }
        oob = pattern[if oob { 511 } else { 0 }] == b'#';
        pixels = new_pixels;
        min_x -= 1;
        max_x += 1;
        min_y -= 1;
        max_y += 1;
    }
    assert!(!oob);
    pixels.len().to_string()
}

pub(super) const DAY20: Solution = Solution {
    part1: |input| Ok(enhance(input, 2)),
    part2: |input| Ok(enhance(input, 50)),
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY20.part1,
        example: lines!(
            "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#"
            ""
            "#..#."
            "#...."
            "##..#"
            "..#.."
            "..###"
        ) => 35,
        input: 5663,
    );

    test!(
        DAY20.part2,
        example: lines!(
            "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#"
            ""
            "#..#."
            "#...."
            "##..#"
            "..#.."
            "..###"
        ) => 3351,
        input: 19638,
    );
}
