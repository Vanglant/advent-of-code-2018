use std::io::{self, prelude::*};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Sleigh {
    finished_steps: Vec<char>,
    step_requirements: HashMap<char, Vec<char>>,
    steps: HashSet<char>,
    ended: bool
}

impl Sleigh {
    fn new(s: String) -> Sleigh {
        let mut step_requirements: HashMap<char, Vec<char>> = HashMap::new();
        let mut steps: HashSet<char> = HashSet::new();
        for line in s.lines() {
            let vector: Vec<char> = line.chars().collect();
            let step = vector[36];
            let requirement = vector[5];
            let required = step_requirements.entry(step).or_insert(vec![]);
            steps.insert(requirement);
            steps.insert(step);
            required.push(requirement);
        }

        Sleigh { finished_steps: vec![], step_requirements, steps, ended: false }
    }

    fn next_step(&mut self) -> Result<(), failure::Error>{
        let mut possible_steps: Vec<char> = vec![];
        for step in &self.steps {
            if self.step_requirements.contains_key(&step) && !self.finished_steps.contains(&step) {
                let requirements = self.step_requirements.get(&step).unwrap();
                if requirements.iter().all(|x| self.finished_steps.contains(x)) {
                    possible_steps.push(*step);
                }
            } else if !self.finished_steps.contains(&step) {
                possible_steps.push(*step);
            }
        }

        if possible_steps.is_empty() {
            self.ended = true;
        }else {
            possible_steps.sort_by(|a, b| a.cmp(b));
            let next_step = possible_steps.first().unwrap().to_owned();
            self.finished_steps.push(next_step);
        }

        Ok(())
    }
}

fn main() -> Result<(), failure::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut sleigh = Sleigh::new(input);

    while !sleigh.ended {
        sleigh.next_step()?;
    }

    let result: String = sleigh.finished_steps.iter().collect();
    println!("{:?}", result);

    Ok(())
}