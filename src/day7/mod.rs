use crate::Solution;
use std::error::Error;

// f needs to have one local minimum for value of mid
fn find_minima(input: &str, f: impl Fn(u16, u16) -> u32) -> Result<String, Box<dyn Error>> {
    let crabs = input
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<u16>, _>>()?;
    let mut min = *crabs.iter().min().unwrap_or(&0);
    let mut max = *crabs.iter().max().unwrap_or(&0);
    let distance_sum = |mid| crabs.iter().map(|&v| f(v, mid)).sum();
    let mut min_sum: u32 = distance_sum(min);
    let mut max_sum = distance_sum(max);
    loop {
        let mid = (min + max) / 2;
        if mid == min {
            return Ok(min_sum.min(max_sum).to_string());
        }
        let sum = distance_sum(mid);
        if max_sum > min_sum {
            max_sum = sum;
            max = mid;
        } else {
            min_sum = sum;
            min = mid;
        }
    }
}

pub(super) const DAY7: Solution = Solution {
    part1: |input| find_minima(input, |v, mid| u32::from(v.max(mid) - v.min(mid))),
    part2: |input| {
        find_minima(input, |v, mid| {
            let v = u32::from(v.max(mid) - v.min(mid));
            (v * v + v) / 2
        })
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY7.part1,
        example: "16,1,2,0,4,2,7,1,2,14" => 37,
        input: 342730,
    );
    test!(
        DAY7.part2,
        example: "16,1,2,0,4,2,7,1,2,14" => 168,
        input: 92335207,
    );
}
