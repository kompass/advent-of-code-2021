use anyhow::{Error, Result};
use nom::{character::complete as character, combinator, multi, sequence, IResult};
use std::io::{self, Read};

const BOARD_WIDTH: usize = 5;
const BOARD_HEIGHT: usize = 5;
const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_HEIGHT;

#[derive(Debug)]
struct Board([(usize, bool); BOARD_SIZE]);

impl Board {
    fn new(cells: Vec<Vec<usize>>) -> Self {
        Self(
            cells
                .into_iter()
                .flatten()
                .map(|cell| (cell, false))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        let parse_line = multi::many1(sequence::preceded(
            character::space0,
            combinator::map_res(character::digit1, |n: &str| n.parse::<usize>()),
        ));

        combinator::map(
            multi::separated_list1(
                character::line_ending,
                parse_line
            ),
            |cells| Board::new(cells),
        )(input)
    }

    fn update(&mut self, value: usize) {
        for cell in self.0.iter_mut() {
            if cell.0 == value {
                cell.1 = true
            }
        }
    }

    fn is_cell_checked(&self, x: usize, y: usize) -> bool {
        self.0[y * BOARD_WIDTH + x].1
    }

    fn is_row_checked(&self, y: usize) -> bool {
        (0..BOARD_WIDTH)
            .into_iter()
            .all(|x| self.is_cell_checked(x, y))
    }

    fn is_col_checked(&self, x: usize) -> bool {
        (0..BOARD_HEIGHT)
            .into_iter()
            .all(|y| self.is_cell_checked(x, y))
    }

    fn is_board_winning(&self) -> bool {
        (0..BOARD_HEIGHT)
            .into_iter()
            .any(|y| self.is_row_checked(y))
            || (0..BOARD_WIDTH).into_iter().any(|x| self.is_col_checked(x))
    }

    fn score(&self, round: usize) -> usize {
        println!("round: {}, board: {:?}", round, self);
        self.0
            .iter()
            .filter_map(|cell| if cell.1 == false { Some(cell.0) } else { None })
            .sum::<usize>()
            * round
    }
}

fn parse_rounds(input: &str) -> IResult<&str, Vec<usize>> {
    multi::separated_list1(
        character::char(','),
        combinator::map_res(character::digit1, |n: &str| n.parse::<usize>()),
    )(input)
}

fn parse_file<'a>(input: &'a str) -> IResult<&'a str, (Vec<usize>, Vec<Board>)> {
    combinator::map(
        sequence::tuple((
            parse_rounds,
            character::line_ending,
            character::line_ending,
            multi::separated_list1(
                sequence::tuple((character::line_ending, character::line_ending)),
                Board::parse,
            ),
            character::line_ending,
            combinator::eof,
        )),
        |(rounds, _, _, boards, _, _)| (rounds, boards),
    )(input)
}

fn main() -> Result<()> {
    let cache = {
        let mut cache = String::new();
        io::stdin().lock().read_to_string(&mut cache)?;
        cache
    };

    let (_, (rounds, mut boards)) = parse_file(&cache).unwrap();

    println!("Board count: {}", boards.len());

    'rounds_loop: for round in rounds {
        let mut i = 0;

        while i < boards.len() {
            let board = &mut boards[i];
            board.update(round);

            if board.is_board_winning() {
                if boards.len() == 1 {
                    println!("{}", boards[0].score(round));
                    break 'rounds_loop;
                } else {
                    boards.remove(i);
                }
            } else {
                i += 1;
            }
        }
    }

    Ok(())
}
