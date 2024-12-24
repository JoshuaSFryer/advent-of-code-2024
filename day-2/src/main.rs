use std::{cmp::Ordering, fs::read_to_string};

struct Report {
    levels: Vec<i32>,
    safe: bool,
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn evaluate_safety(levels: &[i32], skips: u32) -> bool {
    assert!(levels.len() > 1);
    const MAX_DIFFERENCE: u32 = 3;
    // println!("Input: {:?}", levels);

    // Base Case
    if skips < 1 {
        println!("{:?} Unsafe.", &levels);
        return false;
    }

    // Compare the first two elements to see if the sequence is in ascending
    // or descending order. If the first two elements are equal, the report
    // is not safe.
    let ascending_descending = levels[1].cmp(&levels[0]);
    if ascending_descending == Ordering::Equal {
        return false;
    }

    let mut prev_level = levels[0];
    let levels_iter = levels.iter();
    // Burn the first iterator value so that we don't compare the 0th element to itself
    for level in levels_iter.skip(1) {
        if level.cmp(&prev_level) != ascending_descending {
            return false;
        }
        if level.abs_diff(prev_level) > MAX_DIFFERENCE {
            return false;
        }
        prev_level = *level;
    }
    true
}

fn main() {
    let dummy = read_lines("dummy_input.txt");
    let input = read_lines("input.txt");

    let mut safe_count = 0;
    let mut unsafe_count = 0;
    let mut dummy_reports: Vec<Report> = vec![];
    let mut reports: Vec<Report> = vec![];

    const MAX_SKIPS: u32 = 1;

    for line in &dummy {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|digit| digit.parse::<i32>().unwrap())
            .collect();
        let safe = evaluate_safety(&nums[..], MAX_SKIPS);
        println!("Dummy input: {:?}; Safe? {}", &nums, safe);
        dummy_reports.push(Report { safe, levels: nums });
    }

    for line in &input {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|digit| digit.parse::<i32>().unwrap())
            .collect();
        reports.push(Report {
            safe: evaluate_safety(&nums[..], MAX_SKIPS),
            levels: nums,
        });
    }

    for report in &reports {
        if report.safe {
            safe_count += 1;
        } else {
            unsafe_count += 1;
        }
    }

    println!("Number of safe records: {safe_count}");
    println!("Number of unsafe records: {unsafe_count}");
    assert!(safe_count + unsafe_count == reports.len());
    println!("Total number of records: {}", &reports.len());
}
