use std::fs;

fn get_input_content() -> String {
    let file_path = "./puzzle9/input.txt";
    let contents = fs::read_to_string(file_path).expect("Failed to read the file");
    contents
}

fn main() {
    let content = get_input_content();

    for line in content.lines() {
        println!("{line}");
    }
}
