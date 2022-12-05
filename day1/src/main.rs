use std::{fs, u32, vec};

fn read_file_and_parse(path: &str) -> Vec<Vec<u32>> {
    let calories = fs::read_to_string(path).unwrap();
    let calories_per_elf: Vec<&str> = calories.split("\n\n").collect();
    calories_per_elf
        .iter()
        .map(|x| {
            x.split('\n')
                .map(|x| x.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

fn calculate_max_amounts(max_nb: usize, vector: Vec<Vec<u32>>) -> u32 {
    let mut top = vec![0; max_nb];
    for calories in vector {
        let amount = calories.iter().sum();
        if amount > top[0] {
            top[0] = amount;
            top.sort();
        }
    }
    top.iter().sum()
}
fn calculate_max_amount(vector: Vec<Vec<u32>>) -> u32 {
    let mut biggest_amount = 0;
    for calories in vector {
        let amount = calories.iter().sum();
        if amount > biggest_amount {
            biggest_amount = amount
        }
    }
    biggest_amount
}

fn main() {
    let calories_per_elf = read_file_and_parse("./data/input.txt");

    // let biggest_amount = calculate_max_amount(calories_per_elf);
    // println!("{:?}", biggest_amount);

    let three_biggest_amounts = calculate_max_amounts(3, calories_per_elf);
    println!("{:?}", three_biggest_amounts);
}
