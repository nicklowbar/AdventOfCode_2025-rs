/*
--- Day 2: Gift Shop ---

You get inside and take the elevator to its only other stop: the gift shop. "Thank you for visiting the North Pole!" gleefully exclaims a nearby sign. You aren't sure who is even allowed to visit the North Pole, but you know you can access the lobby through here, and from there you can access the rest of the North Pole base.

As you make your way through the surprisingly extensive selection, one of the clerks recognizes you and asks for your help.

As it turns out, one of the younger Elves was playing on a gift shop computer and managed to add a whole bunch of invalid product IDs to their gift shop database! Surely, it would be no trouble for you to identify the invalid product IDs for them, right?

They've even checked most of the product ID ranges already; they only have a few product ID ranges (your puzzle input) that you'll need to check. For example:

11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124

(The ID ranges are wrapped here for legibility; in your input, they appear on a single long line.)

The ranges are separated by commas (,); each range gives its first ID and last ID separated by a dash (-).

Since the young Elf was just doing silly patterns, you can find the invalid IDs by looking for any ID which is made only of some sequence of digits repeated twice. So, 55 (5 twice), 6464 (64 twice), and 123123 (123 twice) would all be invalid IDs.

None of the numbers have leading zeroes; 0101 isn't an ID at all. (101 is a valid ID that you would ignore.)

Your job is to find all of the invalid IDs that appear in the given ranges. In the above example:

    11-22 has two invalid IDs, 11 and 22.
    95-115 has one invalid ID, 99.
    998-1012 has one invalid ID, 1010.
    1188511880-1188511890 has one invalid ID, 1188511885.
    222220-222224 has one invalid ID, 222222.
    1698522-1698528 contains no invalid IDs.
    446443-446449 has one invalid ID, 446446.
    38593856-38593862 has one invalid ID, 38593859.
    The rest of the ranges contain no invalid IDs.

Adding up all the invalid IDs in this example produces 1227775554.

What do you get if you add up all of the invalid IDs?

*/

use anyhow::Result;
use regex::Regex;
use shared::shared_main;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    thread::current,
};
use tracing::{info, warn};
fn main() -> Result<()> {
    shared_main(solution1, solution2)
}

fn solution1(input: &File) -> Result<u64> {
    let range_regex = Regex::new(r"^(\d+)-(\d+)")?;
    let mut solution = 0;

    let mut input_reader = BufReader::new(input);

    let compute_error_num = |half_num: u64, digits: u64| -> Result<u64> {
        Ok(half_num + half_num * 10u64.pow(digits.try_into()?))
    };

    let compute_digits = |num: u64| -> u64 {
        let mut digits = 0;
        let mut num = num;
        while num > 0 {
            num = num / 10u64;
            digits += 1;
        }
        digits
    };

    let mut compute_invalid_code_sum = |range_start: u64, range_end: u64| -> Result<u64> {
        let mut num_invalid = 0;
        let mut tmp = range_start;
        let mut digits = compute_digits(range_start);

        if digits % 2 != 0 {
            warn!("Odd number of digits for range: {range_start}-{range_end}");
            digits += 1;
        }
        info!("Number of digits in: {range_start}: {digits}");

        let start_half_num = range_start / 10u64.pow(digits as u32 / 2);
        info!("Half num of: {range_start} is {start_half_num}");

        let mut current_half_num = start_half_num;
        let error_num_digits = compute_digits(current_half_num);
        let mut current_error_num = compute_error_num(current_half_num, error_num_digits)?;
        while current_error_num <= range_end {
            //info!("Evaluating half num: {current_half_num}, error num:
            // {current_error_num}");
            if current_error_num >= range_start && current_error_num <= range_end {
                num_invalid += 1;
                solution += current_error_num;
                info!("Invalid number in range: {range_start}-{range_end}: {current_error_num} Sum: {solution}")
            }
            current_half_num += 1;

            let error_num_digits = compute_digits(current_half_num);
            current_error_num = compute_error_num(current_half_num, error_num_digits)?;
        }

        Ok(num_invalid)
    };

    loop {
        // Read until a delimiter
        let mut range_bytes = Vec::new();
        let bytes = input_reader.read_until(b',', &mut range_bytes)?;
        if bytes == 0 {
            break;
        }
        let mut range = String::from_utf8(range_bytes)?;
        range.pop();

        // Extract range using a regex.
        let Some((_, [range_start, range_end])) = range_regex.captures(&range).map(|c| c.extract())
        else {
            panic!("Unable to parse input range");
        };

        info!("processing: {range_start}-{range_end}");

        // Finally, process the extracted range.
        let range_start: u64 = range_start.parse()?;
        let range_end: u64 = range_end.parse()?;

        compute_invalid_code_sum(range_start, range_end)?;

        range.clear();
    }
    Ok(solution)
}

