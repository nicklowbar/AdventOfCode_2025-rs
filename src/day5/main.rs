/*
--- Day 5: Cafeteria ---

As the forklifts break through the wall, the Elves are delighted to discover that there was a cafeteria on the other side after all.

You can hear a commotion coming from the kitchen. "At this rate, we won't have any time left to put the wreaths up in the dining hall!" Resolute in your quest, you investigate.

"If only we hadn't switched to the new inventory management system right before Christmas!" another Elf exclaims. You ask what's going on.

The Elves in the kitchen explain the situation: because of their complicated new inventory management system, they can't figure out which of their ingredients are fresh and which are spoiled. When you ask how it works, they give you a copy of their database (your puzzle input).

The database operates on ingredient IDs. It consists of a list of fresh ingredient ID ranges, a blank line, and a list of available ingredient IDs. For example:

3-5
10-14
16-20
12-18

1
5
8
11
17
32

The fresh ID ranges are inclusive: the range 3-5 means that ingredient IDs 3, 4, and 5 are all fresh. The ranges can also overlap; an ingredient ID is fresh if it is in any range.

The Elves are trying to determine which of the available ingredient IDs are fresh. In this example, this is done as follows:

    Ingredient ID 1 is spoiled because it does not fall into any range.
    Ingredient ID 5 is fresh because it falls into range 3-5.
    Ingredient ID 8 is spoiled.
    Ingredient ID 11 is fresh because it falls into range 10-14.
    Ingredient ID 17 is fresh because it falls into range 16-20 as well as range 12-18.
    Ingredient ID 32 is spoiled.

So, in this example, 3 of the available ingredient IDs are fresh.

Process the database file from the new inventory management system. How many of the available ingredient IDs are fresh?

*/

use anyhow::Result;
use regex::Regex;
use shared::shared_main;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;
use tracing::{debug, info};

fn main() -> Result<()> {
    shared_main(solution1, solution2)
}

