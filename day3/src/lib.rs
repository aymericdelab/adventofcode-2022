use std::fs::read_to_string;

// question mark : https://www.becomebetterprogrammer.com/rust-question-mark-operator/#:~:text=operator%20in%20Rust%20is%20used,or%20Option%20in%20a%20function.

pub fn read_file_and_parse(path: &str) -> Vec<String> {
    let content = read_to_string(path).unwrap();
    content.split("\n").map(|x| x.to_string()).collect()
}

pub fn divide_two_equal_parts(x: String) -> (String, String) {
    let len = x.len() / 2;
    let split = x.split_at(len);
    return (split.0.to_string(), split.1.to_string());
}

pub fn find_common_char_score(part1: String, part2: String) -> u8 {
    // if part1.bytes().into_iter().map(|x| if (part2.bytes().into_iter().any(|y| x == y)) {return x});
    let mut score: u8 = 0;
    for byte in part1.bytes() {
        if part2.bytes().into_iter().any(|y| byte == y) {
            if byte < 97 {
                score = byte - 38;
            } else {
                score = byte - 96;
            }
            break;
        }
    }
    score
}

pub fn find_common_group_score(group: Vec<String>) -> u8 {
    let mut score: u8 = 0;
    for byte in group[0].bytes() {
        if group[1].bytes().into_iter().any(|x| x == byte) {
            if group[2].bytes().into_iter().any(|y| y == byte) {
                if byte < 97 {
                    score = byte - 38;
                } else {
                    score = byte - 96;
                }
                break;
            }
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use crate::read_file_and_parse;

    use super::*;

    #[test]
    fn test_read_file_and_parse() {
        let data = read_file_and_parse("src/data/input.txt");
        assert_eq!(data[1], "LjnQlqNpjjmpmQlLlqNfZRvQcTWcTSTTZcSQcZ");
    }

    #[test]
    fn test_divide_in_two_parts() {
        let data = read_file_and_parse("src/data/input.txt");
        let result: Vec<(String, String)> = data
            .into_iter()
            .map(|x| divide_two_equal_parts(x))
            .collect();
        assert_eq!(result[0].0, "GwrhJPDJC");
        assert_eq!(result[0].1, "ZFRcwfZWV");
    }

    #[test]
    fn test_find_score() {
        let score = find_common_char_score("PmmdzqPrV".to_string(), "vPwwTWBwg".to_string());
        assert_eq!(score, 42);
    }

    #[test]
    fn test_find_all_scores() {
        let data = read_file_and_parse("src/data/input.txt");
        let result: Vec<(String, String)> = data
            .into_iter()
            .map(|x| divide_two_equal_parts(x))
            .collect();

        let scores: Vec<u8> = result
            .into_iter()
            .map(|(x, y)| find_common_char_score(x, y))
            .collect();

        let final_score: u32 = scores.into_iter().map(|x| x as u32).sum();
        println!("{}", final_score);
    }

    #[test]
    fn find_group_score_test() {
        let data = read_file_and_parse("src/data/input.txt");
        let mut score: u32 = 0;
        for x in (0..data.len()).step_by(3) {
            score += find_common_group_score(vec![
                data[x].clone(),
                data[x + 1].clone(),
                data[x + 2].clone(),
            ]) as u32;
        }

        println!("{}", score);
    }
}
