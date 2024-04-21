use std::env::args;
use std::env::current_dir;
use std::fs;
use std::fs::write;
use std::process;
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

fn get_currdir_path(file_path: &String) -> String {
    let slash: usize = match file_path.rfind(|c: char| c == '\\' || c == '/') {
        Some(s) => s + 1,
        None => 0,
    };
    let fpath = file_path[slash..file_path.len()].to_string();
    let dir = current_dir().unwrap().to_str().unwrap().to_string();
    let prune = fpath.len() - 4;
    let mut outfile = format!("{}\\{}scaled.txt", dir, fpath[0..prune].to_string());
    outfile = outfile.replace("/", "\\");
    return outfile
}

fn get_recipedir_path(file_path: &String) -> String {
    let fpath = file_path[0..file_path.len()].to_string();
    let prune = fpath.len() - 4;
    let outfile = format!("{}scaled.txt", fpath[0..prune].to_string());
    return outfile
}

fn main() {

    let args: Vec<String> = args().collect();
    if args.len() < 4 {
        eprintln!("Not enough arguments\n
Please use with 'recipe <scale factor> <path to recipe> < -h or -t>'\n
-h for output in exe dir, -t for output in input dir\n
Make sure that the scale factor is > 0");
        process::exit(1);
    }

    let scale: i32 = args[1].parse().unwrap();
    let file_path = &args[2];
    let path = &args[3];
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the input");

    //println!("{}", input);

    let re = Regex::new(r"(?m)([0-9]\/[0-9])|(\d+ [0-9]\/[0-9])|^(\d+)").unwrap();
    let nums = find_numbers(re, &input);
    //println!("{:?}", nums);
    let scaled = scale_up(&nums, scale);
    //println!("{:?}", scaled);

    let scaled_input = input_to_scaled(&input, &nums, &scaled);
    //println!("{:?}", scaled_input);
    //println!("-----------------------------" );
    //println!("{}", scaled_input);

    let output = format!("Original:\r\n{}\r\n-----------------------------\r\nScale of: {}\r\n{}\n", input, scale, scaled_input);
    println!("{output}");

    if path == "-h" {
        let outfile = get_currdir_path(file_path);
        println!("Saved to: {}", outfile);
        let _ = write(outfile, output); 
    } else if path == "-t" {
        let outfile = get_recipedir_path(file_path);
        println!("Saved to: {}", outfile);
        let _ = write(outfile, output); 
    } else {
        eprintln!("Last arg not specified: should be either -h or -t\n 
-h for output in exe dir, -t for output in input dir");
        process::exit(1);
    }

}
