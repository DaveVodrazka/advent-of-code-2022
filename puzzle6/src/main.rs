use std::{collections::HashSet, fs};

// part one is size 4, part two is size 14
const BUFFER_SIZE: usize = 14;

type Buffer = [char; BUFFER_SIZE];

fn get_input_content() -> String {
    let file_path = "./puzzle6/input.txt";
    let contents = fs::read_to_string(file_path).expect("Failed to read the file");
    contents
}

fn is_unique(b: &Buffer) -> bool {
    let mut set = HashSet::new();
    for v in b {
        set.insert(v);
    }
    b.len() == set.len()
}

fn main() {
    let content = get_input_content();

    // need to use one of first chars to prevent terminating too early
    let ch = content.chars().next().unwrap();
    let mut buffer: Buffer = [ch; BUFFER_SIZE];
    let mut position: usize = 0;
    let mut count: u32 = 0;

    for c in content.chars() {
        buffer[position] = c;
        position += 1;
        position %= BUFFER_SIZE;
        count += 1;

        if is_unique(&buffer) {
            break;
        }
    }

    println!("Done! {}", count);
}
