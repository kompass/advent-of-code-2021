use std::io::{self, BufRead};
use anyhow::Result;

fn main() -> Result<()> {
    let mut previous_value = None;
    let mut increases = 0;

    for line in io::stdin().lock().lines() {
        let value: usize = line?.parse().unwrap();

        if let Some(previous_value) = previous_value {
            if value > previous_value {
                increases += 1;
            }
        }

        previous_value = Some(value);
    }

    println!("{}", increases);

    Ok(())
}
