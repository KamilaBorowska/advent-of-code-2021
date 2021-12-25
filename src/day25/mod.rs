use crate::Solution;

pub(super) const DAY25: Solution = Solution {
    part1: |input| {
        let mut map: Vec<Vec<_>> = input.lines().map(|str| str.bytes().collect()).collect();
        for i in 1.. {
            let mut anything_changed = false;
            let mut new_map = vec![vec![b'.'; map[0].len()]; map.len()];
            for y in 0..map.len() {
                for x in 0..map[0].len() {
                    if map[y][x] == b'>' {
                        let new_x = (x + 1) % map[0].len();
                        if map[y][new_x] == b'.' {
                            anything_changed = true;
                            new_map[y][new_x] = b'>';
                        } else {
                            new_map[y][x] = b'>';
                        }
                    }
                }
            }
            for y in 0..map.len() {
                for x in 0..map[0].len() {
                    if map[y][x] == b'v' {
                        let new_y = (y + 1) % map.len();
                        if map[new_y][x] != b'v' && new_map[new_y][x] == b'.' {
                            anything_changed = true;
                            new_map[new_y][x] = b'v';
                        } else {
                            new_map[y][x] = b'v';
                        }
                    }
                }
            }
            if !anything_changed {
                return Ok(i.to_string());
            }
            map = new_map;
        }
        unreachable!()
    },
    part2: |_| Ok("Remotely Start The Sleigh".into()),
};

#[cfg(test)]
mod test {
    use crate::test;

    test!(
        DAY25.part1,
        example: lines!(
            "v...>>.vv>"
            ".vv>>.vv.."
            ">>.>v>...v"
            ">>v>>.>.v."
            "v>v.vv.v.."
            ">.>>..v..."
            ".vv..>.>v."
            "v.v..>>v.v"
            "....v..v.>"
        ) => 58,
        input: 378,
    );
}
