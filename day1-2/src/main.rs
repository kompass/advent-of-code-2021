use std::io::{self, BufRead};
use anyhow::Result;

enum Window {
    Empty,
    OneElem(usize),
    TwoElem(usize, usize),
    ThreeElem(usize, usize, usize),
}

impl Window {
    fn slide(&self, new_value: usize) -> Self {
        match *self {
            Window::Empty => Window::OneElem(new_value),
            Window::OneElem(first) => Window::TwoElem(first, new_value),
            Window::TwoElem(first, second) => Window::ThreeElem(first, second, new_value),
            Window::ThreeElem(_, second, third) => Window::ThreeElem(second, third, new_value),
        }
    }

    fn sum(&self) -> usize {
        match *self {
            Window::Empty => 0,
            Window::OneElem(first) => first,
            Window::TwoElem(first, second) => first + second,
            Window::ThreeElem(first, second, third) => first + second + third,
        }
    }
}

fn main() -> Result<()> {
    let mut previous_window = Window::Empty;
    let mut increases = 0;

    for line in io::stdin().lock().lines() {
        let value: usize = line?.parse().unwrap();
        let window = previous_window.slide(value);

        if let Window::ThreeElem(_, _, _) = previous_window {
            let previous_sum = previous_window.sum();
            let sum = window.sum();
            if sum > previous_sum {
                increases += 1;
            }
        } 

        previous_window = window;
    }

    println!("{}", increases);

    Ok(())
}
