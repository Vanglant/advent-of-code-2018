use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*};
use std::io::Write;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(",").map(|x| x.trim()).collect();

        let x = coords[0].parse::<i32>()?;
        let y = coords[1].parse::<i32>()?;

        Ok(Point { x, y })
    }
}

struct Grid {
    cells: Vec<Vec<String>>,
    id_point: HashMap<u32, Point>,
}

impl Grid {
    fn new(points: Vec<Point>) -> Grid {
        let mut id_point = HashMap::new();
        let x_size = points.iter().map(|x| x.x).max().unwrap() + 1;
        let y_size = points.iter().map(|x| x.y).max().unwrap() + 1;

        let mut matrix = vec![vec!["_".to_string(); y_size as usize]; x_size as usize];
        for (pos, point) in points.iter().enumerate() {
            let value = pos as u32;
            matrix[point.x as usize][point.y as usize] = value.to_string();
            id_point.insert(value, point);
        }

        Grid { cells: matrix, id_point }
    }

    fn write(&self, s: &str) -> Result<(), failure::Error> {
        let mut file = File::create(s.to_string())?;
        for x_cells in &self.cells {
            for cell in x_cells {
                write!(file, "{}", cell);
            }
            writeln!(file, "");
        }
        Ok(())
    }
}

fn parse_points(s: &str) -> Vec<Point> {
    s.lines().map(|x| Point::from_str(x).unwrap()).collect()
}

fn main() -> Result<(), failure::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let points = parse_points(&input);
    let grid = Grid::new(points);
    grid.write("result.txt");

    for x in 0..&grid.cells.len() {
        for location in x {
            // now i have to calc the manhattan distance for each registered point
            // and find the lowest one
        }
    }

    println!("{:?}", grid);
    Ok(())
}