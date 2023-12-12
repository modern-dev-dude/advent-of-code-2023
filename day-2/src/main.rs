use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn main() {
    let file_path = "./input.txt";
    let file = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let contents: Vec<&str> = file.split("\n").collect();

    //12 red cubes, 13 green cubes, and 14 blue cubes
    let mut cubes: HashMap<&str, i16> = HashMap::new();

    cubes.insert("red", 12);
    cubes.insert("green", 13);
    cubes.insert("blue", 14);

    let mut total: i32 = 0;
    let split_pattern = " ";
    for game in contents.iter() {
        let red_re = Regex::new(r"([0-9]+ red)").unwrap();
        let rc_captures = red_re.captures_iter(game);
        let mut num_rc: i16 = 0;

        for rc in rc_captures {
            let rc_match = rc.get(1).map_or("0", |m| m.as_str());
            let rc_to_add: i16 = rc_match.rsplit(split_pattern).collect::<Vec<_>>()[1]
                .parse()
                .unwrap();

            if let Some(&value) = cubes.get("red") {
                if rc_to_add <= value {
                    num_rc += rc_to_add;
                } else {
                    num_rc = -1;
                    break;
                }
            }
        }

        if num_rc == -1 {
            continue;
        }

        // blue
        let mut num_bc: i16 = 0;
        let blue_re = Regex::new(r"([0-9]+ blue)").unwrap();
        let bc_captures = blue_re.captures_iter(game);

        for bc in bc_captures {
            let bc_match = bc.get(1).map_or("0", |m| m.as_str());
            let bc_to_add: i16 = bc_match.rsplit(split_pattern).collect::<Vec<_>>()[1]
                .parse()
                .unwrap();
            if let Some(&value) = cubes.get("blue") {
                if bc_to_add <= value {
                    num_bc += bc_to_add;
                } else {
                    num_bc = -1;
                    break;
                }
            }
        }

        if num_bc == -1 {
            continue;
        }

        // green
        let mut num_gc: i16 = 0;

        let green_re = Regex::new(r"([0-9]+ green)").unwrap();
        let gc_captures = green_re.captures_iter(game);

        for gc in gc_captures {
            let gc_match = gc.get(1).map_or("0", |m| m.as_str());
            let gc_to_add: i16 = gc_match.rsplit(split_pattern).collect::<Vec<&str>>()[1]
                .parse()
                .unwrap();

            if let Some(&value) = cubes.get("green") {
                if gc_to_add <= value {
                    num_gc += gc_to_add;
                } else {
                    num_gc = -1;
                    break;
                }
            }
        }

        if num_gc == -1 {
            continue;
        }

        let game_id_re = Regex::new(r"Game [0-9]+").unwrap();
        if let Some(mat) = game_id_re.find(game) {
            let game_id: i16 = mat
                .as_str()
                .rsplit(split_pattern)
                .next()
                .unwrap_or_default()
                .parse()
                .unwrap_or_default();
            total += game_id as i32;
        };
    }

    println!("Total: {}", total.to_string());
}
