use std::fs;

pub fn score_2(a: String, b: String) -> u32 {
    let a_str = a.as_str();
    let b_str = b.as_str();

    match (a_str, b_str) {
        //scissor
        ("A", "X") => 0 + 3,
        //rock
        ("A", "Y") => 3 + 1,
        //paper
        ("A", "Z") => 6 + 2,

        //rock
        ("B", "X") => 0 + 1,
        //paper
        ("B", "Y") => 3 + 2,
        //scissor
        ("B", "Z") => 6 + 3,

        //paper
        ("C", "X") => 0 + 3,
        //scissor
        ("C", "Y") => 3 + 2,
        //rock
        ("C", "Z") => 6 + 1,
        _ => panic!(),
    }
}

pub fn score_1(a: String, b: String) -> u32 {
    if a == "A".to_string() {
        if b == "X".to_string() {
            3 + 1
        } else if b == "Y" {
            6 + 2
        } else {
            0 + 3
        }
    } else if a == "B".to_string() {
        if b == "X".to_string() {
            0 + 1
        } else if b == "Y" {
            3 + 2
        } else {
            6 + 3
        }
    } else {
        if b == "X".to_string() {
            6 + 1
        } else if b == "Y" {
            0 + 2
        } else {
            3 + 3
        }
    }
}

pub fn read_file_and_parse(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).unwrap();
    content.split("\n").map(|x| x.to_string()).collect()
    // content_vec
}

pub fn compute_score_1(row: String) -> u32 {
    let res: Vec<String> = row.split(" ").map(|x| x.to_string()).collect();
    score_1(res[0].clone(), res[1].clone())
}

pub fn compute_score_2(row: String) -> u32 {
    let res: Vec<String> = row.split(" ").map(|x| x.to_string()).collect();
    score_2(res[0].clone(), res[1].clone())
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
        let results: Vec<u32> = data.iter().map(|x| compute_score_1(x.clone())).collect();
        let result: u32 = results.iter().sum();
        assert_eq!(result, 15);
    }

    #[test]
    fn test_compute_score_1() {
        let data = read_file_and_parse("./data/input.txt");
        let result: u32 = data.iter().map(|x| compute_score_1(x.clone())).sum();
        println!("{}", result);
    }

    #[test]
    fn test_compute_score_2() {
        // let data = read_file_and_parse("./data/test2.txt");
        let data = read_file_and_parse("./data/input.txt");
        let result: u32 = data.iter().map(|x| compute_score_2(x.clone())).sum();
        println!("{}", result);
    }
}
