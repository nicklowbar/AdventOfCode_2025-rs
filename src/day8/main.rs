use anyhow::Result;
use displaydoc::Display;
use regex::Regex;
use shared::shared_main;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::ffi::os_str::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use tracing::{debug, info};

fn main() -> Result<()> {
    shared_main(solution1, solution2)
}

#[derive(PartialEq, PartialOrd, Eq, Hash, Display, Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let sum = (other.x - self.x).pow(2) + (other.y - self.y).pow(2) + (other.z - self.z).pow(2);
        (sum as f64).sqrt()
    }
}

#[derive(Display, Debug)]
struct Edge<'a> {
    p1: &'a Point,
    p2: &'a Point,
    distance: f64,
}

fn solution1(input: &File) -> Result<u64> {
    let point_regex = Regex::new(r"^([0-9]*),([0-9]*),([0-9]*)\s*$")?;
    let mut solution = 0u64;
    //let num_connections = 10; // Test Value
    let num_connections = 1000; // input value
    let num_clusters = 3;

    let mut input_reader = BufReader::new(input);

    let mut points = Vec::new();
    let mut point_clusters = HashMap::<&Point, usize>::new();
    let mut clusters = HashMap::<usize, HashSet<&Point>>::new();

    loop {
        let mut input_buf = Vec::new();
        let delim = b'\n';
        let input_bytes = input_reader.read_until(delim, &mut input_buf)?;
        if input_bytes == 0 {
            break;
        }

        let input_string = String::from_utf8(input_buf)?;
        let Some((_, [x, y, z])) = point_regex.captures(&input_string).map(|c| c.extract()) else {
            panic!("Invalid input format: {input_string}")
        };

        let x = x.parse::<i64>()?;
        let y = y.parse::<i64>()?;
        let z = z.parse::<i64>()?;
        let point = Point { x, y, z };
        points.push(point);
    }

    let mut edges = Vec::<Edge>::with_capacity(points.len().pow(2) - points.len());

    // step 1: compute distance between all points:
    for idx1 in 0..points.iter().count() {
        for idx2 in idx1 + 1..points.iter().count() {
            let p1 = &points[idx1];
            let p2 = &points[idx2];
            edges.push(Edge {
                p1,
                p2,
                distance: p1.distance(p2),
            });
        }
    }

    // now, sort by smallest distance to largest
    edges.sort_by(|a, b| {
        if a.distance == b.distance {
            Ordering::Equal
        } else if a.distance < b.distance {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    // initialize each point as its own cluster.
    for point in &points {
        let mut hs = HashSet::new();
        hs.insert(point);
        let cluster_id = point_clusters.len();
        point_clusters.insert(&point, cluster_id);
        clusters.insert(cluster_id, hs);
    }

    // finally, create clusters by linking ten points with the smallest distance.
    let mut idx = 0usize;
    while idx < num_connections {
        if idx >= edges.iter().count() {
            panic!("Exceeded existing edges in graph. this should not happen!");
        }
        let current_edge = &edges[idx];
        debug!(
            "Processing edge: ({:?} - {:?}) - {}",
            current_edge.p1, current_edge.p2, current_edge.distance
        );

        let cluster1_id = point_clusters[current_edge.p1];
        let cluster2_id = point_clusters[current_edge.p2];

        if cluster1_id != cluster2_id {
            // small optimization, only copy from smaller set to larger to minimize
            // collection resizing
            let (dst_id, src_id) = if clusters[&cluster1_id].len() >= clusters[&cluster2_id].len() {
                (cluster1_id, cluster2_id)
            } else {
                (cluster2_id, cluster1_id)
            };

            info!(
                "Merging cluster {}: {:?} into {}: {:?}",
                src_id, clusters[&src_id], dst_id, clusters[&dst_id]
            );

            let Some(src_points) = clusters.remove(&src_id) else {
                panic!("Invalid cluster for cluster id: {}", src_id);
            };

            let Some(dst_points) = clusters.get_mut(&dst_id) else {
                panic!("Invalid cluster for cluster id: {}", dst_id);
            };

            // combine the clusters together
            for point in src_points {
                dst_points.insert(point);
                point_clusters.insert(point, dst_id);
            }
        }
        idx += 1;
    }

    // Finally, finally, multiply the sizes of the remaining clusters for the
    // solution.
    solution = 1;

    let mut cluster_sizes: Vec<u64> = clusters
        .iter()
        .map(|cluster| {
            let cluster_size = cluster.1.iter().count();
            info!("Cluster{}: {}", cluster.0, cluster_size);
            cluster_size as u64
        })
        .collect();

    cluster_sizes.sort_by(|a, b| b.cmp(a));

    for idx in 0..num_clusters {
        solution *= cluster_sizes[idx];
    }

    Ok(solution)
}

fn solution2(input: &File) -> Result<u64> {
    let point_regex = Regex::new(r"^([0-9]*),([0-9]*),([0-9]*)\s*$")?;
    let mut solution = 0u64;
    let num_clusters = 1;

    let mut input_reader = BufReader::new(input);

    let mut points = Vec::new();
    let mut point_clusters = HashMap::<&Point, usize>::new();
    let mut clusters = HashMap::<usize, HashSet<&Point>>::new();

    loop {
        let mut input_buf = Vec::new();
        let delim = b'\n';
        let input_bytes = input_reader.read_until(delim, &mut input_buf)?;
        if input_bytes == 0 {
            break;
        }

        let input_string = String::from_utf8(input_buf)?;
        let Some((_, [x, y, z])) = point_regex.captures(&input_string).map(|c| c.extract()) else {
            panic!("Invalid input format: {input_string}")
        };

        let x = x.parse::<i64>()?;
        let y = y.parse::<i64>()?;
        let z = z.parse::<i64>()?;
        let point = Point { x, y, z };
        points.push(point);
    }

    let mut edges = Vec::<Edge>::with_capacity(points.len().pow(2) - points.len());

    // step 1: compute distance between all points:
    for idx1 in 0..points.iter().count() {
        for idx2 in idx1 + 1..points.iter().count() {
            let p1 = &points[idx1];
            let p2 = &points[idx2];
            edges.push(Edge {
                p1,
                p2,
                distance: p1.distance(p2),
            });
        }
    }

    // now, sort by smallest distance to largest
    edges.sort_by(|a, b| {
        if a.distance == b.distance {
            Ordering::Equal
        } else if a.distance < b.distance {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    // initialize each point as its own cluster.
    for point in &points {
        let mut hs = HashSet::new();
        hs.insert(point);
        let cluster_id = point_clusters.len();
        point_clusters.insert(&point, cluster_id);
        clusters.insert(cluster_id, hs);
    }

    // finally, merge clusters until we have a single cluster
    let mut idx = 0usize;
    while clusters.iter().count() > num_clusters {
        if idx >= edges.iter().count() {
            panic!("Exceeded existing edges in graph. this should not happen!");
        }
        let current_edge = &edges[idx];
        debug!(
            "Processing edge: ({:?} - {:?}) - {}",
            current_edge.p1, current_edge.p2, current_edge.distance
        );

        let cluster1_id = point_clusters[current_edge.p1];
        let cluster2_id = point_clusters[current_edge.p2];

        if cluster1_id != cluster2_id {
            // small optimization, only copy from smaller set to larger to minimize
            // collection resizing
            let (dst_id, src_id) = if clusters[&cluster1_id].len() >= clusters[&cluster2_id].len() {
                (cluster1_id, cluster2_id)
            } else {
                (cluster2_id, cluster1_id)
            };

            info!(
                "Merging cluster {}: {:?} into {}: {:?}",
                src_id, clusters[&src_id], dst_id, clusters[&dst_id]
            );

            let Some(src_points) = clusters.remove(&src_id) else {
                panic!("Invalid cluster for cluster id: {}", src_id);
            };

            let Some(dst_points) = clusters.get_mut(&dst_id) else {
                panic!("Invalid cluster for cluster id: {}", dst_id);
            };

            // combine the clusters together
            for point in src_points {
                dst_points.insert(point);
                point_clusters.insert(point, dst_id);
            }
        }
        idx += 1;
    }

    // Get the coordinates of the last connection of boxes that form a complete
    // circuit
    let last_edge = &edges[idx - 1];
    let box1 = last_edge.p1;
    let box2 = last_edge.p2;

    info!(
        "The last two boxes to be connected are: {:?} - {:?}",
        box1, box2
    );
    solution = (box1.x * box2.x) as u64;

    Ok(solution)
}
