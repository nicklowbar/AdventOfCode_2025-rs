/*
--- Day 6: Trash Compactor ---

After helping the Elves in the kitchen, you were taking a break and helping them re-enact a movie scene when you over-enthusiastically jumped into the garbage chute!

A brief fall later, you find yourself in a garbage smasher. Unfortunately, the door's been magnetically sealed.

As you try to find a way out, you are approached by a family of cephalopods! They're pretty sure they can get the door open, but it will take some time. While you wait, they're curious if you can help the youngest cephalopod with her math homework.

Cephalopod math doesn't look that different from normal math. The math worksheet (your puzzle input) consists of a list of problems; each problem has a group of numbers that need to be either added (+) or multiplied (*) together.

However, the problems are arranged a little strangely; they seem to be presented next to each other in a very long horizontal list. For example:

123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +

Each problem's numbers are arranged vertically; at the bottom of the problem is the symbol for the operation that needs to be performed. Problems are separated by a full column of only spaces. The left/right alignment of numbers within each problem can be ignored.

So, this worksheet contains four problems:

    123 * 45 * 6 = 33210
    328 + 64 + 98 = 490
    51 * 387 * 215 = 4243455
    64 + 23 + 314 = 401

To check their work, cephalopod students are given the grand total of adding together all of the answers to the individual problems. In this worksheet, the grand total is 33210 + 490 + 4243455 + 401 = 4277556.

Of course, the actual worksheet is much wider. You'll need to make sure to unroll it completely so that you can read the problems clearly.

Solve the problems on the math worksheet. What is the grand total found by adding together all of the answers to the individual problems?
*/

use anyhow::Result;
use regex::Regex;
use shared::shared_main;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use tracing::{debug, info, trace};

fn main() -> Result<()> {
    shared_main(solution1, solution2)
}

enum Operator {
    Multiply,
    Add,
}

fn solution1(input: &File) -> Result<u64> {
    let number_regex = Regex::new(r"^\s*\d+(?:\s+\d+)*\s*$")?;
    let number_extract_regex = Regex::new(r"\d+")?;
    let operation_regex = Regex::new(r"^\s*[+*](?:\s+[+*])*\s*$")?;
    let operation_extract_regex = Regex::new(r"[+*]")?;
    let mut solution = 0;

    let mut input_reader = BufReader::new(input);
    let mut operands = Vec::new();
    let mut operators = Vec::new();
    let mut row_width = 0;

    loop {
        let mut input_buffer = Vec::new();
        let delim = b'\n';
        let bytes_read = input_reader.read_until(delim, &mut input_buffer)?;
        if bytes_read == 0 {
            break;
        }

        let input_str = String::from_utf8(input_buffer)?;
        debug!("Input line: {input_str}");
        if number_regex.is_match(&input_str) {
            for capture in number_extract_regex.find_iter(&input_str) {
                debug!("Capture: {}", capture.as_str());
                operands.push(capture.as_str().parse::<u64>()?);
            }
        } else if operation_regex.is_match(&input_str) {
            for capture in operation_extract_regex.find_iter(&input_str) {
                debug!("Capture: {}", capture.as_str());
                operators.push(match capture.as_str() {
                    "+" => Operator::Add,
                    "*" => Operator::Multiply,
                    err_val => panic!("Unable to parse operand: {err_val}"),
                });
            }
        } else {
            panic!("Invald input format: {input_str}");
        }
    }

    row_width = operators.len();

    info!("Using grid row width of: {row_width}");

    let compute_problem = |grid: Vec<u64>, operators: Vec<Operator>, width| {
        let mut column_solutions = vec![0; width];
        for y in 0..grid.len() / width {
            for x in 0..width {
                match y {
                    0 => column_solutions[x] = grid[y * width + x],
                    _ => match operators[x] {
                        Operator::Add => {
                            column_solutions[x] += grid[y * width + x];
                        }
                        Operator::Multiply => {
                            column_solutions[x] *= grid[y * width + x];
                        }
                    },
                }
            }
        }
        column_solutions
    };

    let column_solutions = compute_problem(operands, operators, row_width);
    info!("column solutions:\n{:?}", column_solutions);
    for column in column_solutions {
        solution += column
    }

    Ok(solution)
}

