#[macro_use]
extern crate nom;

use std::io::{self, prelude::*};
use::std::str::FromStr;
use failure::format_err;
use std::{thread, time};


#[derive(Debug)]
struct PointOfLight {
    pos: Position,
    vel: Velocity,
}

#[derive(Debug)]
struct Position {
    x: i64,
    y: i64
}

#[derive(Debug)]
struct Velocity {
    x: i64,
    y: i64
}

impl FromStr for PointOfLight {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use nom::{digit, types::CompleteStr as NomInput};

        named!(number(NomInput) -> i64,
             map_res!(recognize!(tuple!(opt!(char!('-')), digit)), |x: NomInput| x.parse())
        );

        named!(coord(NomInput) -> (i64, i64),
            delimited!(
                char!('<'),
                separated_pair!(ws!(number), tag!(","), ws!(number)),
                char!('>')
            )
        );

        named!(position(NomInput) -> Position,
            map!(
            preceded!(tag!("position="), coord),
            |(x, y)| Position {x, y}
            )
        );

        named!(velocity(NomInput) -> Velocity,
            map!(
                preceded!(tag!("velocity="), coord),
                |(x, y)| Velocity {x, y}
            )
        );

        named!(velocity_position(NomInput) -> PointOfLight,
            do_parse!(
                pos: position >>
                char!(' ') >>
                vel: velocity >>
                (PointOfLight { pos, vel })
            )
        );

        velocity_position(NomInput(s))
            .map(|(_rest, result)| result)
            .map_err(|e| format_err!("Failed to parse square: {}", e))
    }
}

fn parse_points(s: &str) -> Result<Vec<PointOfLight>, failure::Error> {
    s.lines().map(str::parse::<PointOfLight>).collect()
}

fn run_simulation(s: i64, points: &Vec<PointOfLight>) {
    let mut max_x = 0;
    let mut min_x = 10000;
    let mut max_y = 0;
    let mut min_y = 10000;
    for p in points {
        let x = p.pos.x + (s * p.vel.x);
        let y = p.pos.y + (s * p.vel.y);

        min_x = std::cmp::min(min_x, x);
        min_y = std::cmp::min(min_y, y);
        max_x = std::cmp::max(max_x, x);
        max_y = std::cmp::max(max_y, y);
    }

    let width = max_x - min_x + 1;
    let height= max_y - min_y + 1;
    if width <= 62 && height <= 10 {
        // part 2
        println!("{}", s);
        let mut map: Vec<Vec<&str>> = vec![vec!["."; width as usize]; height as usize];
        for point in points {
            let mut x = point.pos.x + (s * point.vel.x);
            let mut y = point.pos.y + (s * point.vel.y);
            if min_x >= 0 {
                x = x - min_x;
            }else {
                x = x + min_x.abs();
            }
            if min_y >= 0 {
                y = y - min_y;
            }else {
                y = y + min_y.abs();
            }
            map[y as usize][x as usize] = "#";
        }
        print_map(&map);
    }
}

fn print_map(map: &Vec<Vec<&str>>) {
    for x in map {
        for y in x {
            print!("{}", y);
        }
        println!();
    }
    // Clear console in case i want to animate it
    // print!("{}[2J", 27 as char);
}

fn main() -> Result<(), failure::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let points = parse_points(&input)?;
    let seconds_to_run = 12000;

    for i in 0..seconds_to_run {
        run_simulation(i, &points);
    }

    Ok(())
}