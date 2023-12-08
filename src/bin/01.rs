use std::vec;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let numbers: Vec<u32> = input
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
        })
        .map(|line| line.parse::<u32>().unwrap())
        .map(|number| {
            let mut first_digit = number;
            while first_digit >= 10 {
                first_digit /= 10;
            }
            let last_digit = number % 10;
            first_digit * 10 + last_digit
        })
        .collect();

    Some(numbers.into_iter().sum())
}

fn parse_line(line: &str) -> u32 {
    let words = vec![
        (1, "1"),
        (2, "2"),
        (3, "3"),
        (4, "4"),
        (5, "5"),
        (6, "6"),
        (7, "7"),
        (8, "8"),
        (9, "9"),
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine"),
    ];

    let n_chars = line.chars().count();
    let mut numbers_found = Vec::new();
    for i in 0..n_chars {
        let substring = &line[i..n_chars];
        for (number, word) in &words {
            if substring.starts_with(*word) {
                numbers_found.push(*number);
            }
        }
    }
    let first_number_found = numbers_found[0];
    let last_number_found = numbers_found[numbers_found.len() - 1];
    first_number_found * 10 + last_number_found
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<_>>();

    let nums = lines
        .iter()
        .map(|line| parse_line(line))
        .collect::<Vec<_>>();

    //println!("{:?}", nums);
    Some(nums.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }

    #[test]
    fn parser_test() {
        let line = "eightwothree".to_string();
        let numbers = parse_line(&line);
        assert_eq!(numbers, 83);
    }
}
