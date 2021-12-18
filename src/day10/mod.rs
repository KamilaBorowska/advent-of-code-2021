use crate::Solution;

pub(super) const DAY10: Solution = Solution {
    part1: |input| {
        let mut score = 0;
        for line in input.lines() {
            let mut stack = Vec::new();
            for c in line.bytes() {
                let (expected, char_score) = match c {
                    b'(' | b'[' | b'{' | b'<' => {
                        stack.push(c);
                        continue;
                    }
                    b')' => (b'(', 3),
                    b']' => (b'[', 57),
                    b'}' => (b'{', 1197),
                    b'>' => (b'<', 25137),
                    _ => return Err("Unexpected character".into()),
                };
                if stack.pop() != Some(expected) {
                    score += char_score;
                }
            }
        }
        Ok(score.to_string())
    },
    part2: |input| {
        let mut scores = Vec::new();
        'lines: for line in input.lines() {
            let mut stack = Vec::new();
            for c in line.bytes() {
                let expected = match c {
                    b'(' | b'[' | b'{' | b'<' => {
                        stack.push(c);
                        continue;
                    }
                    b')' => b'(',
                    b']' => b'[',
                    b'}' => b'{',
                    b'>' => b'<',
                    _ => return Err("Unexpected character".into()),
                };
                if stack.pop() != Some(expected) {
                    continue 'lines;
                }
            }
            let mut score = 0;
            if !stack.is_empty() {
                for b in stack.iter().rev() {
                    let char_score = match b {
                        b'(' => 1,
                        b'[' => 2,
                        b'{' => 3,
                        b'<' => 4,
                        _ => unreachable!(),
                    };
                    score = char_score + 5_u64 * score;
                }
                scores.push(score);
            }
        }
        scores.sort_unstable();
        Ok(scores.get(scores.len() / 2).ok_or("No scores")?.to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY10.part1,
        empty: "" => 0,
        example: lines!(
            "[({(<(())[]>[[{[]{<()<>>"
            "[(()[<>])]({[<{<<[]>>("
            "{([(<{}[<>[]}>{[]{[(<()>"
            "(((({<>}<{<{<>}{[]{[]{}"
            "[[<[([]))<([[{}[[()]]]"
            "[{[{({}]{}}([{[{{{}}([]"
            "{<[[]]>}<{[{[{[]{()[[[]"
            "[<(<(<(<{}))><([]([]()"
            "<{([([[(<>()){}]>(<<{{"
            "<{([{{}}[<[[[<>{}]]]>[]]"
        ) => 26397,
        input: 316851,
    );
    test!(
        DAY10.part2,
        example: lines!(
            "[({(<(())[]>[[{[]{<()<>>"
            "[(()[<>])]({[<{<<[]>>("
            "{([(<{}[<>[]}>{[]{[(<()>"
            "(((({<>}<{<{<>}{[]{[]{}"
            "[[<[([]))<([[{}[[()]]]"
            "[{[{({}]{}}([{[{{{}}([]"
            "{<[[]]>}<{[{[{[]{()[[[]"
            "[<(<(<(<{}))><([]([]()"
            "<{([([[(<>()){}]>(<<{{"
            "<{([{{}}[<[[[<>{}]]]>[]]"
        ) => 288957,
        input: 2182912364,
    );
}
