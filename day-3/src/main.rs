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
        let winning_nums: Vec<i16> = winning_num_parts
            .split_whitespace()
            .filter_map(|s| s.parse::<i16>().ok())
            .collect();

        let user_card_parts: &str = card_num_parts[1];
        let user_card_nums: Vec<i16> = user_card_parts
            .split_whitespace()
            .filter_map(|s| s.parse::<i16>().ok())
            .collect();

        let mut matched_nums: Vec<i16> = vec![];
        for user_num in user_card_nums {
            if winning_nums.contains(&user_num) {
                matched_nums.push(user_num)
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
    println!("Not implemented");
}

fn main() {
    part_one();
    part_two();
}
