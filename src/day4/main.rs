/*
--- Day 4: Printing Department ---

You ride the escalator down to the printing department. They're clearly getting ready for Christmas; they have lots of large rolls of paper everywhere, and there's even a massive printer in the corner (to handle the really big print jobs).

Decorating here will be easy: they can make their own decorations. What you really need is a way to get further into the North Pole base while the elevators are offline.

"Actually, maybe we can help with that," one of the Elves replies when you ask for help. "We're pretty sure there's a cafeteria on the other side of the back wall. If we could break through the wall, you'd be able to keep moving. It's too bad all of our forklifts are so busy moving those big rolls of paper around."

If you can optimize the work the forklifts are doing, maybe they would have time to spare to break through the wall.

The rolls of paper (@) are arranged on a large grid; the Elves even have a helpful diagram (your puzzle input) indicating where everything is located.

For example:

..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.

The forklifts can only access a roll of paper if there are fewer than four rolls of paper in the eight adjacent positions. If you can figure out which rolls of paper the forklifts can access, they'll spend less time looking and more time breaking down the wall to the cafeteria.

In this example, there are 13 rolls of paper that can be accessed by a forklift (marked with x):

..xx.xx@x.
x@@.@.@.@@
@@@@@.x.@@
@.@@@@..@.
x@.@@@@.@x
.@@@@@@@.@
.@.@.@.@@@
x.@@@.@@@@
.@@@@@@@@.
x.x.@@@.x.

Consider your complete diagram of the paper roll locations. How many rolls of paper can be accessed by a forklift?

*/

use anyhow::Result;
use shared::shared_main;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::{thread, u64};
use tracing::{debug, info, trace};

fn main() -> Result<()> {
    shared_main(solution1, solution2)
}

fn solution1(input: &File) -> Result<u64> {
    let solution: u64 = 0;

    let mut input_reader: BufReader<&File> = BufReader::new(input);
    let mut x: usize = 0;
    let mut y: usize = 0;
    // process the input file, getting the grid bounds.
    loop {
        // Read until a delimiter
        let mut range_bytes = Vec::new();
        let delim = b'\n';
        let bytes = input_reader.read_until(delim, &mut range_bytes)?;
        if bytes == 0 {
            break;
        }

        if range_bytes.last() == Some(&delim) {
            range_bytes.pop();
        }

        let range = String::from_utf8(range_bytes)?;

        x = range.len();
        y += 1;
    }

    info!("Input grid has width: {x}, height: {y}");
    // now that we know the bounds, allocate our grid representation
    let mut grid = Vec::<u8>::with_capacity(x * y);

    let mut line = 0;
    // re-read the file, storing the state each row into our grid
    input_reader.seek(SeekFrom::Start(0))?;
    loop {
        // Read until a delimiter
        let mut range_bytes = Vec::new();
        let delim = b'\n';
        let bytes = input_reader.read_until(delim, &mut range_bytes)?;
        if bytes == 0 {
            break;
        }

        if range_bytes.last() == Some(&delim) {
            range_bytes.pop();
        }

        let range = String::from_utf8(range_bytes)?;

        let row: Vec<u8> = range
            .chars()
            .map(|c| match c {
                '@' => 1u8, // 1 = roll of paper
                '.' => 0u8, // 0 = no paper
                c => panic!("Invalid input cell value: {c}"),
            })
            .collect();
        grid.extend_from_slice(&row);

        line += 1;
    }

    info!("Input grid: {:?}", grid);

    // For each element in the input grid, compute if there are 4 or more adjacent rolls to the current roll.
    let arc_grid: Arc<Vec<u8>> = Arc::<Vec<u8>>::new(grid);

    let sum = Arc::new(AtomicU64::new(0));
    let num_threads = 1; //32.min(y);

    let array_2d = |arr: &[u8], width: usize, x: usize, y: usize| -> u8 { arr[y * width + x] };

    let mut handles = Vec::new();
    for i in 0..num_threads {
        let grid = arc_grid.clone();
        let sum = sum.clone();
        handles.push(thread::spawn(move || {
            let mut accessible_rolls: u64 = 0;
            let num_rows = y / num_threads;
            let start_row = num_rows * i;
            let end_row = (start_row + num_rows).min(y);
            debug!("Thread: {i} - Start Row: {start_row}, End Row: {end_row}");
            for row in start_row as i64..end_row as i64 {
                for col in 0i64..x as i64 {
                    // skip any cells that aren't rolls of paper.
                    if array_2d(&grid, x, col as usize, row as usize) == 0 {
                        continue;
                    }
                    let mut num_rolls = 0;
                    for u in (col - 1).max(0)..(col + 2).min(x as i64) {
                        for v in (row - 1).max(0)..(row + 2).min(y as i64) {
                            if !(u == col && v == row) {
                                let value = array_2d(&grid, x, u as usize, v as usize);
                                trace!("Thread: {i} - {col},{row} - Checking neighbor: {u},{v} = {value}");
                                num_rolls += value;
                            }
                        }
                    }
                    // after processing the 3x3 kernel for a given cell, add it to our running count of accessible rolls.
                    debug!("Thread: {i} - {col},{row} - Num Rolls: {num_rolls}");
                    if num_rolls < 4 {
                        debug!("Thread: {i} - Accessible Roll: {col},{row}");
                        accessible_rolls += 1;
                    }
                }
            }

            info!("Thread: {i} - Number of accessible paper towels: {accessible_rolls}");
            sum.fetch_add(accessible_rolls, Ordering::Relaxed);
        }));
    }

    // wait for all threads to complete
    for h in handles {
        h.join().unwrap();
    }

    let solution = sum.load(Ordering::Relaxed);
    info!("Number of accessible rolls of paper: {solution}");
    Ok(solution)
}