/*
--- Part Two ---

The big cephalopods come back to check on how things are going. When they see that your grand total doesn't match the one expected by the worksheet, they realize they forgot to explain how to read cephalopod math.

Cephalopod math is written right-to-left in columns. Each number is given in its own column, with the most significant digit at the top and the least significant digit at the bottom. (Problems are still separated with a column consisting only of spaces, and the symbol at the bottom of the problem is still the operator to use.)

Here's the example worksheet again:

123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +

Reading the problems right-to-left one column at a time, the problems are now quite different:

    The rightmost problem is 4 + 431 + 623 = 1058
    The second problem from the right is 175 * 581 * 32 = 3253600
    The third problem from the right is 8 + 248 + 369 = 625
    Finally, the leftmost problem is 356 * 24 * 1 = 8544

Now, the grand total is 1058 + 3253600 + 625 + 8544 = 3263827.

Solve the problems on the math worksheet again. What is the grand total found by adding together all of the answers to the individual problems?
*/

fn solution2(input: &File) -> Result<u64> {
    let mut solution = 0;

    let mut input_reader = BufReader::new(input);
    let mut buf = Vec::new();

    let delim = b'\n';
    let mut row_width = 0;
    loop {
        let mut line_buf = Vec::new();
        let bytes = input_reader.read_until(delim, &mut line_buf)?;

        if bytes == 0 {
            break;
        }

        if line_buf.last() == Some(&delim) {
            line_buf.pop();
        }

        row_width = line_buf.len();
        buf.append(&mut line_buf);
    }

    info!("Using grid row width of: {row_width}");
    debug!("Grid: {:?}", buf);

    let compute_problem = |grid: Vec<u8>, width| {
        let mut problem_solutions = Vec::new();
        let height = grid.len() / width;
        let mut values = Vec::new();
        let mut num_values = 0;
        let mut operator = Operator::Add;
        for x in (0..width).rev() {
            let mut column_has_value = false;
            let mut value: u64 = 0;
            for y in 0..height {
                trace!("Processing character: {}", grid[y * width + x] as char);
                match grid[y * width + x] {
                    b'\n' => continue,
                    b'+' => {
                        operator = Operator::Add;
                        debug!("Setting operator to +");
                    }
                    b'*' => {
                        operator = Operator::Multiply;
                        debug!("Setting operator to *");
                    }
                    val => {
                        if val.is_ascii_digit() {
                            value = value * 10 + (val - b'0') as u64;
                            column_has_value = true;
                            trace!("Appended {val} to column value: {value}");
                        }
                    }
                };
            }
            if !column_has_value || x == 0 {
                if x == 0 {
                    debug!("Adding final value: {value} to list of operands");
                    values.push(value);
                }
                debug!(
                    "Processing list of operands: {:?} with operator {}",
                    values,
                    match operator {
                        Operator::Add => "+",
                        Operator::Multiply => "*",
                    }
                );
                if num_values > 0 {
                    let mut total = 0u64;
                    for &value in &values {
                        if total == 0 {
                            total = value;
                        } else {
                            match operator {
                                Operator::Add => total += value,
                                Operator::Multiply => total *= value,
                            }
                        }
                    }
                    problem_solutions.push(total);
                    values.clear();
                }
            } else {
                debug!("Adding value: {value} to list of operands");
                values.push(value);
                num_values += 1;
            }
        }
        problem_solutions
    };

    let column_solutions = compute_problem(buf, row_width);
    info!("column solutions:\n{:?}", column_solutions);
    for column in column_solutions {
        solution += column
    }

    Ok(solution)
}
