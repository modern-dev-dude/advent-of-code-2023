use std::fs;

fn part_one()   {

    let file_path = "./puzzle-input.txt";
    let file = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let contents: Vec<&str> = file.split("\n").collect();

    let mut total: i32 = 0; 

    for item in contents.iter(){
        let mut line_num: Vec<&str> = item.matches(char::is_numeric).collect();
        let default:&str = "0";
        let first_num: String = line_num.get(0).unwrap_or(&default).to_string();
        let last_num: String = line_num.pop().unwrap_or(default).to_string();
        let str_num = [first_num, last_num].join("");
        let num: i32 = str_num.parse().expect("Not a number");
        total += num;
    }

    println!("Part one answer: {}", total.to_string());
}

fn part_two(){
    let num_strings: Vec<String> = vec![
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string(),
    ];

    let file_path = "./puzzle-input.txt";
    let file = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let contents: Vec<&str> = file.split("\n").collect();

     for item in contents.iter(){
        num_strings.iter().enumerate().
        item
     }
    // let index: i32 = contents
    // .iter()
    // .enumerate()
    // .find(|&r| r.1.to_string() == "two".to_string())
    // .unwrap()
    // .0;
    // iterate through the string 
    // if number push to stack
    // if matches str num push to stack
    

}

//one, two, three, four, five, six, seven, eight, and nine
fn main(){
    part_one();
}
