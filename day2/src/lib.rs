use std::fs;

pub fn score(a: String, b: String) -> u32 {
    if a == "A".to_string() {
        if b == "X".to_string() {
            3 + 1
        } else if b == "Y" {
            6 + 1
        } else {
            0 + 1
        }
    } else if a == "B".to_string() {
        if b == "X".to_string() {
            0 + 2
        } else if b == "Y" {
            3 + 2
        } else {
            6 + 2
        }
    } else {
        if b == "X".to_string() {
            6 + 3
        } else if b == "Y" {
            0 + 3
        } else {
            3 + 3
        }
    }
}

pub fn read_file_and_parse(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).unwrap();
    let content_vec: Vec<String> = content.split("\n").map(|x| x.to_string()).collect();
    content_vec
}

pub fn compute_score(row: String) -> u32 {
    let res: Vec<String> = row.split(" ").map(|x| x.to_string()).collect();
    score(res[0].clone(), res[1].clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_file() {
        let data = read_file_and_parse("./data/input.txt");
        assert_eq!(data.len(), 2500);
        println!("{}", data[2499]);
    }

    #[test]
    fn test_algo_on_test_data() {
        let data = read_file_and_parse("./data/test.txt");
        let results: Vec<u32> = data.iter().map(|x| compute_score(x.clone())).collect();
        let result: u32 = results.iter().sum();
        assert_eq!(result, 15);
    }

    #[test]
    fn compute_scores() {
        let data = read_file_and_parse("./data/input.txt");
        let results: Vec<u32> = data.iter().map(|x| compute_score(x.clone())).collect();
        let result: u32 = results.iter().sum();
        println!("{}", result);
    }
}
