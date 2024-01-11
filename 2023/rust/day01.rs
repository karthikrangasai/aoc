use std::cmp::{max, min};
use std::{env, fs};

static STR_NUMS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

static NUMS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

fn get_calibration_value_a(contents: &str) -> i32 {
    contents
        .split_whitespace()
        .map(|line| {
            let numeric_chars: Vec<i32> = line
                .chars()
                .map(|c| match c.to_digit(10) {
                    Some(i) => i as i32,
                    None => -1,
                })
                .filter(|i| i != &-1)
                .collect();

            (numeric_chars[0] * 10 + numeric_chars[numeric_chars.len() - 1]) as i32
        })
        .sum::<i32>()
}

fn get_calibration_value_b(contents: &str) -> i32 {
    contents
        .split_whitespace()
        .map(|line| {
            println!("Line = {:?}", line);
            let mut min_num: (i32, usize) = (i32::MAX, 0 as usize);
            let mut max_num: (i32, usize) = (i32::MIN, 0 as usize);

            for (num, str_num) in STR_NUMS.iter().enumerate() {
                let all_occurences = line.match_indices(str_num).collect::<Vec<_>>();

                if all_occurences.len() == 0 {
                    continue;
                }

                min_num = if min(all_occurences[0].0 as i32, min_num.0) == min_num.0 {
                    min_num
                } else {
                    (all_occurences[0].0 as i32, num)
                };

                max_num = if max(all_occurences[all_occurences.len() - 1].0 as i32, max_num.0)
                    == max_num.0
                {
                    max_num
                } else {
                    (all_occurences[all_occurences.len() - 1].0 as i32, num)
                };
            }

            for (num, str_num) in NUMS.iter().enumerate() {
                let all_occurences = line.match_indices(str_num).collect::<Vec<_>>();

                if all_occurences.len() == 0 {
                    continue;
                }

                min_num = if min(all_occurences[0].0 as i32, min_num.0) == min_num.0 {
                    min_num
                } else {
                    (all_occurences[0].0 as i32, num)
                };

                max_num = if max(all_occurences[all_occurences.len() - 1].0 as i32, max_num.0)
                    == max_num.0
                {
                    max_num
                } else {
                    (all_occurences[all_occurences.len() - 1].0 as i32, num)
                };
            }

            let x = (min_num.1 * 10 + max_num.1) as i32;

            println!("  Num = {:?}", x);
            x
        })
        .sum::<i32>()
}

fn main() {
    let input_file_path = env::current_dir()
        .expect("Could not retreive the current directory")
        .parent()
        .expect("The directory 'inputs' does not exist.")
        .join("inputs")
        .join("day01.txt");

    let contents = fs::read_to_string(input_file_path).expect("Unable to read the input file.");

    println!("Sum = {}", get_calibration_value_b(&contents));
}
