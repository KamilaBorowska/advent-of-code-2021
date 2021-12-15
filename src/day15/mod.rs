use crate::Solution;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::error::Error;

fn find_path(map: &mut [Vec<u8>]) -> Result<String, Box<dyn Error>> {
    let mut min_heap = BinaryHeap::new();
    min_heap.push((Reverse(0), (0, 0)));
    let max_x = map.first().ok_or("Empty map")?.len() - 1;
    let max_y = map.len() - 1;
    while let Some((Reverse(cost), (x, y))) = min_heap.pop() {
        if x == max_x && y == max_y {
            return Ok(cost.to_string());
        }
        let neighbours = [(!0, 0), (0, !0), (1, 0), (0, 1)]
            .into_iter()
            .map(|(mod_x, mod_y)| (x.wrapping_add(mod_x), y.wrapping_add(mod_y)))
            .filter_map(|p @ (x, y)| {
                let additional_cost = map.get_mut(y)?.get_mut(x)?;
                if *additional_cost == 0 {
                    return None;
                }
                let value = *additional_cost;
                *additional_cost = 0;
                Some((Reverse(cost + u16::from(value)), p))
            });
        min_heap.extend(neighbours);
    }
    Err("Unable to find a path".into())
}

pub(super) const DAY15: Solution = Solution {
    part1: |input| {
        let mut map = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| -> Result<u8, Box<dyn Error>> {
                        Ok(c.to_digit(10)
                            .ok_or("Unable to parse a digit")?
                            .try_into()?)
                    })
                    .collect()
            })
            .collect::<Result<Vec<Vec<_>>, _>>()?;
        find_path(&mut map)
    },
    part2: |input| {
        let mut map = (0..5)
            .flat_map(|i| {
                input.lines().map(move |line| {
                    (0..5)
                        .flat_map(|j| {
                            line.chars().map(move |c| -> Result<u8, Box<dyn Error>> {
                                let parsed = c.to_digit(10).ok_or("Unable to parse a digit")?;
                                let mut value = u8::try_from(parsed)? + i + j;
                                while value > 9 {
                                    value -= 9;
                                }
                                Ok(value)
                            })
                        })
                        .collect()
                })
            })
            .collect::<Result<Vec<Vec<_>>, _>>()?;
        find_path(&mut map)
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY15.part1,
        example: lines!(
            "1163751742"
            "1381373672"
            "2136511328"
            "3694931569"
            "7463417111"
            "1319128137"
            "1359912421"
            "3125421639"
            "1293138521"
            "2311944581"
        ) => 40,
        input: 390,
    );
    test!(
        DAY15.part2,
        example: lines!(
            "1163751742"
            "1381373672"
            "2136511328"
            "3694931569"
            "7463417111"
            "1319128137"
            "1359912421"
            "3125421639"
            "1293138521"
            "2311944581"
        ) => 315,
        input: 2814,
    );
}
