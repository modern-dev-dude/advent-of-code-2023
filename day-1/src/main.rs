use std::fs;

fn main()   {
    // let tst: Vec<&str>  =  "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet".split("\n").collect();

     let file_path = "./src/puzzle-input.txt";
    println!("In file {}", file_path);

    let file = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    let contents: Vec<&str> = file.split("\n").collect();
    // let mut buf_reader = BufReader::new(file);
    // let mut contents = String::new();
    // buf_reader.read_to_string(&mut contents);


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

    println!("{}", total.to_string());
}
