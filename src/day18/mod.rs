use crate::Solution;

#[derive(Clone)]
enum SnailfishNumber {
    Pair(Box<[SnailfishNumber; 2]>),
    Literal(u8),
}

impl SnailfishNumber {
    fn parse(s: &str) -> Self {
        let (out, rest) = Self::parse_partial(s, '\0');
        assert_eq!(rest, "");
        out
    }

    fn parse_partial(s: &str, expected_separator: char) -> (Self, &str) {
        match s.strip_prefix('[') {
            Some(rest) => {
                let (a, rest) = Self::parse_partial(rest, ',');
                let (b, rest) = Self::parse_partial(&rest[1..], ']');
                (Self::Pair(Box::new([a, b])), &rest[1..])
            }
            None => {
                let index = s.find(expected_separator).unwrap();
                (Self::Literal(s[..index].parse().unwrap()), &s[index..])
            }
        }
    }

    fn reduce(mut self) -> Self {
        while self.explode(0, None, None) | self.split() {}
        self
    }

    fn explode(&mut self, depth: u8, left: Option<&mut Self>, right: Option<&mut Self>) -> bool {
        if let Self::Pair(pair) = self {
            if depth == 4 {
                let (a, b) = match **pair {
                    [Self::Literal(a), Self::Literal(b)] => (a, b),
                    _ => unreachable!(),
                };
                if let Some(left) = left {
                    *left.find(1) += a;
                }
                if let Some(right) = right {
                    *right.find(0) += b;
                }
                *self = Self::Literal(0);
                true
            } else {
                let [a, b] = &mut **pair;
                let a_exploded = a.explode(depth + 1, left, Some(b));
                let b_exploded = b.explode(depth + 1, Some(a), right);
                a_exploded || b_exploded
            }
        } else {
            false
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Self::Pair(pair) => {
                let [a, b] = &mut **pair;
                a.split() || b.split()
            }
            &mut Self::Literal(literal) => {
                if literal >= 10 {
                    *self = Self::Pair(Box::new([
                        Self::Literal(literal / 2),
                        Self::Literal((literal + 1) / 2),
                    ]));
                    true
                } else {
                    false
                }
            }
        }
    }

    fn find(&mut self, index: usize) -> &mut u8 {
        let mut number = self;
        loop {
            match number {
                Self::Pair(pair) => number = &mut pair[index],
                Self::Literal(l) => break l,
            }
        }
    }

    fn magnitude(&self) -> u32 {
        match self {
            Self::Pair(pair) => {
                let [a, b] = &**pair;
                3 * a.magnitude() + 2 * b.magnitude()
            }
            &Self::Literal(l) => l.into(),
        }
    }
}

pub(super) const DAY18: Solution = Solution {
    part1: |input| {
        let number = input
            .lines()
            .map(SnailfishNumber::parse)
            .reduce(|a, b| SnailfishNumber::Pair(Box::new([a, b])).reduce())
            .unwrap();
        Ok(number.magnitude().to_string())
    },
    part2: |input| {
        let numbers: Vec<_> = input.lines().map(SnailfishNumber::parse).collect();
        let mut max_magnitude = 0;
        for a in &numbers {
            for b in &numbers {
                max_magnitude = SnailfishNumber::Pair(Box::new([a.clone(), b.clone()]))
                    .reduce()
                    .magnitude()
                    .max(max_magnitude);
            }
        }
        Ok(max_magnitude.to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY18.part1,
        example1: "[9,1]" => 29,
        example2: "[[9,1],[1,9]]" => 129,
        example3: "[[1,2],[[3,4],5]]" => 143,
        example4: "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]" => 1384,
        example5: "[[[[1,1],[2,2]],[3,3]],[4,4]]" => 445,
        example6: "[[[[3,0],[5,3]],[4,4]],[5,5]]" => 791,
        example7: "[[[[5,0],[7,4]],[5,5]],[6,6]]" => 1137,
        example8: "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]" => 3488,
        example9: lines!(
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]"
            "[[[5,[2,8]],4],[5,[[9,9],0]]]"
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]"
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]"
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]"
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]"
            "[[[[5,4],[7,7]],8],[[8,3],8]]"
            "[[9,3],[[9,9],[6,[4,9]]]]"
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]"
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
        ) => 4140,
        input: 3892,
    );
    test!(
        DAY18.part2,
        example: lines!(
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]"
            "[[[5,[2,8]],4],[5,[[9,9],0]]]"
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]"
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]"
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]"
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]"
            "[[[[5,4],[7,7]],8],[[8,3],8]]"
            "[[9,3],[[9,9],[6,[4,9]]]]"
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]"
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
        ) => 3993,
        input: 4909,
    );
}
