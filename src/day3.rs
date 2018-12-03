#[macro_use]
extern crate nom;

use failure::format_err;
use std::fs::File;
use std::io::{self, prelude::*};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Claim {
    id: u32,
    edge_left_top: (u32, u32),
    width: u32,
    height: u32,
}

impl FromStr for Claim {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom::{digit, types::CompleteStr as NomInput};

        named!(parse_id(NomInput) -> u32,
            map_res!(preceded!(char!('#'), digit), |d: NomInput| d.parse())
        );
        named!(corner(NomInput) -> (u32, u32),
            map!(
                separated_pair!(
                    map_res!(digit, |d: NomInput| d.parse()),
                    char!(','),
                    map_res!(digit, |d: NomInput| d.parse())
                ),
                |(x, y)| (x, y)
            )
        );

        named!(claim(NomInput) -> Claim,
            do_parse!(
                id: parse_id >>
                ws!(char!('@')) >>
                top_left: corner >>
                ws!(char!(':')) >>
                width: map_res!(terminated!(digit, tag!("x")), |d: NomInput| d.parse()) >>
                height: map_res!(digit, |d: NomInput| d.parse()) >>
                (Claim { id, edge_left_top: top_left, width, height})
            )
        );

        claim(NomInput(s))
            .map(|(_rest, result)| result)
            .map_err(|e| format_err!("Failed to parse square: {}", e))
    }
}

fn parse_claims(s: &str) -> Result<Vec<Claim>, failure::Error> {
    s.lines().map(str::parse::<Claim>).collect()
}

fn main() -> Result<(), failure::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let claims = parse_claims(&input)?;

    for claim in claims {
        println!("{:?}", claim);
    }

    let fabric: Vec<Vec<&str>> = vec![vec!["."; 1000]; 1000];
    let mut file = File::create("result.txt")?;
    for x in 0..1000 {
        for i in 0..1000 {
            // print!("{}", fabric[x][i]);
            write!(file, "{}", fabric[x][i])?;
        }
        writeln!(file, " ")?;
    }

    Ok(())
}
