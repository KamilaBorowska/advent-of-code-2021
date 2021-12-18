use crate::Solution;
use std::collections::BinaryHeap;

fn neighbours(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    [(!0, 0), (1, 0), (0, !0), (0, 1)]
        .into_iter()
        .map(move |(x_mod, y_mod)| (x.wrapping_add(x_mod), y.wrapping_add(y_mod)))
}

fn is_lowest_point(matrix: &[impl AsRef<[u8]>], x: usize, y: usize) -> bool {
    let height = matrix[y].as_ref()[x];
    neighbours(x, y)
        .filter_map(|(x, y)| matrix.get(y)?.as_ref().get(x))
        .all(|&other_height| other_height > height)
}

fn recurse_basin(matrix: &mut [&mut [u8]], x: usize, y: usize) -> usize {
    matrix[y][x] = b'9';
    let neighbours: usize = neighbours(x, y)
        .map(|(x, y)| {
            if let Some(&other_height) = matrix.get(y).and_then(|row| row.get(x)) {
                if other_height != b'9' {
                    return recurse_basin(matrix, x, y);
                }
            }
            0
        })
        .sum();
    neighbours + 1
}

pub(super) const DAY9: Solution = Solution {
    part1: |input| {
        let matrix = input.lines().map(str::as_bytes).collect::<Vec<_>>();
        let mut sum = 0;
        for (y, row) in matrix.iter().enumerate() {
            for (x, height) in row.iter().enumerate() {
                if is_lowest_point(&matrix, x, y) {
                    sum += char::from(*height).to_digit(10).ok_or("Expected a digit")? + 1;
                }
            }
        }
        Ok(sum.to_string())
    },
    part2: |input| {
        let mut input = input.as_bytes().to_vec();
        let mut matrix = input.split_mut(|&b| b == b'\n').collect::<Vec<_>>();
        let mut sizes = BinaryHeap::new();
        for y in 0..matrix.len() {
            for x in 0..matrix[y].len() {
                if matrix[y][x] != b'9' {
                    sizes.push(recurse_basin(&mut matrix, x, y));
                }
            }
        }
        Ok((0..3)
            .map_while(|_| sizes.pop())
            .product::<usize>()
            .to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY9.part1,
        example: lines!(
            2199943210
            3987894921
            9856789892
            8767896789
            9899965678
        ) => 15,
        input: 468,
    );
    test!(
        DAY9.part2,
        example: lines!(
            2199943210
            3987894921
            9856789892
            8767896789
            9899965678
        ) => 1134,
        input: 1280496,
    );
}
