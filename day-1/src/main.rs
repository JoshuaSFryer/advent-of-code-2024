use std::{fs::read_to_string, iter::zip};

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn sum_of_distances(column_a: &Vec<i32>, column_b: &Vec<i32>) -> i32 {
    let mut diff_sum = 0;
    let joint_list = zip(column_a, column_b);
    for pair in joint_list {
        diff_sum += (pair.0 - pair.1).abs();
    }

    diff_sum
}

fn similarity_score(col_a: &[i32], col_b: &[i32]) -> i32 {
    let mut similarity_sum = 0;
    for elem in col_a {
        similarity_sum += elem * col_b.iter().filter(|&n| n == elem).count() as i32;
    }
    similarity_sum
}

fn main() {
    let input_lines = read_lines("input.txt");

    let mut left_list: Vec<i32> = vec![];
    let mut right_list: Vec<i32> = vec![];

    for line in &input_lines {
        let mut split = line.split_whitespace();
        left_list.push(String::from(split.next().unwrap()).parse::<i32>().unwrap());
        right_list.push(String::from(split.next().unwrap()).parse::<i32>().unwrap());
    }

    left_list.sort();
    right_list.sort();

    let diff_sum = sum_of_distances(&left_list, &right_list);
    let similarity_sum = similarity_score(&left_list, &right_list);
    println!("The sum of differences is {}", diff_sum);
    println!("The similarity score is {}", similarity_sum);
}
