use std::collections::HashSet;
use std::fs;
use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn move_in_direction(&mut self, direction: &str) {
        match direction {
            "U" => self.y += 1,
            "D" => self.y -= 1,
            "R" => self.x += 1,
            "L" => self.x -= 1,
            "UR" => {
                self.x += 1;
                self.y += 1;
            }
            "UL" => {
                self.x -= 1;
                self.y += 1;
            }
            "DR" => {
                self.x += 1;
                self.y -= 1;
            }
            "DL" => {
                self.x -= 1;
                self.y -= 1;
            }
            _ => panic!("Invalid direction"),
        }
    }
}

impl Hash for Position {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Position {}

fn get_input_content() -> String {
    let file_path = "./puzzle9/input.txt";
    let contents = fs::read_to_string(file_path).expect("Failed to read the file");
    contents
}

fn get_tail_move_direction(head: &Position, tail: &Position) -> Option<String> {
    let x_dif = head.x - tail.x;
    let y_dif = head.y - tail.y;

    if x_dif.abs() < 2 && y_dif.abs() < 2 {
        return None;
    }

    match (x_dif, y_dif) {
        (0, 2) => Some("U".to_string()),
        (0, -2) => Some("D".to_string()),
        (2, 0) => Some("R".to_string()),
        (-2, 0) => Some("L".to_string()),
        (1, 2) => Some("UR".to_string()),
        (-1, 2) => Some("UL".to_string()),
        (1, -2) => Some("DR".to_string()),
        (-1, -2) => Some("DL".to_string()),
        (2, 1) => Some("UR".to_string()),
        (2, -1) => Some("DR".to_string()),
        (-2, 1) => Some("UL".to_string()),
        (-2, -1) => Some("DL".to_string()),
        _ => panic!("Unexpected combination"),
    }
}

fn main() {
    let content = get_input_content();

    let mut head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0 };
    let mut set = HashSet::new();
    let copy = tail.clone();
    set.insert(copy);

    for line in content.lines() {
        let mut parts = line.split_whitespace();
        let head_direction = parts.next().unwrap();
        let num_of_moves: usize = parts.next().unwrap().parse().unwrap();

        for _ in 0..num_of_moves {
            head.move_in_direction(head_direction);

            let tail_move = get_tail_move_direction(&head, &tail);

            match tail_move {
                Some(direction) => tail.move_in_direction(&direction),
                _ => {}
            }

            let copy = tail.clone();
            set.insert(copy);
        }
    }

    let res = set.len();
    println!("The tail visited {res} different positions");
}
