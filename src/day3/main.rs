/*
--- Day 3: Lobby ---

You descend a short staircase, enter the surprisingly vast lobby, and are quickly cleared by the security checkpoint. When you get to the main elevators, however, you discover that each one has a red light above it: they're all offline.

"Sorry about that," an Elf apologizes as she tinkers with a nearby control panel. "Some kind of electrical surge seems to have fried them. I'll try to get them online soon."

You explain your need to get further underground. "Well, you could at least take the escalator down to the printing department, not that you'd get much further than that without the elevators working. That is, you could if the escalator weren't also offline."

"But, don't worry! It's not fried; it just needs power. Maybe you can get it running while I keep working on the elevators."

There are batteries nearby that can supply emergency power to the escalator for just such an occasion. The batteries are each labeled with their joltage rating, a value from 1 to 9. You make a note of their joltage ratings (your puzzle input). For example:

987654321111111
811111111111119
234234234234278
818181911112111

The batteries are arranged into banks; each line of digits in your input corresponds to a single bank of batteries. Within each bank, you need to turn on exactly two batteries; the joltage that the bank produces is equal to the number formed by the digits on the batteries you've turned on. For example, if you have a bank like 12345 and you turn on batteries 2 and 4, the bank would produce 24 jolts. (You cannot rearrange batteries.)

You'll need to find the largest possible joltage each bank can produce. In the above example:

    In 987654321111111, you can make the largest joltage possible, 98, by turning on the first two batteries.
    In 811111111111119, you can make the largest joltage possible by turning on the batteries labeled 8 and 9, producing 89 jolts.
    In 234234234234278, you can make 78 by turning on the last two batteries (marked 7 and 8).
    In 818181911112111, the largest joltage you can produce is 92.

The total output joltage is the sum of the maximum joltage from each bank, so in this example, the total output joltage is 98 + 89 + 78 + 92 = 357.

There are many batteries in front of you. Find the maximum joltage possible from each bank; what is the total output joltage?
s
*/

use anyhow::Result;
use regex::Regex;
use shared::shared_main;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};
use tracing::{info, warn};
fn main() -> Result<()> {
    shared_main(solution1, solution2)
}

fn solution1(input: &File) -> Result<u64> {
    let mut solution = 0;

    let mut input_reader = BufReader::new(input);

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

        let mut range = String::from_utf8(range_bytes)?;
        let mut joltages: Vec<u32> = range
            .chars()
            .map(|c| c.to_digit(10).expect("Invalid integer value"))
            .collect();

        // iterate through the array of joltages, keeping track of the last relative maxima.
        let mut largest_joltage = 0u32;

        let compute_joltage = |a, b| a * 10 + b;

        let mut maxima = 0u32;
        for joltage in joltages {
            largest_joltage = largest_joltage.max(compute_joltage(maxima, joltage));
            maxima = maxima.max(joltage);
        }

        info!("{range}: {largest_joltage}");

        solution += largest_joltage;
    }
    Ok(solution.into())
}

/*
--- Part Two ---

The escalator doesn't move. The Elf explains that it probably needs more joltage to overcome the static friction of the system and hits the big red "joltage limit safety override" button. You lose count of the number of times she needs to confirm "yes, I'm sure" and decorate the lobby a bit while you wait.

Now, you need to make the largest joltage by turning on exactly twelve batteries within each bank.

The joltage output for the bank is still the number formed by the digits of the batteries you've turned on; the only difference is that now there will be 12 digits in each bank's joltage output instead of two.

Consider again the example from before:

987654321111111
811111111111119
234234234234278
818181911112111

Now, the joltages are much larger:

    In 987654321111111, the largest joltage can be found by turning on everything except some 1s at the end to produce 987654321111.
    In the digit sequence 811111111111119, the largest joltage can be found by turning on everything except some 1s, producing 811111111119.
    In 234234234234278, the largest joltage can be found by turning on everything except a 2 battery, a 3 battery, and another 2 battery near the start to produce 434234234278.
    In 818181911112111, the joltage 888911112111 is produced by turning on everything except some 1s near the front.

The total output joltage is now much larger: 987654321111 + 811111111119 + 434234234278 + 888911112111 = 3121910778619.

What is the new total output joltage?
*/

fn solution2(input: &File) -> Result<u64> {
    let mut solution: u64 = 0;

    let mut input_reader: BufReader<&File> = BufReader::new(input);

    let _compute_joltage = |joltages: &mut [u8; 12]| -> u64 {
        let mut sum: u64 = 0;
        joltages.into_iter().for_each(|j| {
            sum *= 10;
            sum += *j as u64;
        });
        sum
    };

    // Axiom 1: the largest n digit number you can create from n+1 consecutive digits is the largest of all possible consecutive arrangements of n digits.
    let compute_max_joltage_minus_index = |joltages: &[u8; 13]| -> (u64, usize) {
        // for each index in the sequence, compute the joltage of the number formed if we remove this number from the sequence.
        let mut max = 0;
        let mut max_idx = 0;
        for idx in 0..13 {
            let mut sum: u64 = 0;
            for jdx in 0..13 {
                if jdx != idx {
                    sum *= 10;
                    sum += joltages[jdx as usize] as u64;
                }
            }
            if sum >= max {
                max = sum;
                max_idx = idx;
            }
        }

        // Compute the largest digit possible if you remove one digit
        (max, max_idx)
    };

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
        let joltages: Vec<u8> = range
            .chars()
            .map(|c| c.to_digit(10).expect("Invalid integer value") as u8)
            .collect();

        // create initial array of digits representing our max number plus our incoming
        let mut max_digits: [u8; 13] = [0; 13];
        max_digits.copy_from_slice(&joltages[0..13]);

        let mut max_joltage = 0;

        // for rest of digits in joltages, greedily keep the 12 digits that make the largest number:
        for idx in 12..joltages.len() {
            // append our next digit to evaluate
            max_digits[12] = joltages[idx];

            let mut index = 0;
            // compute the index to remove, and max joltage value if removed
            (max_joltage, index) = compute_max_joltage_minus_index(&max_digits);

            // remove the value at index `index` and slide all indices to the left
            for jdx in index..12 {
                max_digits[jdx] = max_digits[jdx + 1];
            }
        }
        solution += max_joltage;
        info!("Max joltage: {max_joltage} Sum: {solution}");
    }

    Ok(solution)
}
