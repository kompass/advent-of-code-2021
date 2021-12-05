use anyhow::{Error, Result};
use std::io::{self, Read};

enum Criteria {
    MostCommon,
    LeastCommon,
}

fn find_value<'a>(lines: impl Iterator<Item = &'a str>, criteria: Criteria, pos: usize) -> &'a str {
    let (ones, zeros): (Vec<&str>, Vec<&str>) =
        lines.partition(|line| line.chars().nth(pos).unwrap() == '1');

    let filtered = match (&criteria, ones.len() >= zeros.len()) {
        (Criteria::MostCommon, true) => ones,
        (Criteria::MostCommon, false) => zeros,
        (Criteria::LeastCommon, true) => zeros,
        (Criteria::LeastCommon, false) => ones,
    };

    if filtered.len() == 1 {
        filtered[0]
    } else {
        find_value(filtered.into_iter(), criteria, pos + 1)
    }
}

fn main() -> Result<()> {
    let cache = {
        let mut cache = String::new();
        io::stdin().lock().read_to_string(&mut cache)?;
        cache
    };

    let oxygen = find_value(cache.lines(), Criteria::MostCommon, 0);
    let co2 = find_value(cache.lines(), Criteria::LeastCommon, 0);

    let oxygen = usize::from_str_radix(oxygen, 2)?;
    let co2 = usize::from_str_radix(co2, 2)?;

    println!("{}", oxygen * co2);

    Ok(())
}