fn solution1(input: &File) -> Result<u64> {
    let range_regex = Regex::new(r"^(\d+)-(\d+)")?;
    let mut solution = 0;

    let mut ranges = Vec::new();

    // process the input file, extracting ranges.
    let mut input_reader = BufReader::new(input);
    loop {
        // Read until a delimiter
        let mut range_bytes = Vec::new();
        let delim = b'\n';
        let bytes = input_reader.read_until(delim, &mut range_bytes)?;
        if bytes == 0 || bytes == 1 {
            break;
        }

        if range_bytes.last() == Some(&delim) {
            range_bytes.pop();
        }

        let range_str = String::from_utf8(range_bytes)?;
        let Some((_, [range_start, range_end])) =
            range_regex.captures(&range_str).map(|c| c.extract())
        else {
            panic!("Unable to parse input range");
        };

        // construct a new range using the extracted values
        ranges.push((range_start.parse::<u64>()?, range_end.parse::<u64>()?));
    }

    // Sort all the ranges by minimum, then maximum.
    ranges.sort_by(|a, b| {
        if a.0 != b.0 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });

    debug!("Ranges: {:?}", ranges);

    // then, iterate and compute the union of all ranges.
    let mut current_idx = 0;
    let mut ranges_length = ranges.len().clone();
    while current_idx < ranges.len() - 1 {
        // if this range overlaps with the next range, merge them.
        let (start1, end1) = ranges[current_idx];
        let (start2, end2) = ranges[current_idx + 1];
        // overlap occurs if the end or start of one range lies between the other range.
        if !(end1 < start2 || start1 > end2) {
            debug!("merging ranges: {start1}-{end1} and {start2}-{end2}");
            ranges[current_idx] = (start1.min(start2), end1.max(end2));
            ranges.remove(current_idx + 1);
            continue;
        }

        current_idx += 1;
    }

    debug!("Ranges after merging: {:?}", ranges);

    let get_containing_range = |ranges: &Vec<(u64, u64)>, value: u64| -> Option<usize> {
        // use binary search to find if a range contains this element.
        match ranges.binary_search_by(|p| {
            if value > p.0 && value > p.1 {
                return Ordering::Less;
            } else if value < p.0 && value < p.1 {
                return Ordering::Greater;
            }
            Ordering::Equal
        }) {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    };

    // skip the newline in the input
    input_reader.read_line(&mut String::new());

    loop {
        let mut value_bytes = Vec::new();
        let delim = b'\n';
        let bytes = input_reader.read_until(delim, &mut value_bytes)?;
        if bytes == 0 {
            break;
        }

        if value_bytes.last() == Some(&delim) {
            value_bytes.pop();
        }

        let value = String::from_utf8(value_bytes)?;
        let value = value.parse::<u64>()?;

        if let Some(index) = get_containing_range(&ranges, value) {
            solution += 1;
            info!(
                "Value: {value} is in range {:?}-{:?}",
                ranges[index].0, ranges[index].1
            )
        }
    }

    Ok(solution)
}

/*
--- Part Two ---

The Elves start bringing their spoiled inventory to the trash chute at the back of the kitchen.

So that they can stop bugging you when they get new inventory, the Elves would like to know all of the IDs that the fresh ingredient ID ranges consider to be fresh. An ingredient ID is still considered fresh if it is in any range.

Now, the second section of the database (the available ingredient IDs) is irrelevant. Here are the fresh ingredient ID ranges from the above example:

3-5
10-14
16-20
12-18

The ingredient IDs that these ranges consider to be fresh are 3, 4, 5, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, and 20. So, in this example, the fresh ingredient ID ranges consider a total of 14 ingredient IDs to be fresh.

Process the database file again. How many ingredient IDs are considered to be fresh according to the fresh ingredient ID ranges?

*/

fn solution2(input: &File) -> Result<u64> {
    let range_regex = Regex::new(r"^(\d+)-(\d+)")?;
    let mut solution = 0;

    let mut ranges = Vec::new();

    // process the input file, extracting ranges.
    let mut input_reader = BufReader::new(input);
    loop {
        // Read until a delimiter
        let mut range_bytes = Vec::new();
        let delim = b'\n';
        let bytes = input_reader.read_until(delim, &mut range_bytes)?;
        if bytes == 0 || bytes == 1 {
            break;
        }

        if range_bytes.last() == Some(&delim) {
            range_bytes.pop();
        }

        let range_str = String::from_utf8(range_bytes)?;
        let Some((_, [range_start, range_end])) =
            range_regex.captures(&range_str).map(|c| c.extract())
        else {
            panic!("Unable to parse input range");
        };

        // construct a new range using the extracted values
        ranges.push((range_start.parse::<u64>()?, range_end.parse::<u64>()?));
    }

    // Sort all the ranges by minimum, then maximum.
    ranges.sort_by(|a, b| {
        if a.0 != b.0 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });

    debug!("Ranges: {:?}", ranges);

    // then, iterate and compute the union of all ranges.
    let mut current_idx = 0;
    let mut ranges_length = ranges.len().clone();
    while current_idx < ranges.len() - 1 {
        // if this range overlaps with the next range, merge them.
        let (start1, end1) = ranges[current_idx];
        let (start2, end2) = ranges[current_idx + 1];
        // overlap occurs if the end or start of one range lies between the other range.
        if !(end1 < start2 || start1 > end2) {
            debug!("merging ranges: {start1}-{end1} and {start2}-{end2}");
            ranges[current_idx] = (start1.min(start2), end1.max(end2));
            ranges.remove(current_idx + 1);
            continue;
        }

        current_idx += 1;
    }

    debug!("Ranges after merging: {:?}", ranges);

    let get_containing_range = |ranges: &Vec<(u64, u64)>, value: u64| -> Option<usize> {
        // use binary search to find if a range contains this element.
        match ranges.binary_search_by(|p| {
            if value > p.0 && value > p.1 {
                return Ordering::Less;
            } else if value < p.0 && value < p.1 {
                return Ordering::Greater;
            }
            Ordering::Equal
        }) {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    };

    for range in ranges {
        // compute the width of this range, and add to our running total of fresh
        // ingredient ids.
        solution += range.1 - range.0 + 1;
    }
    Ok(solution)
}
