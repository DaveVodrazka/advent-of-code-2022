use std::fs;

// A rock - 1
// B paper - 2
// C scissor - 3
// X - loss - 2
// Y - draw - 0
// Z - win - 1

// win - 6
// draw - 3
// loss - 0

const LETTERS: [char; 6] = ['A', 'B', 'C', 'Y', 'Z', 'X'];
const WIN_POINTS: i32 = 6;
const DRAW_POINTS: i32 = 3;

fn map_letter_to_value(ch: char) -> i32 {
    let index = LETTERS.iter().position(|&r| r == ch).unwrap();
    (index % 3 + 1) as i32 // 0 -> 1 ...
}

fn evaluate_position(first: char, second: char) -> i32 {
    let mut score: i32 = 0;

    let val_first = map_letter_to_value(first);
    let position_value = map_letter_to_value(second) - 1;

    let val_second = (val_first - 1 + position_value) % 3 + 1;

    match position_value {
        1 => score += WIN_POINTS,
        0 => score += DRAW_POINTS,
        _ => {}
    }

    println!(
        "First {}, Second {}, Result {} - {}",
        val_first, val_second, second, position_value
    );

    score + val_second
}

fn main() {
    let file_path = "./puzzle2/input.txt";
    let contents = fs::read_to_string(file_path).expect("Failed to read the file");
    let lines = contents.split("\n");

    let mut score = 0;

    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        score += evaluate_position(chars[0], chars[2]);
    }

    println!("Final score: {score}")
}
