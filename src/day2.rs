use std::collections::HashMap;
use std::io::{self, prelude::*};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines().map(Result::unwrap).into_iter();
    let (number1, number2) = lines.fold((0, 0), |acc, x| calculate_coincidences(&acc, &x));
    println!("{}", number1 * number2);
}

fn calculate_coincidences(matches: &(u32, u32), box_id: &str) -> (u32, u32) {
    let mut coincidences_per_char: HashMap<char, u32> = HashMap::new();
    let (mut two_letters, mut three_letters) = matches;

    for single_char in box_id.chars() {
        let count = coincidences_per_char.entry(single_char).or_insert(0);
        *count += 1;
    }
    if coincidences_per_char.iter().any(|(_, &value)| value == 2) {
        two_letters += 1;
    }
    if coincidences_per_char.iter().any(|(_, &value)| value == 3) {
        three_letters += 1;
    }
    (two_letters, three_letters)
}
