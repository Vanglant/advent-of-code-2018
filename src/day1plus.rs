use std::collections::HashSet;
use std::io::{self, prelude::*};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(Result::unwrap);

    let differences: Vec<i32> = lines.map(|line| line.parse::<i32>().unwrap()).collect();
    let mut number_of_repetitions = HashSet::new();

    let mut sum = 0;

    for difference in differences.iter().cycle() {
        sum += difference;
        if !number_of_repetitions.insert(sum) {
            println!("{}", sum);
            break;
        }
    }
}
