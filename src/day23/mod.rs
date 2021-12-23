use crate::Solution;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::mem;

const TARGETS: u8 = 7;
const ROW_ELEMENTS: u8 = 4;

// #############
// #01.2.3.4.56#
// ###7#8#9#A###
//   #B#C#D#E#
//   #########
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct Position(u8);

impl Position {
    fn is_burrow(self) -> bool {
        self.0 >= TARGETS
    }

    fn parent_burrow(self) -> Option<Position> {
        (self.0 >= TARGETS + ROW_ELEMENTS).then(|| Position(self.0 - ROW_ELEMENTS))
    }

    fn checked_range(self, to: Self) -> impl Iterator<Item = Self> {
        let pre = self.0 - 6;
        if to.0 > pre {
            pre + 1..to.0
        } else {
            to.0 + 1..pre + 1
        }
        .map(Self)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct Positions<const N: usize>([Position; N]);

impl<const N: usize> Positions<N> {
    fn path_cost(self, i: usize, mut to: Position) -> Option<u32> {
        let mut from = self.0[i];
        let mut cost = 0;
        if self.0.contains(&to) {
            return None;
        }
        if to.is_burrow() {
            mem::swap(&mut from, &mut to);
        }
        while let Some(parent) = from.parent_burrow() {
            if self.0.contains(&parent) {
                return None;
            }
            from = parent;
            cost += 1;
        }
        for pos in from.checked_range(to) {
            if self.0.contains(&pos) {
                return None;
            }
            cost += 2;
        }
        if to.0 == 0 || to.0 == TARGETS - 1 {
            cost += 1;
        } else {
            cost += 2;
        }
        let class = Self::class(i);
        Some(cost * 10_u32.pow(class as u32))
    }

    fn class(i: usize) -> u8 {
        (i / (N / ROW_ELEMENTS as usize)) as u8
    }

    fn class_positions(&self, class: u8) -> &[Position] {
        let elems = N / ROW_ELEMENTS as usize;
        &self.0[class as usize * elems..][..elems]
    }

    fn is_placed_correctly(self, i: usize) -> bool {
        let class = Self::class(i);
        let mut position = self.0[i];
        if (position.0 - TARGETS) % ROW_ELEMENTS != class {
            return false;
        }
        while position.0 < TARGETS + N as u8 {
            if !self.class_positions(class).contains(&position) {
                return false;
            }
            position.0 += ROW_ELEMENTS;
        }
        true
    }

    fn find_positions(self) -> u32 {
        let mut heap = BinaryHeap::from_iter([(Reverse(0), self)]);
        while let Some((Reverse(cost), positions)) = heap.pop() {
            if (0..N).all(|i| positions.is_placed_correctly(i)) {
                return cost;
            }
            for i in 0..N {
                let current_pos = positions.0[i];
                if current_pos.is_burrow() && !positions.is_placed_correctly(i) {
                    for target in (0..TARGETS).map(Position) {
                        if let Some(mut path_cost) = positions.path_cost(i, target) {
                            let mut positions = positions;
                            positions.0[i] = target;
                            let mut found_something = true;
                            while found_something {
                                found_something = false;
                                for i in 0..N {
                                    let class = Self::class(i);
                                    if !positions.0[i].is_burrow() {
                                        let mut new_pos = Position(TARGETS + N as u8 - 4 + class);
                                        while positions.class_positions(class).contains(&new_pos) {
                                            new_pos.0 -= 4;
                                        }
                                        if let Some(move_path_cost) =
                                            positions.path_cost(i, new_pos)
                                        {
                                            path_cost += move_path_cost;
                                            positions.0[i] = new_pos;
                                            found_something = true;
                                        }
                                    }
                                }
                            }
                            heap.push((Reverse(cost + path_cost), positions));
                        }
                    }
                }
            }
        }
        unreachable!();
    }
}

fn parse_input<const N: usize>(s: &str, extras: &str) -> Positions<N> {
    let mut arr = [Position(0); N];
    let mut positions = [0, N / 4, N / 2, 3 * N / 4];
    let mut position = TARGETS;
    let mut add = |c| {
        let index = match c {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            'D' => 3,
            _ => return false,
        };
        arr[positions[index]].0 = position;
        positions[index] += 1;
        position += 1;
        true
    };
    let mut chars = s.chars();
    for _ in 0..4 {
        for c in &mut chars {
            if add(c) {
                break;
            }
        }
    }
    for c in extras.chars().chain(chars) {
        add(c);
    }
    Positions(arr)
}

pub(super) const DAY23: Solution = Solution {
    part1: |input| Ok(parse_input::<8>(input, "").find_positions().to_string()),
    part2: |input| {
        Ok(parse_input::<16>(input, "DCBADBAC")
            .find_positions()
            .to_string())
    },
};

#[cfg(test)]
mod test {
    use super::{Position, Positions};
    use crate::test;

    #[test]
    fn path_cost() {
        let positions = Positions([11, 14, 7, 9, 13, 8, 10, 12].map(Position));
        assert_eq!(positions.path_cost(2, Position(6)), Some(90));
        assert_eq!(positions.path_cost(2, Position(0)), Some(30))
    }

    #[test]
    fn class() {
        assert_eq!(Positions::<8>::class(0), 0);
        assert_eq!(Positions::<8>::class(1), 0);
        assert_eq!(Positions::<8>::class(2), 1);
        assert_eq!(Positions::<8>::class(3), 1);
        assert_eq!(Positions::<8>::class(4), 2);
    }

    #[test]
    fn is_placed_correctly() {
        let positions = Positions([11, 14, 7, 9, 13, 8, 10, 12].map(Position));
        assert!(positions.is_placed_correctly(0));
        assert!(!positions.is_placed_correctly(1));
        assert!(!positions.is_placed_correctly(6));
    }

    test!(
        DAY23.part1,
        example: lines!(
            "#############"
            "#...........#"
            "###B#C#B#D###"
            "  #A#D#C#A#"
            "  #########"
        ) => 12521,
        input: lines!(
            "#############"
            "#...........#"
            "###D#C#A#B###"
            "  #B#C#D#A#"
            "  #########"
        ) => 15160,
    );
    test!(
        DAY23.part2,
        example: lines!(
            "#############"
            "#...........#"
            "###B#C#B#D###"
            "  #A#D#C#A#"
            "  #########"
        ) => 44169,
        input: lines!(
            "#############"
            "#...........#"
            "###D#C#A#B###"
            "  #B#C#D#A#"
            "  #########"
        ) => 46772,
    );
}
