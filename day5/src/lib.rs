use std::fs::read_to_string;

pub fn read_file(path: &str) -> String {
    read_to_string(path).unwrap()
}

pub fn parse_crates(crates_string: &str, col_nb: i32) -> Vec<Vec<String>> {
    let rows: Vec<&str> = crates_string.split("\n").collect();
    let mut parsed_rows: Vec<Vec<&str>> =
        rows.into_iter().map(|x| x.split(' ').collect()).collect();
    parsed_rows.pop();

    let mut crates: Vec<Vec<String>> = Vec::new();
    for _ in 0..col_nb {
        crates.push(vec![]);
    }
    for row in parsed_rows.into_iter() {
        // let row = &parsed_rows[0];
        let mut i = 0;
        let mut index = 0;
        while i < row.len() {
            if row[i] == "" {
                i += 4;
            } else {
                crates[index].push(row[i].to_string().chars().nth(1).unwrap().to_string());
                i += 1;
            }
            index += 1;
        }
    }
    crates
}

pub fn get_result(mut crates: Vec<Vec<String>>, moves: &str, reverse: bool) -> String {
    let moves: Vec<&str> = moves.split(&[' ', '\n']).collect();

    for chunk in moves.chunks(6) {
        let origin: usize = chunk[3].parse().unwrap();
        let destination: usize = chunk[5].parse().unwrap();
        let size: usize = chunk[1].parse().unwrap();
        let mut moved_crates: Vec<String>;
        if reverse {
            moved_crates = crates[origin - 1].drain(0..size).rev().collect();
        } else {
            moved_crates = crates[origin - 1].drain(0..size).collect();
        }
        moved_crates.append(&mut crates[destination - 1]);
        crates[destination - 1] = moved_crates;
    }

    let mut top_crates: String = "".to_string();
    for i_crate in crates.into_iter() {
        if i_crate.len() != 0 {
            top_crates.push_str(&i_crate[0]);
        }
    }
    top_crates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_result_test() {
        let result = read_file("./data/test.txt");
        let parsed_result: Vec<&str> = result.split("\n\n").collect();
        let crates = parse_crates(parsed_result[0], 3);
        let final_result = get_result(crates, parsed_result[1], true);
        assert_eq!(final_result, "CMZ");
    }
    #[test]
    fn test_get_result_reverse() {
        let result = read_file("./data/input.txt");
        let parsed_result: Vec<&str> = result.split("\n\n").collect();
        let crates = parse_crates(parsed_result[0], 9);
        let final_result = get_result(crates, parsed_result[1], true);
        dbg!("reverse", final_result);
    }
    #[test]

    fn test_get_result() {
        let result = read_file("./data/input.txt");
        let parsed_result: Vec<&str> = result.split("\n\n").collect();
        let crates = parse_crates(parsed_result[0], 9);
        let final_result = get_result(crates, parsed_result[1], false);
        dbg!("non reverse", final_result);
    }
}
