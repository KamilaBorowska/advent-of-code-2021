use crate::Solution;
use std::error::Error;

fn run<const DAYS: u32>(input: &str) -> Result<String, Box<dyn Error>> {
    let mut counts = [0; 9];
    for fish in input.trim().split(',').map(str::parse::<u8>) {
        *counts
            .get_mut(usize::from(fish?))
            .ok_or("Got value greater than 8")? += 1;
    }
    for _ in 0..DAYS {
        let mut new_counts = [0; 9];
        for (i, count) in counts.iter().enumerate() {
            if i == 0 {
                new_counts[6] += count;
                new_counts[8] += count;
            } else {
                new_counts[i - 1] += count;
            }
        }
        counts = new_counts;
    }
    Ok(counts.iter().sum::<u64>().to_string())
}

pub(super) const DAY6: Solution = Solution {
    part1: run::<80>,
    part2: run::<256>,
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY6.part1,
        example: "3,4,3,1,2" => 5934,
        input: 372300,
    );
    test!(
        DAY6.part2,
        example: "3,4,3,1,2" => 26984457539,
        input: 1675781200288,
    );
}
