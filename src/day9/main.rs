use anyhow::Result;
use displaydoc::Display;
use regex::Regex;
use shared::shared_main;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Sub};
use tracing::{debug, info};

fn main() -> Result<()> {
    shared_main(solution1, solution2)
}

#[derive(Debug, Ord, Eq, PartialOrd, PartialEq, Display, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn square(&self, other: &Self) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

fn solution1(input: &File) -> Result<u64> {
    let mut solution = 0;

    let input_rg = Regex::new(r"^(\d+),(\d+)\s*$")?; // number,bnumber
    let mut input_buf = BufReader::new(input);
    let mut input_line = String::new();

    let mut points = Vec::<Point>::new();

    // parse input from file line by line.
    while input_buf.read_line(&mut input_line)? > 0 {
        let Some((_, [x, y])) = input_rg.captures(&input_line).map(|c| c.extract()) else {
            panic!("Invalid input format: {}", input_line);
        };

        debug!("Adding point: {x},{y}");

        points.push(Point {
            x: x.parse()?,
            y: y.parse()?,
        });

        input_line.clear();
    }

    // Sort points top left -> bottom right
    points.sort();

    for point1_idx in 0..points.len() {
        for point2_idx in point1_idx + 1..points.len() {
            let square_size = points[point1_idx].square(&points[point2_idx]);
            debug!(
                "Computing square: {:?} - {:?} : {}",
                points[point1_idx], points[point2_idx], square_size
            );
            if square_size > solution {
                info!(
                    "Largest square: {:?} - {:?} : {}",
                    points[point1_idx], points[point2_idx], square_size
                );
                solution = square_size;
            }
        }
    }

    Ok(solution)
}

fn solution2(input: &File) -> Result<u64> {
    let mut solution = 0;

    let input_rg = Regex::new(r"^(\d+),(\d+)\s*$")?; // number,bnumber
    let mut input_buf = BufReader::new(input);
    let mut input_line = String::new();

    let mut points = Vec::<Point>::new();

    // parse input from file line by line to create a polygon,
    // where sequential vertices are connected by an edge.
    while input_buf.read_line(&mut input_line)? > 0 {
        let Some((_, [x, y])) = input_rg.captures(&input_line).map(|c| c.extract()) else {
            panic!("Invalid input format: {}", input_line);
        };

        debug!("Adding point: {x},{y}");

        points.push(Point {
            x: x.parse()?,
            y: y.parse()?,
        });

        input_line.clear();
    }

    let dot =
        |v1: &Point, v2: &Point| -> f64 { v1.x as f64 * v2.x as f64 + v1.y as f64 * v2.y as f64 };

    let magnitude = |v1: &Point| -> f64 { ((v1.x as f64).powi(2) + (v1.y as f64).powi(2)).sqrt() };

    let cross =
        |v1: &Point, v2: &Point| -> f64 { v1.x as f64 * v2.y as f64 - v1.y as f64 * v2.y as f64 };

    let orient = |p1: &Point, p2: &Point, pt: &Point| -> f64 { cross(&(*p2 - *p1), &(*pt - *p1)) };

    let line_intersect = |p1: &Point, p2: &Point, p3: &Point, p4: &Point, p: &mut Point| -> bool {
        let p1x = p1.x as f64;
        let p1y = p1.y as f64;
        let p2x = p2.x as f64;
        let p2y = p2.y as f64;
        let p3x = p3.x as f64;
        let p3y = p3.y as f64;
        let p4x = p4.x as f64;
        let p4y = p4.y as f64;

        let t_top = (p1x - p3x) * (p3y - p4y) - (p1y - p3y) * (p3x - p4x);
        let bot = (p1x - p2x) * (p3y - p4y) - (p1y - p2y) * (p3x - p4x);
        let u_top = (p1x - p2x) * (p1y - p3y) - (p1y - p2y) * (p1x - p3x);

        let t = t_top / bot;
        let u = u_top / bot;

        p.x = (p1x + t * (p2x - p1x)) as i64;
        p.y = (p1y + t * (p2y - p1y)) as i64;

        0f64 <= t && t <= 1f64 && 0f64 <= u && u <= 1f64
    };

    let square_within_polygon = |square: &Vec<Point>, polygon: &Vec<Point>| -> bool {
        let mut p = Point { x: 0, y: 0 };
        // For each line in the square
        for square_idx in 0..=3 {
            for polygon_idx in 0..polygon.len() {
                let p1 = &square[square_idx];
                let p2 = if square_idx == 3 {
                    &square[0]
                } else {
                    &square[square_idx + 1]
                };
                let p3 = &polygon[polygon_idx];
                let p4 = if polygon_idx == polygon.len() - 1 {
                    &polygon[0]
                } else {
                    &polygon[polygon_idx + 1]
                };

                if line_intersect(p1, p2, p3, p4, &mut p) && p != *p1 && p != *p2 {
                    debug!(
                        "Square line {:?} - {:?} crosses line {:?} - {:?} at point: {:?}",
                        p1, p2, p3, p4, p
                    );
                    return false;
                }
            }
        }
        return true;
    };

    let mut square = vec![Point { x: 0, y: 0 }; 4];

    // create a square, clip its bounds against the input polygon
    for point1_idx in 0..points.len() {
        for point2_idx in point1_idx + 1..points.len() {
            let square_size = points[point1_idx].square(&points[point2_idx]);
            let p1 = &points[point1_idx];
            let p2 = &points[point2_idx];

            if square_size > solution {
                debug!(
                    "Evaluating square: {:?} - {:?} : {}",
                    points[point1_idx], points[point2_idx], square_size
                );

                square[0].x = p1.x;
                square[0].y = p1.y;
                square[1].x = p2.x;
                square[1].y = p1.y;
                square[2].x = p2.x;
                square[2].y = p2.y;
                square[3].x = p1.x;
                square[3].y = p2.y;

                if square_within_polygon(&square, &points) {
                    info!(
                        "Largest square within polygon: {:?} - {:?} : {}",
                        points[point1_idx], points[point2_idx], square_size
                    );
                    solution = square_size;
                }
            }
        }
    }

    Ok(solution)
}
