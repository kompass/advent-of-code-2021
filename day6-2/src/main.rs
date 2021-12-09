use std::io::{self, BufRead};

fn main() {
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    let initial_state = line.split(",").fold([0; 9], |mut fishes, age| {
        fishes[age.parse::<usize>().unwrap()] += 1;
        fishes
    });
    let final_count = itertools::iterate(initial_state, |state| {
        let mut state = *state;
        state.rotate_left(1);
        state[6] += state[8];
        state
    })
    .nth(256)
    .unwrap()
    .into_iter()
    .sum::<usize>();

    println!("{}", final_count);
}
