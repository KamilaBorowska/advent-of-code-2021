use crate::Solution;
use std::num::ParseIntError;

pub(super) const DAY1: Solution = Solution {
    part1: |input| {
        let (count, _) = input.lines().try_fold(
            (0, None),
            |(mut count, previous), line| -> Result<_, ParseIntError> {
                let number = line.parse::<u32>()?;
                if let Some(previous) = previous {
                    count += u32::from(number > previous);
                }
                Ok((count, Some(number)))
            },
        )?;
        Ok(count.to_string())
    },
    part2: |input| {
        let (count, _) = input.lines().try_fold(
            (0, [None; 3]),
            |(mut count, [a, b, c]), line| -> Result<_, ParseIntError> {
                let number = line.parse::<u32>()?;
                if let Some(a) = a {
                    count += u32::from(number > a);
                }
                Ok((count, [b, c, Some(number)]))
            },
        )?;
        Ok(count.to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY1.part1,

        empty: "" => 0,
        example: lines!(199 200 208 210 200 207 240 269 260 263) => 7,
        input: 1532,
    );
    test!(
        DAY1.part2,
        empty: "" => 0,
        example: lines!(199 200 208 210 200 207 240 269 260 263) => 5,
        input: 1571,
    );
}
