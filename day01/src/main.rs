use std::env;
use std::fs;

fn read_file_lines(file_path: String) -> Vec<String> {
    let input_contents = fs::read_to_string(file_path).unwrap();
    return input_contents.lines().map(str::to_string).collect();
}

fn window_sums(arr: &Vec<i32>, size: usize) -> Vec<i32> {
    return arr
        .iter()
        .enumerate()
        .filter_map(|(i, _)| {
            if i >= size - 1 {
                let result = (0..size)
                    .map(|offset| arr[i - offset])
                    .fold(0, |x, y| x + y);
                Some(result)
            } else {
                None
            }
        })
        .collect();
}

fn count_values_greater_than_prev(arr: Vec<i32>) -> usize {
    return arr
        .iter()
        .enumerate()
        .filter(|(i, curr_val)| {
            if *i < arr.len() - 1 {
                **curr_val < arr[i + 1]
            } else {
                false
            }
        })
        .count();
}

fn main() {
    let file_path = env::args().nth(1).expect("Expected file_path for arg 1");
    let window_size = env::args()
        .nth(2)
        .expect("Expected window_size for arg 2")
        .parse::<usize>()
        .expect("Invalid window_size argument");
    let input_lines: Vec<String> = read_file_lines(file_path);
    let raw_measurements: Vec<i32> = input_lines
        .iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let window_sums = window_sums(&raw_measurements, window_size);
    println!("{}", count_values_greater_than_prev(window_sums));
}
