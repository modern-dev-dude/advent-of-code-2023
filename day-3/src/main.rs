use regex::Regex;
use std::fs;

fn part_one() {
    let file_path = "./input.txt";
    let file = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = file.replace("\n", "");
    let mut line_len: i32 = 0;
    if let Some(idx) = file.find("\n") {
        line_len = idx as i32;
    }

    let contents_len = contents.len();
    // get all matches
    // check adjacent cells
    // add if adjacent
    let nums_reg_ex = Regex::new(r"(\d+)").unwrap();
    let reg_ex_sp_chars = Regex::new(r"[!@#$%^&*()_+\-=\[\]{};':\\|,<>\/?]").unwrap();

    let all_matches = nums_reg_ex.find_iter(&contents);
    let mut total = 0;
    for item in all_matches {
        let num_str = &contents[item.start()..item.end()];
        let length = num_str.len();
        let str_st: i32 = item.start() as i32;

        let r_start = str_st + line_len - 1 as i32;
        let r_end = str_st + line_len + 1 + length as i32;

        for i in r_start..r_end {
            if i >= 0 && i < contents_len as i32 {
                if let Some(c) = contents.chars().nth((i) as usize) {
                    if reg_ex_sp_chars.is_match(&c.to_string()) {
                        if let Ok(num) = num_str.parse::<i32>() {
                            total += num;
                        }
                        break;
                    }
                }
            }
        }

        let l_start = str_st - line_len - 1;
        let l_end = str_st - line_len + 1 + length as i32;

        for k in l_start..l_end {
            if k >= 0 && k < contents_len as i32 {
                if let Some(c) = contents.chars().nth(k as usize) {
                    if reg_ex_sp_chars.is_match(&c.to_string()) {
                        if let Ok(num) = num_str.parse::<i32>() {
                            total += num;
                        }
                        break;
                    }
                }
            }
        }

        // check left and right position
        if item.start() > 0 {
            let l_pos = item.start() - 1;
            if let Some(c) = contents.chars().nth(l_pos) {
                if reg_ex_sp_chars.is_match(&c.to_string()) {
                    if let Ok(num) = num_str.parse::<i32>() {
                        total += num;
                    }
                }
            }
        }

        if item.end() < contents.len() {
            let r_pos = item.end();
            if let Some(c) = contents.chars().nth(r_pos) {
                if reg_ex_sp_chars.is_match(&c.to_string()) {
                    if let Ok(num) = num_str.parse::<i32>() {
                        total += num;
                    }
                }
            }
        }
    }

    println!("Part one result: {}", total);
}

fn part_two() {
    let file_path = "./test.txt";
    let file = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents = file.replace("\n", "");

    let reg_ex_sp_chars = Regex::new(r"[*]").unwrap();
    let all_matches = reg_ex_sp_chars.find_iter(&contents);
    for item in all_matches {
        println!(
            "item {}  Start: {}  End: {}",
            item.as_str(),
            item.start(),
            item.end()
        )
    }
}

fn main() {
    part_one();
    part_two();
}
