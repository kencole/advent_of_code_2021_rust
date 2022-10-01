use std::env;
use std::fs;

fn read_file_input_strings(file_path: String) -> Vec<String> {
    let input_contents = fs::read_to_string(file_path).unwrap();
    return input_contents.lines().map(str::to_string).collect();
}

fn sum_as_binary_digits(arr: &Vec<i32>) -> i32 {
    return arr.iter().fold(0, |num, elem| (num * 2) + elem);
}

fn most_and_least_common_digits(lines: &Vec<Vec<i32>>) -> (Vec<i32>, Vec<i32>) {
    let mut digit_sums = vec![0i32; lines[0].len()];
    for digits in lines {
        for (i, x) in digits.iter().enumerate() {
            digit_sums[i] += *x;
        }
    }
    let most_common_digits: Vec<i32> = digit_sums
        .iter()
        .map(|sum| {
            let threshold = lines.len() / 2 + (lines.len() - (lines.len() / 2) * 2);
            if *sum as usize >= threshold {
                1i32
            } else {
                0i32
            }
        })
        .collect();
    let least_common_digits = most_common_digits
        .iter()
        .map(|x| if *x == 0i32 { 1i32 } else { 0i32 })
        .collect();
    return (most_common_digits, least_common_digits);
}

fn main() {
    let file_path = env::args().nth(1).expect("Expected file_path for arg 1");
    let should_filter = env::args()
        .nth(2)
        .expect("Expected should_filter for arg 2")
        .parse::<bool>()
        .expect("Invalid with_aim argument");
    let input_lines = read_file_input_strings(file_path);
    let lines: Vec<Vec<i32>> = input_lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| if c == '0' { 0i32 } else { 1i32 })
                .collect()
        })
        .collect();
    if should_filter {
        let lines_copy: Vec<Vec<i32>> = lines.clone();
        let filtered_most_common = (0..lines[0].len()).fold(lines, |lines, idx| {
            if lines.len() == 1 {
                lines
            } else {
                let (most_common_digits, _) = most_and_least_common_digits(&lines);
                lines
                    .into_iter()
                    .filter(|line| line[idx] == most_common_digits[idx])
                    .collect()
            }
        });
        let filtered_least_common = (0..lines_copy[0].len()).fold(lines_copy, |lines, idx| {
            if lines.len() == 1 {
                lines
            } else {
                let (_, least_common_digits) = most_and_least_common_digits(&lines);
                lines
                    .into_iter()
                    .filter(|line| line[idx] == least_common_digits[idx])
                    .collect()
            }
        });
        let oxygen = sum_as_binary_digits(&(filtered_most_common[0]));
        let co2 = sum_as_binary_digits(&(filtered_least_common[0]));

        println!("{} {} {}", oxygen, co2, oxygen * co2);
    } else {
        let (most_common_digits, least_common_digits) = most_and_least_common_digits(&lines);

        let gamma = sum_as_binary_digits(&most_common_digits);
        let epsilon = sum_as_binary_digits(&least_common_digits);

        println!("{}", gamma * epsilon);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_() {
        assert_eq!(sum_as_binary_digits(&vec![]), 0);
    }

    #[test]
    fn test_1() {
        assert_eq!(sum_as_binary_digits(&vec![1]), 1);
    }

    #[test]
    fn test_01() {
        assert_eq!(sum_as_binary_digits(&vec![0, 1]), 1);
    }

    #[test]
    fn test_10() {
        assert_eq!(sum_as_binary_digits(&vec![1, 0]), 2);
    }

    #[test]
    fn test_11() {
        assert_eq!(sum_as_binary_digits(&vec![1, 1]), 3);
    }
}
