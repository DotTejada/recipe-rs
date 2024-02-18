use std::fs;
use std::fs::write;
use fancy_regex::Regex;
use recipe::frac_to_scaled;
use recipe::input_to_scaled;

fn find_numbers(re: Regex, input: &String) -> Vec<String> {
    let nums: Vec<String> = re
        .find_iter(input.as_str())
        .map(|c| c.unwrap().as_str().to_string())
        .collect::<Vec<String>>();
    nums
}

fn scale_up(nums: &Vec<String>, scale: i32) -> Vec<String> {
    let mut scaled_list: Vec<String> = Vec::new();
    for num in nums {
        let scaled_num: String = match num.parse::<i32>() {
            Ok(scaled_num) => (scaled_num * scale).to_string(),
            Err(_) => frac_to_scaled(num.to_string(), scale),
        };
        scaled_list.push(scaled_num);
    }
    scaled_list
}

fn main() {

    let file_path = String::from("src/cookierecipe.txt");

    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the input");

    //println!("{}", input);

    //let lines: Vec<String> = input.lines().map(|c| c.to_string()).collect();

    let scale: i32 = 2;

    let re = Regex::new(r"(?m)([0-9]\/[0-9])|(\d+ [0-9]\/[0-9])|^(\d+)").unwrap();
    let nums = find_numbers(re, &input);
    //println!("{:?}", nums);
    let scaled = scale_up(&nums, scale);
    //println!("{:?}", scaled);

    let scaled_input = input_to_scaled(&input, &nums, &scaled);
    //println!("{:?}", scaled_input);
    //println!("-----------------------------" );
    //println!("{}", scaled_input);

    let output = format!("Original:\r\n{}\r\n-----------------------------\r\nScale of: {}\r\n{}", input, scale, scaled_input);
    println!("{}", output);
    let _ = write("output.txt", output); 

}
