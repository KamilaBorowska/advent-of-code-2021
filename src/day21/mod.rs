use crate::Solution;
use std::collections::HashMap;
use std::error::Error;
use std::mem;

const ROLLS: [(u16, u64); 7] = {
    let mut arr = [(0, 0); 7];
    let mut a = 1;
    while a <= 3 {
        let mut b = 1;
        while b <= 3 {
            let mut c = 1;
            while c <= 3 {
                let sum = a + b + c;
                arr[sum as usize - 3].0 = sum;
                arr[sum as usize - 3].1 += 1;
                c += 1;
            }
            b += 1;
        }
        a += 1;
    }
    arr
};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Player {
    position: u16,
    score: u16,
}

fn parse_input(input: &str) -> Result<[Player; 2], Box<dyn Error>> {
    let input = input.trim();
    let input = input
        .strip_prefix("Player 1 starting position: ")
        .ok_or("Expected player 1 starting position")?;
    let (a, b) = input
        .split_once("\nPlayer 2 starting position: ")
        .ok_or("Expected player 2 starting position")?;
    let a = a.parse()?;
    let b = b.parse()?;
    Ok([a, b].map(|position| Player { position, score: 0 }))
}

fn quantum_game(cache: &mut HashMap<[Player; 2], [u64; 2]>, players: [Player; 2]) -> [u64; 2] {
    let mut sum = [0, 0];
    for (roll, multiplier) in &ROLLS {
        let mut players = players;
        let [a, b] = &mut players;
        a.position += roll;
        if a.position > 10 {
            a.position -= 10;
        }
        a.score += a.position;
        mem::swap(a, b);
        let cached = if b.score >= 21 {
            [1, 0]
        } else if let Some(cached) = cache.get(&players) {
            *cached
        } else {
            let mut cached = quantum_game(cache, players);
            cached.swap(0, 1);
            cache.insert(players, cached);
            cached
        };
        for (s, v) in sum.iter_mut().zip(cached) {
            *s += v * multiplier;
        }
    }
    sum
}

pub(super) const DAY21: Solution = Solution {
    part1: |input| {
        let [mut a, mut b] = parse_input(input)?;
        let mut dice = (1..=100).cycle();
        let mut rolls = 0;
        let mut roll = || -> u16 {
            rolls += 3;
            (&mut dice).take(3).sum()
        };
        loop {
            a.position = (a.position + roll()) % 10;
            if a.position == 0 {
                a.position = 10;
            }
            a.score += a.position;
            if a.score >= 1000 {
                return Ok((u32::from(b.score) * rolls).to_string());
            }
            mem::swap(&mut a, &mut b);
        }
    },
    part2: |input| {
        let players = parse_input(input)?;
        Ok(quantum_game(&mut HashMap::new(), players)
            .iter()
            .max()
            .unwrap()
            .to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY21.part1,
        example: lines!(
            "Player 1 starting position: 4"
            "Player 2 starting position: 8"
        ) => 739785,
        input: lines!(
            "Player 1 starting position: 7"
            "Player 2 starting position: 9"
        ) => 679329,
    );
    test!(
        DAY21.part2,
        example: lines!(
            "Player 1 starting position: 4"
            "Player 2 starting position: 8"
        ) => 444356092776315,
        input: lines!(
            "Player 1 starting position: 7"
            "Player 2 starting position: 9"
        ) => 433315766324816,
    );
}
