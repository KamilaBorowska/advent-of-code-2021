use crate::Solution;
use std::collections::HashMap;
use std::error::Error;

fn into_counter(i: impl Iterator<Item = [u8; 2]>) -> HashMap<[u8; 2], u64> {
    let mut map = HashMap::new();
    for elem in i {
        *map.entry(elem).or_insert(0) += 1;
    }
    map
}

fn run(input: &str, steps: u32) -> Result<String, Box<dyn Error>> {
    let mut lines = input.lines();
    let first_line = lines.next().ok_or("Expected polymer template")?.as_bytes();
    let mut polymer = into_counter(first_line.windows(2).map(|w| [w[0], w[1]]));
    let &first = first_line.get(0).ok_or("Expected first element")?;
    let mut rules = HashMap::new();
    if lines.next() != Some("") {
        return Err("Expected an empty line".into());
    }
    for rule in lines {
        if let [first, second, b' ', b'-', b'>', b' ', to] = *rule.as_bytes() {
            rules.insert([first, second], to);
        } else {
            return Err("Invalid rule pattern".into());
        }
    }
    for _ in 0..steps {
        let mut new_polymer = HashMap::new();
        for (pair @ [first, second], count) in polymer {
            let to = rules[&pair];
            *new_polymer.entry([first, to]).or_insert(0) += count;
            *new_polymer.entry([to, second]).or_insert(0) += count;
        }
        polymer = new_polymer;
    }
    let mut element_counts = HashMap::new();
    for ([_, b], count) in polymer {
        *element_counts.entry(b).or_insert(0) += count;
    }
    *element_counts.entry(first).or_insert(0) += 1;
    let min = element_counts
        .values()
        .min()
        .ok_or("Expected minimum value")?;
    let max = element_counts
        .values()
        .max()
        .ok_or("Expected maximum value")?;
    Ok((max - min).to_string())
}

pub(super) const DAY14: Solution = Solution {
    part1: |input| run(input, 10),
    part2: |input| run(input, 40),
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY14.part1,
        example: lines!(
            "NNCB"
            ""
            "CH -> B"
            "HH -> N"
            "CB -> H"
            "NH -> C"
            "HB -> C"
            "HC -> B"
            "HN -> C"
            "NN -> C"
            "BH -> H"
            "NC -> B"
            "NB -> B"
            "BN -> B"
            "BB -> N"
            "BC -> B"
            "CC -> N"
            "CN -> C"
        ) => 1588,
        input: 3118,
    );
    test!(
        DAY14.part2,
        example: lines!(
            "NNCB"
            ""
            "CH -> B"
            "HH -> N"
            "CB -> H"
            "NH -> C"
            "HB -> C"
            "HC -> B"
            "HN -> C"
            "NN -> C"
            "BH -> H"
            "NC -> B"
            "NB -> B"
            "BN -> B"
            "BB -> N"
            "BC -> B"
            "CC -> N"
            "CN -> C"
        ) => 2188189693529,
        input: 4332887448171,
    );
}