/*
--- Part Two ---

The clerk quickly discovers that there are still invalid IDs in the ranges in your list. Maybe the young Elf was doing other silly patterns as well?

Now, an ID is invalid if it is made only of some sequence of digits repeated at least twice. So, 12341234 (1234 two times), 123123123 (123 three times), 1212121212 (12 five times), and 1111111 (1 seven times) are all invalid IDs.

From the same example as before:

    11-22 still has two invalid IDs, 11 and 22.
    95-115 now has two invalid IDs, 99 and 111.
    998-1012 now has two invalid IDs, 999 and 1010.
    1188511880-1188511890 still has one invalid ID, 1188511885.
    222220-222224 still has one invalid ID, 222222.
    1698522-1698528 still contains no invalid IDs.
    446443-446449 still has one invalid ID, 446446.
    38593856-38593862 still has one invalid ID, 38593859.
    565653-565659 now has one invalid ID, 565656.
    824824821-824824827 now has one invalid ID, 824824824.
    2121212118-2121212124 now has one invalid ID, 2121212121.

Adding up all the invalid IDs in this example produces 4174379265.

What do you get if you add up all of the invalid IDs using these new rules?

*/

fn solution2(input: &File) -> Result<u64> {
    let range_regex = Regex::new(r"^(\d+)-(\d+)")?;
    let mut solution = 0;

    let mut input_reader = BufReader::new(input);

    let compute_digits = |num: u64| -> u64 {
        let mut digits = 0;
        let mut num = num;
        while num > 0 {
            num = num / 10u64;
            digits += 1;
        }
        digits
    };

    let mut unique_invalid_codes = HashSet::new();

    let mut compute_invalid_codes = |range_start: u64, range_end: u64| -> Result<()> {
        let mut tmp = range_start;
        let mut digits = compute_digits(range_start);

        info!("Number of digits in: {range_start}: {digits}");

        let mut digits = compute_digits(range_end);
        info!("Number of digits in: {range_end}: {digits}");
        let end_half_num = range_end / 10u64.pow(digits as u32 / 2);
        info!("Half num of: {range_end} is {end_half_num}");

        let append_num = |in_num: &mut u64, appendee: u64, appendee_digits: u64| -> Result<()> {
            *in_num = *in_num * 10u64.pow((appendee_digits).try_into()?) + appendee;
            Ok(())
        };

        let mut current_digit = 1u64;
        let mut current_digits = compute_digits(current_digit);
        let mut current_error_num = 11;
        while current_digit <= end_half_num + 1 {
            //while current_error_num <= range_end {
            //info!("Current digit: {current_digit}, number of digits: {current_digits}");
            while current_error_num <= range_end {
                if current_error_num >= range_start {
                    unique_invalid_codes.insert(current_error_num);
                    info!("Invalid number in range: {range_start}-{range_end}: {current_error_num} Sum: {solution}")
                }

                append_num(&mut current_error_num, current_digit, current_digits)?;
            }
            current_digit += 1;
            current_digits = compute_digits(current_digit);
            current_error_num = current_digit;
            append_num(&mut current_error_num, current_digit, current_digits)?;
        }

        Ok(())
    };

    loop {
        // Read until a delimiter
        let mut range_bytes = Vec::new();
        let bytes = input_reader.read_until(b',', &mut range_bytes)?;
        if bytes == 0 {
            break;
        }
        let mut range = String::from_utf8(range_bytes)?;

        info!("Input: {range}");
        // Extract range using a regex.
        let Some((_, [range_start, range_end])) = range_regex.captures(&range).map(|c| c.extract())
        else {
            panic!("Unable to parse input range");
        };

        // Finally, process the extracted range.
        let range_start: u64 = range_start.parse()?;
        let range_end: u64 = range_end.parse()?;

        info!("processing: {range_start}-{range_end}");

        compute_invalid_codes(range_start, range_end)?;

        range.clear();
    }

    info!("Unique invalid codes:\n");
    let mut invalid_code_string = String::new();
    unique_invalid_codes.iter().for_each(|val| {
        solution += val;
        invalid_code_string.push_str(&format!("{}\n", val).to_string());
    });
    info!("\n{}", invalid_code_string);

    Ok(solution)
}