fn array_2d (arr: &[u8], width: usize, x: usize, y: usize) -> u8 { arr[y * width + x] }
fn array_2d_mut (arr: &mut[u8], width: usize, x: usize, y: usize) -> & mut u8 { &mut arr[y * width + x] }

fn solution2(input: &File) -> Result<u64> {
    let mut solution: u64 = 0;

    let mut input_reader: BufReader<&File> = BufReader::new(input);
    let mut x: usize = 0;
    let mut y: usize = 0;
    // process the input file, getting the grid bounds.
    loop {
        // Read until a delimiter
        let mut range_bytes = Vec::new();
        let delim = b'\n';
        let bytes = input_reader.read_until(delim, &mut range_bytes)?;
        if bytes == 0 {
            break;
        }

        if range_bytes.last() == Some(&delim) {
            range_bytes.pop();
        }

        let range = String::from_utf8(range_bytes)?;

        x = range.len();
        y += 1;
    }

    info!("Input grid has width: {x}, height: {y}");
    // now that we know the bounds, allocate our grid representation
    let mut grid = Vec::<u8>::with_capacity(x * y);
    let mut mask = vec![1u8; x * y];

    let mut line = 0;
    // re-read the file, storing the state each row into our grid
    input_reader.seek(SeekFrom::Start(0))?;
    loop {
        // Read until a delimiter
        let mut range_bytes = Vec::new();
        let delim = b'\n';
        let bytes = input_reader.read_until(delim, &mut range_bytes)?;
        if bytes == 0 {
            break;
        }

        if range_bytes.last() == Some(&delim) {
            range_bytes.pop();
        }

        let range = String::from_utf8(range_bytes)?;

        let row: Vec<u8> = range
            .chars()
            .map(|c| match c {
                '@' => 1u8, // 1 = roll of paper
                '.' => 0u8, // 0 = no paper
                c => panic!("Invalid input cell value: {c}"),
            })
            .collect();
        grid.extend_from_slice(&row);

        line += 1;
    }

    info!("Input grid: {:?}", grid);

    // For each element in the input grid, compute if there are 4 or more adjacent rolls to the current roll.

    let num_threads = 32.min(y);
    let num_rows = y / num_threads; 
    let chunk_size = num_rows * x; // split the output mask into disjoint chunks for thread processing.

    let reset_mask = |mask: &mut [u8]| {
        mask.iter_mut().for_each(|ele| *ele = 1u8);
    };

    let compute_roll_remove_mask = |mask: &mut [u8], grid: &[u8]| -> Result<u64> {
        debug!("Computing mask for buffer.");
        let arc_grid = Arc::new(grid);
        let removed_rolls = Arc::new(AtomicU64::new(0));
        
        thread::scope(|scope| {
            for (tid, mask_chunk) in mask.chunks_mut(chunk_size).enumerate(){
                let grid = arc_grid.clone();
                let removed_rolls = removed_rolls.clone();
                scope.spawn(move || {
                    let start_row = num_rows * tid;
                    let end_row = (start_row + num_rows).min(y);
                    debug!("Thread: {tid} - Start Row: {start_row}, End Row: {end_row}");
                    let mut part_sum = 0u64;

                    for row in start_row as i64..end_row as i64 {
                        for col in 0i64..x as i64 {
                            // skip any cells that aren't rolls of paper.
                            if array_2d(&grid, x, col as usize, row as usize) == 0 {
                                continue;
                            }
                            let mut num_rolls = 0;
                            for u in (col - 1).max(0)..(col + 2).min(x as i64) {
                                for v in (row - 1).max(0)..(row + 2).min(y as i64) {
                                    if !(u == col && v == row) {
                                        let value = array_2d(&grid, x, u as usize, v as usize);
                                        trace!("Thread: {tid} - {col},{row} - Checking neighbor: {u},{v} = {value}");
                                        num_rolls += value;
                                    }
                                }
                            }
                            // after processing the 3x3 kernel for a given cell, add it to our running count of accessible rolls.
                            trace!("Thread: {tid} - {col},{row} - Num Rolls: {num_rolls}");
                            if num_rolls < 4 {
                                debug!("Thread: {tid} - Accessible Roll: {col},{row}");
                                *array_2d_mut(mask_chunk, x, col as usize, row as usize - start_row) = 0;
                                part_sum += 1;
                                
                            }
                        }
                    }
                    removed_rolls.fetch_add(part_sum, Ordering::Relaxed); 
                     
                });
            }
        }); // wait for all threads to complete
        Ok(removed_rolls.load(Ordering::Relaxed))
    };

    let apply_mask = |buffer: &mut[u8], mask: &[u8]| -> Result<()> {
        debug!("Applying mask to buffer.");
        thread::scope(|scope| {
            for (tid, buffer_chunk) in buffer.chunks_mut(chunk_size).enumerate(){
                let mask = Arc::new(mask);
                scope.spawn(move || {
                    let start_row = num_rows * tid;
                    let end_row = (start_row + num_rows).min(y);
                    debug!("Thread: {tid} - Start Row: {start_row}, End Row: {end_row}");
                    for row in start_row as i64..end_row as i64 {
                        for col in 0i64..x as i64 {
                            // Multiply the mask over the buffer to remove the paper roll
                            *array_2d_mut(buffer_chunk, x, col as usize, row as usize - start_row) *= array_2d(&mask, x, col as usize, row as usize);
                            if array_2d(buffer_chunk, x, col as usize, row as usize - start_row) == 1{
                            }
                        }
                    }
                });
            }
        }); // wait for all threads to complete
        Ok(())
    };

    let mut current_removed = u64::MAX;
    while current_removed != 0 {
        reset_mask(&mut mask);
        current_removed = compute_roll_remove_mask(&mut mask, &grid)?;
        solution += current_removed;
        apply_mask(&mut grid, &mask)?;
        info!("Removed {current_removed} paper rolls. Total: {solution}");
    }

    info!("Total removed rolls of paper: {solution}");
    Ok(solution)
}
