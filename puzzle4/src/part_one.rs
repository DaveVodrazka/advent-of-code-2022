extern crate regex;
use regex::Regex;
use std::fs;

fn get_input_content() -> String {
    let file_path = "./puzzle4/input.txt";
    let contents = fs::read_to_string(file_path).expect("Failed to read the file");
    contents
}

fn is_range_included(arr: [u16; 4]) -> bool {
    if arr[0] <= arr[2] && arr[1] >= arr[3] {
        return true;
    }
    if arr[0] >= arr[2] && arr[1] <= arr[3] {
        return true;
    }
    false
}

fn get_numbers(s: &str) -> [u16; 4] {
    let re = Regex::new(r"(\d*)-(\d*),(\d*)-(\d*)").unwrap();
    let captures = re.captures(s).unwrap();
    assert!(captures.len() == 5, "Must capture 4 groups");
    let mut res: [u16; 4] = [0; 4];
    for i in 1..=4 {
        res[i - 1] = String::from(&captures[i])
            .parse::<u16>()
            .expect("Failed to parse str into number!");
    }
    println!("{s} parsed into: {res:?}");
    res
}

fn main() {
    let content = get_input_content();
    let mut count: u32 = 0;

    for line in content.lines() {
        let nums = get_numbers(line);
        if is_range_included(nums) {
            println!("{line} - {nums:?} - is contained");
            count += 1;
        } else {
            println!("{line} - {nums:?} - is NOT contained");
        }
    }

    println!("Done! Final count is {count}");
}
