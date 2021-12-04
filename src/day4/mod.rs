use crate::Solution;
use std::error::Error;
use std::num::ParseIntError;

type Board = [BoardRow; 5];
type BoardRow = [Option<u8>; 5];

fn parse_board(
    input: &str,
) -> Result<
    (
        impl Iterator<Item = Result<u8, ParseIntError>> + '_,
        Vec<Board>,
    ),
    Box<dyn Error>,
> {
    let mut lines = input.lines();
    let draws = lines
        .next()
        .ok_or("Missing draws")?
        .split(',')
        .map(str::parse);
    let mut boards = Vec::new();
    while let Some(line) = lines.next() {
        if !line.is_empty() {
            return Err("Expected an empty line".into());
        }
        let mut line = || get_line(&mut lines);
        boards.push([line()?, line()?, line()?, line()?, line()?]);
    }
    Ok((draws, boards))
}

fn get_line<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Result<BoardRow, Box<dyn Error>> {
    let line = iter.next().ok_or("Missing line")?;
    let mut split = line
        .split_whitespace()
        .map(|number| -> Result<_, Box<dyn Error>> { number.parse().map_err(From::from) });
    let mut next = || split.next().unwrap_or_else(|| Err("Missing number".into()));
    Ok([next()?, next()?, next()?, next()?, next()?].map(Some))
}

fn draw_number(draw: u8, boards: &mut [Board]) {
    for board in boards {
        for row in board {
            for tile in row {
                if *tile == Some(draw) {
                    *tile = None;
                }
            }
        }
    }
}

fn is_winning_board(board: Board) -> bool {
    board.iter().any(|row| row.iter().all(|&t| t == None))
        || (0..5).any(|i| board.iter().all(|row| row[i] == None))
}

fn board_sum(board: Board) -> u32 {
    board
        .iter()
        .flatten()
        .flatten()
        .copied()
        .map(u32::from)
        .sum()
}

pub(super) const DAY4: Solution = Solution {
    part1: |input| {
        let (draws, mut boards) = parse_board(input)?;
        for draw in draws {
            let draw = draw?;
            draw_number(draw, &mut boards);
            for &board in &boards {
                if is_winning_board(board) {
                    return Ok((board_sum(board) * u32::from(draw)).to_string());
                }
            }
        }
        Err("No valid board has been found".into())
    },
    part2: |input| {
        let (draws, mut boards) = parse_board(input)?;
        for draw in draws {
            let draw = draw?;
            draw_number(draw, &mut boards);
            let board = boards[0];
            boards.retain(|&board| !is_winning_board(board));
            if boards.is_empty() {
                return Ok((board_sum(board) * u32::from(draw)).to_string());
            }
        }
        Err("There are multiple winning boards".into())
    },
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY4.part1,
        example: lines!(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1"
            ""
            "22 13 17 11  0"
            " 8  2 23  4 24"
            "21  9 14 16  7"
            " 6 10  3 18  5"
            " 1 12 20 15 19"
            ""
            " 3 15  0  2 22"
            " 9 18 13 17  5"
            "19  8  7 25 23"
            "20 11 10 24  4"
            "14 21 16 12  6"
            ""
            "14 21 17 24  4"
            "10 16 15  9 19"
            "18  8 23 26 20"
            "22 11 13  6  5"
            " 2  0 12  3  7"
        ) => 4512,
        input: 22680,
    );
    test!(
        DAY4.part2,
        example: lines!(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1"
            ""
            "22 13 17 11  0"
            " 8  2 23  4 24"
            "21  9 14 16  7"
            " 6 10  3 18  5"
            " 1 12 20 15 19"
            ""
            " 3 15  0  2 22"
            " 9 18 13 17  5"
            "19  8  7 25 23"
            "20 11 10 24  4"
            "14 21 16 12  6"
            ""
            "14 21 17 24  4"
            "10 16 15  9 19"
            "18  8 23 26 20"
            "22 11 13  6  5"
            " 2  0 12  3  7"
        ) => 1924,
        input: 16168,
    );
}
