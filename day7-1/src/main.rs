use std::io::{self, BufRead};

fn main() {
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    let sorted_crabs = {
        let mut crabs = line.split(",").map(|pos| pos.parse().unwrap()).collect::<Vec<usize>>();
        crabs.sort();
        crabs
    };

    let median = sorted_crabs[sorted_crabs.len() / 2];

    let fuel_amount = sorted_crabs.into_iter().map(|pos| (pos as isize - median as isize).abs() as usize).sum::<usize>();

    println!("{}", fuel_amount);
}
