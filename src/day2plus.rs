use itertools::Itertools;
use std::io::{self, prelude::*};

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin
        .lock()
        .lines()
        .map(Result::unwrap)
        .into_iter()
        .collect();
    let number_of_lines = lines.len();
    for i in 1..number_of_lines {
        let line_to_check = &lines[i];
        for n in i + 1..number_of_lines {
            let number_of_repetitions = line_to_check
                .chars()
                .zip(lines[n].chars())
                .fold(0, |acc, (x, y)| if x != y { acc + 1 } else { acc + 0 });
            if number_of_repetitions == 1 {
                println!("{}, {}", lines[i], lines[n]);
            }
        }
    }
}
