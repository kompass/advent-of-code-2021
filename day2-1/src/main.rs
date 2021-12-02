use std::str::FromStr;
use std::io::{self, BufRead};
use anyhow::{Result, Error};

enum Move {
    Forward(usize),
    Up(usize),
    Down(usize),
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
       let mut parts = s.split(" ");
       let command = parts.next().ok_or(Error::msg("empty command"))?;
       let amount = parts.next().ok_or(Error::msg("empty amount"))?.parse::<usize>()?;

       match command {
           "forward" => Ok(Self::Forward(amount)),
           "up" => Ok(Self::Up(amount)),
           "down" => Ok(Self::Down(amount)),
           _ => Err(Error::msg("unknown command")),
       }
    }
}

impl Move {
    fn apply(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Move::Forward(amount) => (pos.0 + amount, pos.1),
            Move::Up(amount) => (pos.0, pos.1 - amount),
            Move::Down(amount) => (pos.0, pos.1 + amount),
        }
    }
}

fn main() -> Result<()> {
    let mut pos = (0, 0);

    for line in io::stdin().lock().lines() {
        let command = Move::from_str(&line?)?;
        pos = command.apply(pos);
    }

    println!("{}", pos.0 * pos.1);

    Ok(())
}
