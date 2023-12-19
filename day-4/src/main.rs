use std::collections::HashMap;
use std::fs;

fn part_one() {
    let file_path = "./input.txt";
    let file = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents: Vec<&str> = file.split("\n").collect();
    let mut total: i32 = 0;
    for card in contents.iter() {
        // get card info
        // split by | and find numbers
        // check for winning numbers in idx 0 from idx 1
        let card_parts: Vec<&str> = card.split(":").collect();
        let card_nums = card_parts[1];
        let card_num_parts: Vec<&str> = card_nums.split("|").collect();

        let winning_num_parts = card_num_parts[0];
        let winning_nums: Vec<i32> = winning_num_parts
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        let user_card_parts: &str = card_num_parts[1];
        let user_card_nums: Vec<i32> = user_card_parts
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        let mut matched_nums: Vec<i32> = vec![];
        for user_num in user_card_nums {
            if winning_nums.contains(&user_num) {
                matched_nums.push(user_num);
            }
        }
        if !matched_nums.is_empty() {
            let pow: i32 = i32::pow(2, (matched_nums.len() - 1) as u32);
            total += pow;
        }
    }
    println!("Part one solution: {}", total);
}

fn part_two() {
    let file_path = "./input.txt";
    let file = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents: Vec<&str> = file.split("\n").collect();

    let mut num_card_copies: HashMap<String, i32> = HashMap::new();

    for (card_idx, card) in contents.iter().enumerate() {
        let card_parts: Vec<&str> = card.split(":").collect();
        let card_nums = card_parts[1];
        let card_num_parts: Vec<&str> = card_nums.split("|").collect();

        let winning_num_parts = card_num_parts[0];
        let winning_nums: Vec<i32> = winning_num_parts
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        let user_card_parts: &str = card_num_parts[1];
        let user_card_nums: Vec<i32> = user_card_parts
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        let mut matched_nums: Vec<i32> = vec![];
        for user_num in user_card_nums {
            if winning_nums.contains(&user_num) {
                matched_nums.push(user_num)
            }
        }
        let cur_card_num = card_idx + 1;
        let mut curr_card_copies = 0;
        if let Some(&val) = num_card_copies.get(&cur_card_num.to_string()) {
            curr_card_copies = val;
        }

        if let Some(&val) = num_card_copies.get(&cur_card_num.to_string()) {
            // num_card_copies.insert(cur_card_num.to_string(), &val + 1);
        } else {
            num_card_copies.insert(cur_card_num.to_string(), 1);
        }

        if !matched_nums.is_empty() {
            for i in 0..matched_nums.len() {
                let card_num_for_copies = i + 1 + cur_card_num;
                let cur_card_num_for_copies_str = card_num_for_copies.to_string();

                if let Some(&val) = num_card_copies.get(&cur_card_num_for_copies_str) {
                    num_card_copies.insert(cur_card_num_for_copies_str, &val + curr_card_copies);
                } else {
                    num_card_copies.insert(cur_card_num_for_copies_str, 2);
                }
            }
        }
    }
    let mut result = 0;
    for (key, value) in &num_card_copies {
        result += value;
    }

    println!("result: {}", result);
}

fn main() {
    part_one();
    part_two();
}
