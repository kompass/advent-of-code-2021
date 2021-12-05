use std::io::{self, BufRead};
use anyhow::{Result, Error};

fn main() -> Result<()> {
    let mut bit_sums = Vec::new();
    let mut line_count = 0;

    for line in io::stdin().lock().lines() {
        let line = line?;
        line_count += 1;

        if line.len() > bit_sums.len() {
            bit_sums.resize(line.len(), 0);
        }

        for (pos, c) in line.chars().enumerate() {
            if c == '1' {
                bit_sums[pos] += 1;
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for bit_sum in bit_sums {
        gamma *= 2;
        epsilon *= 2;

        if bit_sum * 2 >= line_count {
            gamma += 1;
        } else {
            epsilon += 1;
        }
    }

    println!("{}", gamma * epsilon);

    Ok(())
}
