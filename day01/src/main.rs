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

fn count_windows_greater_than_prev(arr: Vec<i32>, window_size: usize) -> usize {
    let window_sums = window_sums(&arr, window_size);
    return count_values_greater_than_prev(window_sums);
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
    let windows_greater_than_prev = count_windows_greater_than_prev(raw_measurements, window_size);
    println!("{}", windows_greater_than_prev);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list() {
        assert_eq!(count_windows_greater_than_prev(vec![], 1), 0);
        assert_eq!(count_windows_greater_than_prev(vec![], 2), 0);
        assert_eq!(count_windows_greater_than_prev(vec![], 3), 0);
    }

    #[test]
    fn test_single_list() {
        assert_eq!(count_windows_greater_than_prev(vec![1], 1), 0);
    }

    #[test]
    fn test_some_small_lists() {
        assert_eq!(count_windows_greater_than_prev(vec![1, 2], 1), 1);
        assert_eq!(count_windows_greater_than_prev(vec![-3, 3], 1), 1);
        assert_eq!(count_windows_greater_than_prev(vec![5, 4, 3], 1), 0);
        assert_eq!(count_windows_greater_than_prev(vec![5, 4, 5], 1), 1);
        assert_eq!(count_windows_greater_than_prev(vec![1, 7, 8, 9], 1), 3);
    }
    #[test]
    fn test_some_small_windows() {
        assert_eq!(count_windows_greater_than_prev(vec![1, 9, 2, 5], 1), 2);
        assert_eq!(count_windows_greater_than_prev(vec![1, 9, 2, 5], 2), 1);
        assert_eq!(count_windows_greater_than_prev(vec![1, 9, 2, 5], 3), 1);
    }
}
