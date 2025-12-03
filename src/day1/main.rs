/*
--- Day 1: Secret Entrance ---

The Elves have good news and bad news.

The good news is that they've discovered project management! This has given them the tools they need to prevent their usual Christmas emergency. For example, they now know that the North Pole decorations need to be finished soon so that other critical tasks can start on time.

The bad news is that they've realized they have a different emergency: according to their resource planning, none of them have any time left to decorate the North Pole!

To save Christmas, the Elves need you to finish decorating the North Pole by December 12th.

Collect stars by solving puzzles. Two puzzles will be made available on each day; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

You arrive at the secret entrance to the North Pole base ready to start decorating. Unfortunately, the password seems to have been changed, so you can't get in. A document taped to the wall helpfully explains:

"Due to new security protocols, the password is locked in the safe below. Please see the attached document for the new combination."

The safe has a dial with only an arrow on it; around the dial are the numbers 0 through 99 in order. As you turn the dial, it makes a small click noise as it reaches each number.

The attached document (your puzzle input) contains a sequence of rotations, one per line, which tell you how to open the safe. A rotation starts with an L or R which indicates whether the rotation should be to the left (toward lower numbers) or to the right (toward higher numbers). Then, the rotation has a distance value which indicates how many clicks the dial should be rotated in that direction.

So, if the dial were pointing at 11, a rotation of R8 would cause the dial to point at 19. After that, a rotation of L19 would cause it to point at 0.

Because the dial is a circle, turning the dial left from 0 one click makes it point at 99. Similarly, turning the dial right from 99 one click makes it point at 0.

So, if the dial were pointing at 5, a rotation of L10 would cause it to point at 95. After that, a rotation of R5 could cause it to point at 0.

The dial starts by pointing at 50.

You could follow the instructions, but your recent required official North Pole secret entrance security training seminar taught you that the safe is actually a decoy. The actual password is the number of times the dial is left pointing at 0 after any rotation in the sequence.

For example, suppose the attached document contained the following rotations:

L68
L30
R48
L5
R60
L55
L1
L99
R14
L82

Following these rotations would cause the dial to move as follows:

    The dial starts by pointing at 50.
    The dial is rotated L68 to point at 82.
    The dial is rotated L30 to point at 52.
    The dial is rotated R48 to point at 0.
    The dial is rotated L5 to point at 95.
    The dial is rotated R60 to point at 55.
    The dial is rotated L55 to point at 0.
    The dial is rotated L1 to point at 99.
    The dial is rotated L99 to point at 0.
    The dial is rotated R14 to point at 14.
    The dial is rotated L82 to point at 32.

Because the dial points at 0 a total of three times during this process, the password in this example is 3.

Analyze the rotations in your attached document. What's the actual password to open the door?

 */
use anyhow::Result;
use regex::Regex;
use shared::shared_main;
use std::fs::File;
use std::io::{BufRead, BufReader};
use tracing::info;

fn main() -> Result<()> {
    shared_main(solution1, solution2)
}

fn modulo(input: i32, modulo: i32) -> i32 {
    let remainder = input % modulo;
    if remainder < 0 {
        remainder + modulo
    } else {
        remainder
    }
}

fn solution1(input: &File) -> Result<u64> {
    let re = Regex::new(r"^([LR])(\d+)")?; // L/R, then a number

    let mut password = 0;
    let mut dial = 50i32;
    let m = 100i32;

    let reader = BufReader::new(input);

    for line in reader.lines() {
        let line = line?;
        if let Some((_, [direction, amount])) = re.captures(&line).map(|c| c.extract()) {
            info!(r"Processing input: {direction}, {amount}");

            let amount: i32 = amount.parse()?;
            match direction {
                "R" => dial = dial + amount,
                "L" => dial = dial - amount,
                _ => panic!("Invalid value:{direction}"),
            }

            dial = modulo(dial, m);
            match dial {
                0 => {
                    password = password + 1;
                    info!("Dial is {dial}, incrementing password!");
                }
                _ => {
                    info!("Dial is {dial}")
                }
            }
        }
    }

    Ok(password)
}

fn solution2(input: &File) -> Result<u64> {
    let re = Regex::new(r"^([LR])(\d+)")?; // L/R, then a number

    let mut password = 0;
    let mut dial = 50i32;
    let m = 100i32;

    let reader = BufReader::new(input);

    for line in reader.lines() {
        let line = line?;
        if let Some((_, [direction, amount])) = re.captures(&line).map(|c| c.extract()) {
            info!(r"Processing input: {direction}, {amount}");

            let amount: i32 = amount.parse()?;
            let increment = match direction {
                "R" => 1,
                "L" => -1,
                _ => panic!("Invalid value:{direction}"),
            };

            for _ in 0..amount {
                dial += increment;
                dial = modulo(dial, m);
                if dial == 0 {
                    password = password + 1;
                    info!("Incrementing password: {password}");
                }
            }

            info!("Dial is {dial}");
        }
    }
    Ok(password)
}
