use std::collections::HashSet;
use std::fs;

const ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

const ASCII_UPPER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn get_input_content() -> String {
    let file_path = "./puzzle3/input.txt";
    let contents = fs::read_to_string(file_path).expect("Failed to read the file");
    contents
}

fn char_to_priority(c: char) -> usize {
    let mut index: usize = 0;
    if ASCII_LOWER.contains(&c) {
        index = ASCII_LOWER.iter().position(|&r| r == c).unwrap() + 1;
    } else if ASCII_UPPER.contains(&c) {
        index = ASCII_UPPER.iter().position(|&r| r == c).unwrap() + 27;
    }
    index
}

fn find_common_char(first: &str, second: &str) -> char {
    let set1: HashSet<char> = first.chars().collect();
    let set2: HashSet<char> = second.chars().collect();
    for c in set1.intersection(&set2) {
        return *c;
    }
    unreachable!("Should always find match")
}

fn main() {
    let content = get_input_content();
    let lines = content.split("\n");
    let mut sum: u32 = 0;

    for line in lines {
        let length = line.len();
        assert!(
            length > 0 && length % 2 == 0,
            "Rucksack should have non-zero even size {length}"
        );
        let (first, second) = line.split_at(length / 2);
        let res = find_common_char(first, second);
        let priority = char_to_priority(res);
        println!("{first}-{second}, common: {res}, priority: {priority}");
        sum += priority as u32;
    }

    println!("Done! Sum of priorities is {sum}");
}
