use crate::Solution;
use std::error::Error;

fn rating(mut inputs: Vec<&str>, invert: bool) -> Result<u32, Box<dyn Error>> {
    for i in 0.. {
        if inputs.len() <= 1 {
            let first = inputs.first().ok_or("Missing inputs")?;
            return Ok(u32::from_str_radix(first, 2)?);
        }
        let mut frequency = 0;
        for line in &inputs {
            let bit = line
                .as_bytes()
                .get(i)
                .ok_or("Not enough bits to determine oxygen rating")?;
            frequency += match bit {
                b'0' => 0,
                b'1' => 1,
                _ => return Err("Unrecognized bit".into()),
            }
        }
        let expected = if (frequency * 2 >= inputs.len()) ^ invert {
            b'1'
        } else {
            b'0'
        };
        inputs.retain(|line| *line.as_bytes().get(i).unwrap() == expected);
    }
    unreachable!()
}

pub(super) const DAY3: Solution = Solution {
    part1: |input| {
        let length = input.find('\n').unwrap_or_else(|| input.len());
        let mut frequencies = vec![0; length];
        let mut count = 0;
        for line in input.lines() {
            count += 1;
            for (bit, frequency) in line.bytes().zip(&mut frequencies) {
                *frequency += match bit {
                    b'0' => 0,
                    b'1' => 1,
                    _ => return Err("Unrecognized bit".into()),
                };
            }
        }
        let gamma = frequencies
            .into_iter()
            .fold(0, |a, b| (a << 1) | u32::from(b > count / 2));
        let epsilon = gamma ^ ((1 << length) - 1);
        Ok((gamma * epsilon).to_string())
    },
    part2: |input| {
        let inputs: Vec<&str> = input.lines().collect();
        Ok((rating(inputs.clone(), false)? * rating(inputs, true)?).to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY3.part1,
        empty: "" => 0,
        example: lines!(
            "00100"
            "11110"
            "10110"
            "10111"
            "10101"
            "01111"
            "00111"
            "11100"
            "10000"
            "11001"
            "00010"
            "01010"
        ) => 198,
        input: 852500,
    );
    test!(
        DAY3.part2,
        example: lines!(
            "00100"
            "11110"
            "10110"
            "10111"
            "10101"
            "01111"
            "00111"
            "11100"
            "10000"
            "11001"
            "00010"
            "01010"
        ) => 230,
        input: 1007985,
    );
}
