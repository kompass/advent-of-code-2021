use std::io::{self, BufRead};

fn main() {
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    let crabs = line.split(",").map(|pos| pos.parse().unwrap()).collect::<Vec<usize>>();

    let mean = crabs.iter().sum::<usize>() / crabs.len();

    let fuel_amount = crabs.into_iter().map(|pos| (pos as isize - mean as isize).abs() as usize).map(|dist| dist * (dist + 1) / 2).sum::<usize>();

    println!("{}", fuel_amount);
}
