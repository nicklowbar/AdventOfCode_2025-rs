use anyhow::Result;
use displaydoc::Display;
use regex::Regex;
use shared::shared_main;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Add, Sub};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use tracing::{debug, info, trace};

fn main() -> Result<()> {
    shared_main(solution1, solution2)
}

#[derive(Debug, Default, Ord, Eq, PartialOrd, PartialEq, Display, Copy, Clone)]
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

impl Sub<&Point> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Point) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

struct AABB {
    tl: Point,
    br: Point,
}

impl AABB {
    fn new() -> AABB {
        AABB {
            tl: Point::default(),
            br: Point::default(),
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

    let dot = |v1: &Point, v2: &Point| -> i128 { (v1.x * v2.x) as i128 + (v1.y * v2.y) as i128 };

    let cross =
        |v1: &Point, v2: &Point| -> i128 { -((v1.x * v2.y) as i128 - (v1.y * v2.x) as i128) }; // using +x, -y axes

    let orient = |p1: &Point, p2: &Point, pt: &Point| -> i128 { -cross(&p2.sub(p1), &pt.sub(p1)) };

    let line_intersect_interior = |p1: &Point, p2: &Point, p3: &Point, p4: &Point| -> bool {
        orient(p1, p2, p3) * orient(p1, p2, p4) < 0 && orient(p3, p4, p1) * orient(p3, p4, p2) < 0
    };

    let intervals_overlap = |a0: i64, a1: i64, b0: i64, b1: i64| {
        a0.min(a1).max(b0.min(b1)) <= a0.max(a1).min(b0.max(b1))
    };

    let project = |p: &Point, use_x: bool| {
        if use_x {
            p.x
        } else {
            p.y
        }
    };

    let lines_collinear_and_overlap = |p1: &Point, p2: &Point, p3: &Point, p4: &Point| {
        let p1p2 = p2.sub(p1);

        // first check collinearity
        if cross(&p1p2, &p3.sub(p1)) != 0 || cross(&p1p2, &p4.sub(p1)) != 0 {
            return false;
        }

        let use_x = p1p2.x.abs() >= p1p2.y.abs();

        let a0 = project(p1, use_x);
        let a1 = project(p2, use_x);
        let b0 = project(p3, use_x);
        let b1 = project(p4, use_x);

        intervals_overlap(a0, a1, b0, b1)
    };

    let point_on_polygon_border = |point: &Point, polygon: &Vec<Point>| -> bool {
        for current_idx in 0..polygon.len() {
            let pt1 = &polygon[current_idx];
            let pt2 = &polygon[(current_idx + 1) % polygon.len()];
            trace!(
                "Testing if {:?} lies within line: {:?} -> {:?}",
                point,
                pt1,
                pt2
            );

            let b_minus_a = pt2.sub(pt1);
            let p_minus_a = point.sub(pt1);
            let p_minus_b = point.sub(pt2);
            let cross_b_minus_a_p_minus_a = cross(&b_minus_a, &p_minus_a);
            let collinear = cross_b_minus_a_p_minus_a == 0;
            let dot_p_minus_a_p_minus_b = dot(&p_minus_a, &p_minus_b);
            let point_within_segment_bounds = dot_p_minus_a_p_minus_b <= 0; // < 0 = point lies within A -> B, 0 = Point is A or B, and > 0 means point is not within a -> b.

            trace!("collinearity test: B-A: {:?}, P-A {:?}, P-B {:?} (B-A)X(P-A) {}, (P-A)*(P-B) {}  is collinear? {} is within segment? {}", b_minus_a, p_minus_a, p_minus_b, cross_b_minus_a_p_minus_a, dot_p_minus_a_p_minus_b, collinear, point_within_segment_bounds);

            if collinear && point_within_segment_bounds {
                debug!("{:?} lies on segment: {:?} -> {:?}", point, pt1, pt2);
                return true;
            }
            trace!(
                "{:?} does not lie on segment: {:?} -> {:?}",
                point,
                pt1,
                pt2
            );
        }
        false
    };

    let point_within_polygon_winding = |point: &Point, polygon: &Vec<Point>| -> bool {
        // use upward/downward crossings to determine if the point is enclosed in the polygon
        let mut winding = 0;
        debug!("Evaluating point {:?} for polygon winding area.", point);
        for current_idx in 0..polygon.len() {
            let pt1 = &polygon[current_idx];
            // precondition: polygon vertices are clockwise.
            let pt2 = &polygon[(current_idx + 1) % polygon.len()];
            trace!("Evaluating line: {:?}->{:?}", pt1, pt2);

            // Upward crossing, point is to the left of the segment
            if pt1.y <= point.y {
                if pt2.y > point.y {
                    trace!("Evaluating potential upward crossing");
                    if orient(pt1, pt2, point) > 0 {
                        // we are left of the current segment, increase winding order
                        winding += 1;
                        trace!(
                            "{:?}->{:?} is an upward crossing, incrementing winding - {}",
                            pt1,
                            pt2,
                            winding
                        );
                    }
                }
            } else {
                // Downward crossing, point is to the right of the segment
                if pt2.y <= point.y {
                    trace!("Evaluating potential downward crossing");
                    if orient(pt1, pt2, point) <= 0 {
                        winding -= 1;
                        trace!(
                            "{:?}->{:?} is a downward crossing, decrementing winding - {}",
                            pt1,
                            pt2,
                            winding
                        );
                    }
                }
            }
        }
        winding != 0
    };

    let aabb_within_or_bordering_polygon = |square: &AABB, polygon: &Vec<Point>| -> bool {
        // Check if each corner of the aabb lies within the polygon.
        for current_x in [square.tl.x, square.br.x] {
            for current_y in [square.tl.y, square.br.y] {
                let point = Point {
                    x: current_x,
                    y: current_y,
                };
                // first check polygon overlap
                if point_on_polygon_border(&point, polygon) {
                    debug!("{:?} lies on polygon border.", point);
                    continue;
                }
                // use winding count to determine if the point lies within the polygon bounds.
                if !point_within_polygon_winding(&point, polygon) {
                    debug!(
                        "Square {:?} - {:?} is not within polygon, corner {:?} is outside of polygon.",
                        square.tl, square.br, point
                    );
                    return false;
                }
            }
        }

        let square_corners = [
            Point {
                x: square.tl.x,
                y: square.tl.y,
            },
            Point {
                x: square.br.x,
                y: square.tl.y,
            },
            Point {
                x: square.br.x,
                y: square.br.y,
            },
            Point {
                x: square.tl.x,
                y: square.br.y,
            },
        ];
        // now, check if any edge of the polygon crosses the square interior.
        for current_idx in 0..polygon.len() {
            let p1 = &polygon[current_idx];
            let p2 = &polygon[(current_idx + 1) % polygon.len()];

            for square_idx in 0..square_corners.len() {
                let p3 = &square_corners[square_idx];
                let p4 = &square_corners[(square_idx + 1) % square_corners.len()];
                if line_intersect_interior(p1, p2, p3, p4) {
                    return false;
                }
                // check if collinear and if segmsnts overlap
                if lines_collinear_and_overlap(p1, p2, p3, p4) {
                    trace!(
                        "Checking line collinearity between: {:?} -> {:?}, {:?} -> {:?}",
                        p1,
                        p2,
                        p3,
                        p4
                    );
                    // if they do, check directonality, to see if polygon segment intrudes into square area
                    let square_dir = p4.sub(p3);
                    let polygon_dir = p2.sub(p1);
                    let s = cross(&square_dir, &polygon_dir);
                    if s < 0 {
                        // polygon interior is on the left side of the line intersection, these do not overlap.
                        debug!("Polygon intrudes in test square area. Square is not within or along polygon boundary.");
                        return false;
                    }
                }
            }
        }

        return true;
    };

    let mut potential_squares = Vec::new();

    // for each possible square...
    for point1_idx in 0..points.len() {
        for point2_idx in point1_idx + 1..points.len() {
            let square_size = points[point1_idx].square(&points[point2_idx]);

            potential_squares.push((point1_idx, point2_idx, square_size));
        }
    }

    potential_squares.sort_by(|a, b| b.2.cmp(&a.2));

    // start with worst possible solution
    let best_idx = Arc::new(AtomicUsize::new(usize::MAX));

    let num_threads = std::thread::available_parallelism().unwrap().get();
    let num_cases = potential_squares.len();
    let cases = Arc::new(potential_squares);
    let points = Arc::new(points);

    let evaluate_square_in_or_bordering_polygon =
        |(point1_idx, point2_idx, square_size): &(usize, usize, u64),
         points: &Vec<Point>|
         -> Result<u64> {
            let p1 = points[*point1_idx];
            let p2 = points[*point2_idx];
            let aabb = AABB {
                tl: Point {
                    x: p1.x.min(p2.x),
                    y: p1.y.min(p2.y),
                },
                br: Point {
                    x: p1.x.max(p2.x),
                    y: p1.y.max(p2.y),
                },
            };

            info!(
                "Evaluating potential square: {:?} - {:?} : {}",
                points[*point1_idx], points[*point2_idx], square_size
            );

            // test each point in the square if it is within the polygon.
            if !aabb_within_or_bordering_polygon(&aabb, &points) {
                info!(
                    "Square {:?} - {:?} does not lie exclusively within polygon or along border.",
                    aabb.tl, aabb.br
                );
            } else {
                info!(
                    "Largest square within polygon: {:?} - {:?} : {}",
                    points[*point1_idx], points[*point2_idx], square_size
                );
                return Ok(*square_size);
            }
            Err(anyhow::anyhow!("Square not within polygon."))
        };

    // spin up worker threads to evaluate multiple cases in parallel.
    thread::scope(|s| {
        for t_idx in 0..num_threads {
            let cases = cases.clone();
            let best_idx = best_idx.clone();
            let points = points.clone();
            s.spawn(move || {
                let mut i = t_idx;
                while i < num_cases {
                    // branch and bound prune
                    let current_best = best_idx.load(Ordering::Relaxed);
                    if i >= current_best {
                        break;
                    }

                    if evaluate_square_in_or_bordering_polygon(&cases[i], &points).is_ok() {
                        best_idx.fetch_min(i, Ordering::Relaxed);
                        break;
                    }

                    i += num_threads;
                }
            });
        }
    });

    let result = best_idx.load(Ordering::Relaxed);
    if result == usize::MAX {
        Err(anyhow::anyhow!("unable to converge on solution"))
    } else {
        Ok(cases[result].2)
    }
}
