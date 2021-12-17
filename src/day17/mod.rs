use crate::Solution;
use std::error::Error;

fn parse(input: &str) -> Result<(i16, i16, i16, i16), Box<dyn Error>> {
    let input = input
        .strip_prefix("target area: x=")
        .ok_or("Missing prefix")?;
    let (x0, input) = input.split_once("..").ok_or("missing ..")?;
    let (x1, input) = input.split_once(", y=").ok_or("missing y")?;
    let (y0, y1) = input.split_once("..").ok_or("missing ..")?;
    Ok((x0.parse()?, x1.parse()?, y0.parse()?, y1.parse()?))
}

pub(super) const DAY17: Solution = Solution {
    part1: |input| {
        let y0 = parse(input)?.2;
        Ok(((y0 * y0 + y0) / 2).to_string())
    },
    part2: |input| {
        let (x0, x1, y0, y1) = parse(input)?;
        let mut count = 0;
        for x in 0..x1 + 1 {
            for y in y0..-y0 {
                let mut x_vel = x;
                let mut y_vel = y;
                let mut current_x = 0;
                let mut current_y = 0;
                while current_x <= x1 && current_y >= y0 {
                    if (x0..=x1).contains(&current_x) && (y0..=y1).contains(&current_y) {
                        count += 1;
                        break;
                    }
                    current_x += x_vel;
                    current_y += y_vel;
                    if x_vel > 0 {
                        x_vel -= 1;
                    }
                    y_vel -= 1;
                }
            }
        }
        Ok(count.to_string())
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY17.part1,
        example: "target area: x=20..30, y=-10..-5" => 45,
        input: "target area: x=236..262, y=-78..-58" => 3003,
    );
    test!(
        DAY17.part2,
        example: "target area: x=20..30, y=-10..-5" => 112,
        input: "target area: x=236..262, y=-78..-58" => 940,
    );
}
