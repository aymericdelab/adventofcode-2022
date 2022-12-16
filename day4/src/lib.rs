use std::{cmp, error::Error, fs::read_to_string, num::ParseIntError};

pub fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    //fn read_file(path: &str) -> Result<String, Box<dyn Error>> {
    let content = read_to_string(path)?;
    Ok(content)
}

pub fn parse_file(content: String) -> Vec<Vec<Vec<String>>> {
    let content_by_pair: Vec<&str> = content.split("\n").collect();
    let content_by_elf: Vec<Vec<Vec<String>>> = content_by_pair
        .into_iter()
        .map(|x| {
            x.split(',')
                .map(|x| x.split('-').into_iter().map(|x| x.to_string()).collect())
                .collect()
        })
        .collect();

    content_by_elf
}

pub fn calculate_score_1(elves: Vec<Vec<String>>) -> i32 {
    let elf1: Result<Vec<i32>, ParseIntError> = elves[0]
        .clone()
        .into_iter()
        .map(|x| x.parse::<i32>())
        .collect();
    let elf2: Result<Vec<i32>, ParseIntError> = elves[1]
        .clone()
        .into_iter()
        .map(|x| x.parse::<i32>())
        .collect();
    let elf1_ok = elf1.unwrap();
    let elf2_ok = elf2.unwrap();

    if elf1_ok[0] <= elf2_ok[0] && elf1_ok[1] >= elf2_ok[1] {
        1
    } else if elf1_ok[1] <= elf2_ok[1] && elf1_ok[0] >= elf2_ok[0] {
        1
    } else {
        0
    }
}

pub fn calculate_score_2(elves: Vec<Vec<String>>) -> i32 {
    let elf1: Result<Vec<i32>, ParseIntError> = elves[0]
        .clone()
        .into_iter()
        .map(|x| x.parse::<i32>())
        .collect();
    let elf2: Result<Vec<i32>, ParseIntError> = elves[1]
        .clone()
        .into_iter()
        .map(|x| x.parse::<i32>())
        .collect();
    let elf1_ok = elf1.unwrap();
    let elf2_ok = elf2.unwrap();

    let number_of_overlapping_numbers = cmp::max(
        cmp::min(elf1_ok[1], elf2_ok[1]) - cmp::max(elf1_ok[0], elf2_ok[0]) + 1,
        0,
    );
    if number_of_overlapping_numbers > 0 {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_read_file_passes() {
        let result = read_file("./data/test.txt").unwrap();
        assert_eq!(result.chars().nth(0).unwrap(), '2');
    }

    #[test]
    fn test_read_file_and_parse_file_passes() {
        let result = read_file("./data/test.txt").unwrap();
        assert_eq!(result.chars().nth(0).unwrap(), '2');

        let parsed_result = parse_file(result);
        assert_eq!(parsed_result[0][0], vec!["2".to_string(), "4".to_string()]);
    }

    #[test]
    fn test_calculate_score_passes() {
        let result = read_file("./data/test.txt").unwrap();
        assert_eq!(result.chars().nth(0).unwrap(), '2');

        let parsed_result = parse_file(result);
        assert_eq!(parsed_result[0][0], vec!["2".to_string(), "4".to_string()]);

        let score: i32 = parsed_result
            .into_iter()
            .map(|pair| calculate_score_1(pair))
            .sum();

        assert_eq!(score, 2);
    }
    #[test]
    fn test_calculate_score_1() {
        let result = read_file("./data/input.txt").unwrap();

        let parsed_result = parse_file(result);

        let score: i32 = parsed_result
            .into_iter()
            .map(|pair| calculate_score_1(pair))
            .sum();
        println!("{}", score);
    }

    #[test]
    fn test_calculate_score_2() {
        let result = read_file("./data/input.txt").unwrap();

        let parsed_result = parse_file(result);

        let score: i32 = parsed_result
            .into_iter()
            .map(|pair| calculate_score_2(pair))
            .sum();
        println!("{:?}", score);
    }
}
