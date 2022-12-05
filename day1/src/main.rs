use std::{fs, u32};

fn main() {
    let calories = fs::read_to_string("./data/input.txt").unwrap();
    let calories_per_elf: Vec<&str> = calories.split("\n\n").collect();
    let calories_per_elf_2: Vec<Vec<u32>> = calories_per_elf
        .iter()
        .map(|x| {
            x.split('\n')
                .map(|x| x.to_string().parse().unwrap())
                .collect()
        })
        .collect();

    let mut biggest_amount = 0;
    for calories in calories_per_elf_2 {
        let amount = calories.iter().sum();
        if amount > biggest_amount {
            biggest_amount = amount
        }
    }

    println!("{:?}", biggest_amount);
}
