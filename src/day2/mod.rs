use crate::Solution;

pub(super) const DAY2: Solution = Solution {
    part1: |input| {
        let mut x = 0;
        let mut depth = 0;
        for line in input.lines() {
            let (action, count) = line
                .split_once(' ')
                .ok_or("Line doesn't contain two parts")?;
            let count = count.parse::<i32>()?;
            match action {
                "forward" => x += count,
                "down" => depth += count,
                "up" => depth -= count,
                _ => return Err("Unrecognized action".into()),
            }
        }
        Ok((x * depth).to_string())
    },
    part2: |input| {
        let mut x = 0;
        let mut aim = 0;
        let mut depth = 0;
        for line in input.lines() {
            let (action, count) = line
                .split_once(' ')
                .ok_or("Line doesn't contain two parts")?;
            let count = count.parse::<i32>()?;
            match action {
                "forward" => {
                    x += count;
                    depth += aim * count;
                }
                "down" => aim += count,
                "up" => aim -= count,
                _ => return Err("Unrecognized action".into()),
            }
        }
        Ok((x * depth).to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY2.part1,
        empty: "" => 0,
        example: lines!("forward 5" "down 5" "forward 8" "up 3" "down 8" "forward 2") => 150,
        input: 1855814,
    );
    test!(
        DAY2.part2,
        empty: "" => 0,
        example: lines!("forward 5" "down 5" "forward 8" "up 3" "down 8" "forward 2") => 900,
        input: 1845455714,
    );
}
