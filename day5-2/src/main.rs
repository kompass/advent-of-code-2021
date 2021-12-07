use anyhow::Result;
use nom::{
    bytes::complete as bytes, character::complete as character, combinator, multi, sequence,
    IResult,
};
use std::io::{self, Read};

struct Point(usize, usize);

struct FromTo(Point, Point);

#[derive(Debug, PartialEq, Eq)]
enum DiagDir {
    LeftRight,
    RightLeft,
}

#[derive(Debug, PartialEq, Eq)]
enum Line {
    Vert {
        x: usize,
        y1: usize,
        y2: usize,
    },
    Horiz {
        y: usize,
        x1: usize,
        x2: usize,
    },
    Diag {
        x_left: usize,
        y_left: usize,
        length: usize,
        dir: DiagDir,
    },
}

impl From<FromTo> for Line {
    fn from(from_to: FromTo) -> Line {
        let FromTo(from, to) = from_to;

        match (from.0 == to.0, from.1 == to.1) {
            (true, _) => Line::Vert {
                x: from.0,
                y1: from.1.min(to.1),
                y2: from.1.max(to.1),
            },
            (_, true) => Line::Horiz {
                y: from.1,
                x1: from.0.min(to.0),
                x2: from.0.max(to.0),
            },
            _ => {
                let (left_point, right_point) = if from.0 < to.0 {
                    (from, to)
                } else {
                    (to, from)
                };
                Line::Diag {
                    x_left: left_point.0,
                    y_left: left_point.1,
                    length: right_point.0 - left_point.0 + 1,
                    dir: if left_point.1 < right_point.1 {
                        DiagDir::LeftRight
                    } else {
                        DiagDir::RightLeft
                    },
                }
            }
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    offset: (usize, usize),
    size: (usize, usize),
    cells: Vec<usize>,
}

impl Rectangle {
    fn new(offset: (usize, usize), size: (usize, usize)) -> Self {
        Self {
            offset,
            size,
            cells: vec![0; size.0 * size.1],
        }
    }

    fn bounding(from_tos: &[FromTo]) -> Self {
        let (min_x, min_y, max_x, max_y) = from_tos
            .iter()
            .flat_map(|from_to| [&from_to.0, &from_to.1])
            .fold(
                (usize::MAX, usize::MAX, usize::MIN, usize::MIN),
                |(min_x, min_y, max_x, max_y), &Point(x, y)| {
                    (min_x.min(x), min_y.min(y), max_x.max(x), max_y.max(y))
                },
            );

        Self::new((min_x, min_y), (max_x - min_x + 1, max_y - min_y + 1))
    }

    fn add_point(&mut self, x: usize, y: usize) {
        let (local_x, local_y) = (x - self.offset.0, y - self.offset.1);

        self.cells[local_y * self.size.0 + local_x] += 1;
    }

    fn add_line(&mut self, line: &Line) {
        match line {
            &Line::Vert { x, y1, y2 } => {
                for y in y1..=y2 {
                    self.add_point(x, y)
                }
            }
            &Line::Horiz { y, x1, x2 } => {
                for x in x1..=x2 {
                    self.add_point(x, y)
                }
            }
            &Line::Diag {
                dir: DiagDir::LeftRight,
                x_left,
                y_left,
                length,
            } => {
                for (x, y) in
                    itertools::iterate((x_left, y_left), |(x, y)| (x + 1, y + 1)).take(length)
                {
                    self.add_point(x, y);
                }
            }
            &Line::Diag {
                dir: DiagDir::RightLeft,
                x_left,
                y_left,
                length,
            } => {
                for (x, y) in
                    itertools::iterate((x_left, y_left), |(x, y)| (x + 1, y - 1)).take(length)
                {
                    self.add_point(x, y);
                }
            }
        }
    }

    fn intersect_count(&self) -> usize {
        self.cells
            .iter()
            .filter(|&&passing_lines| passing_lines > 1)
            .count()
    }
}

fn parse_number(input: &str) -> IResult<&str, usize> {
    combinator::map_res(character::digit1, |n: &str| n.parse::<usize>())(input)
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    combinator::map(
        sequence::separated_pair(parse_number, character::char(','), parse_number),
        |(x, y)| Point(x, y),
    )(input)
}

fn parse_from_to(input: &str) -> IResult<&str, FromTo> {
    combinator::map(
        sequence::separated_pair(parse_point, bytes::tag(" -> "), parse_point),
        |(p1, p2)| FromTo(p1, p2),
    )(input)
}

fn parse_file(input: &str) -> IResult<&str, Vec<FromTo>> {
    multi::separated_list1(character::line_ending, parse_from_to)(input)
}

fn main() -> Result<()> {
    let cache = {
        let mut cache = String::new();
        io::stdin().lock().read_to_string(&mut cache)?;
        cache
    };

    let (_, from_tos) = parse_file(&cache).unwrap();
    let mut rect = Rectangle::bounding(&from_tos);
    let lines: Vec<Line> = from_tos.into_iter().map(|from_to| from_to.into()).collect();

    for line in lines {
        rect.add_line(&line);
    }

    println!("{}", rect.intersect_count());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diags() {
        let diag = FromTo(Point(3, 3), Point(1, 1));

        assert_eq!(
            Line::from(diag),
            Line::Diag {
                x_left: 1,
                y_left: 1,
                length: 3,
                dir: DiagDir::LeftRight
            }
        );
    }
}
