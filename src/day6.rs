use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*};
use std::io::Write;
use std::num::ParseIntError;
use std::str::FromStr;
use std::collections::HashSet;

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

impl Point {
    fn manhattan_distance(&self, y: i32, x: i32) -> u32 {
        (abs(self.y - y) + abs(self.x - x)) as u32
    }
}

fn abs(s: i32) -> i32{
    if s > 0 {
        s
    }else {
        -s
    }
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<String>>,
}

impl Grid {
    fn new(points: &Vec<Point>) -> Grid {
        let x_size = points.iter().map(|x| x.x).max().unwrap() + 2;
        let y_size = points.iter().map(|x| x.y).max().unwrap() + 1;

        let mut matrix = vec![vec!["_".to_string(); x_size as usize]; y_size as usize];
        for (pos, point) in points.iter().enumerate() {
            let value = pos as u32;
            matrix[point.y as usize][point.x as usize] = value.to_string();
        }

        Grid { cells: matrix }
    }

    fn write(&self, s: &str) -> Result<(), failure::Error> {
        let mut file = File::create(s.to_string())?;
        for x_cells in &self.cells {
            for cell in x_cells {
                write!(file, "{}", cell)?;
            }
            writeln!(file, "")?;
        }
        Ok(())
    }
}

fn parse_points(s: &str) -> Vec<Point> {
    s.lines().map(|x| Point::from_str(x).unwrap()).collect()
}

fn calculate_closer(hmap: HashMap<usize, u32>) -> String {
    let mut lowest_value = std::u32::MAX;
    let mut id = "".to_string();
    for (pos, value) in &hmap {
        if *value < lowest_value {
            lowest_value = *value;
            id = pos.to_string();
        }
    }
    if hmap.iter().filter(|(_, y)| **y == lowest_value).count() >= 2 {
        '.'.to_string()
    }else {
        id
    }
}

fn fill_cells(cells: Vec<Vec<String>>, points: &Vec<Point>) -> Grid {
    let mut filled_cells = cells.clone();
    let mut chosen_points: HashMap<String, u32> = HashMap::new();
    let mut infinite_points: HashSet<String>= HashSet::new();
    for y in 0..filled_cells.len() {
        for x in 0..filled_cells[y].len() {
            let mut id_manhattan: HashMap<usize, u32> = HashMap::new();
            for (pos, point) in points.iter().enumerate() {
                let manhattan_distance = point.manhattan_distance(x as i32, y as i32);
                id_manhattan.insert(pos, manhattan_distance);
            }
            let closer_point = calculate_closer(id_manhattan);
            if y == filled_cells.len() || x == filled_cells.len() || x == 0 || y == 0 {
                infinite_points.insert(closer_point.clone());
            }
            filled_cells[x][y] = closer_point.clone();
            let count = chosen_points.entry(closer_point.clone()).or_insert(0);
            *count += 1;
        }
    }
    println!("{:?}", &chosen_points);
    let higher_area_not_infinite: Vec<u32> = chosen_points
        .iter()
        .filter(|(x , _)| !infinite_points.contains(*x))
        .map(|(_ , y)| *y)
        .collect();

    println!("{:?}", higher_area_not_infinite.iter().max().unwrap());


    Grid { cells: filled_cells }
}

fn main() -> Result<(), failure::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let points = parse_points(&input);
    let grid = Grid::new(&points);
    grid.write("result_unfilled.txt")?;
    let filled_grid = fill_cells(grid.cells, &points);

    filled_grid.write("result.txt")?;

    Ok(())
}