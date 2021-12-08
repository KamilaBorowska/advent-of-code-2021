use crate::Solution;

fn into_bits(b: &str) -> Result<u8, &'static str> {
    let mut sum = 0;
    for c in b.chars() {
        sum |= match c {
            'a' => 0x40,
            'b' => 0x20,
            'c' => 0x10,
            'd' => 0x08,
            'e' => 0x04,
            'f' => 0x02,
            'g' => 0x01,
            _ => return Err("Unrecognized character"),
        };
    }
    Ok(sum)
}

fn digit(bits: u8) -> Result<u8, &'static str> {
    Ok(match bits {
        0b1110111 => 0,
        0b0010010 => 1,
        0b1011101 => 2,
        0b1011011 => 3,
        0b0111010 => 4,
        0b1101011 => 5,
        0b1101111 => 6,
        0b1010010 => 7,
        0b1111111 => 8,
        0b1111011 => 9,
        _ => return Err("Unrecognized bit"),
    })
}

pub(super) const DAY8: Solution = Solution {
    part1: |input| {
        let mut count = 0;
        for line in input.lines() {
            let outputs = line.split_once(" | ").ok_or("No delimiter found")?.1;
            count += outputs
                .split_whitespace()
                .filter(|output| [2, 3, 4, 7].contains(&output.len()))
                .count();
        }
        Ok(count.to_string())
    },
    part2: |input| {
        let mut output = 0;
        for line in input.lines() {
            let (patterns, outputs) = line.split_once(" | ").ok_or("No delimiter found")?;
            let mut possible_positions = [0x7F; 7];
            let into_bits = |part: &str| {
                part.split_whitespace()
                    .map(into_bits)
                    .collect::<Result<Vec<u8>, _>>()
            };
            let mut patterns = into_bits(patterns)?;
            let mut outputs = into_bits(outputs)?;
            for bits in patterns.iter().chain(&outputs) {
                let (guaranteed, remove) = match bits.count_ones() {
                    2 => (0b0100100, true),
                    3 => (0b0100101, true),
                    4 => (0b0101110, true),
                    5 => (0b1001001, false),
                    6 => (0b1100011, false),
                    7 => (0b1111111, false),
                    _ => return Err("Unrecognized number of bits".into()),
                };
                for (i, possible) in possible_positions.iter_mut().enumerate() {
                    if guaranteed & 1 << i != 0 {
                        *possible &= bits;
                    } else if remove {
                        *possible &= !bits;
                    }
                }
            }
            for i in 0..possible_positions.len() {
                let current = possible_positions[i];
                if current.count_ones() == 1 {
                    for (j, possible_position) in possible_positions.iter_mut().enumerate() {
                        if i != j {
                            *possible_position &= !current;
                        }
                    }
                }
            }
            for v in patterns.iter_mut().chain(&mut outputs) {
                let mut new_v = 0;
                for possible_position in possible_positions {
                    new_v <<= 1;
                    if *v & possible_position != 0 {
                        new_v |= 1;
                    }
                }
                *v = digit(new_v)?;
            }
            for (i, &output_digit) in (0..).zip(outputs.iter().rev()) {
                output += u32::from(output_digit) * 10u32.pow(i);
            }
        }
        Ok(output.to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY8.part1,
        empty: "" => 0,
        example1: "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf" => 0,
        example2: lines!(
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc"
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg"
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb"
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea"
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb"
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe"
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef"
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb"
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
        ) => 26,
        input: 452,
    );
    test!(
        DAY8.part2,
        empty: "" => 0,
        example: lines!(
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc"
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg"
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb"
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea"
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb"
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe"
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef"
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb"
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
        ) => 61229,
        input: 1096964,
    );
}
