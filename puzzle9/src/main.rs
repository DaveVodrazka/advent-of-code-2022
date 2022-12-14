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

    println!("({x_dif}, {y_dif})");

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
        (2, 2) => Some("UR".to_string()),
        (-2, 2) => Some("UL".to_string()),
        (2, -2) => Some("DR".to_string()),
        (-2, -2) => Some("DL".to_string()),
        _ => panic!("Unexpected combination"),
    }
}

fn main() {
    let content = get_input_content();

    let mut knots: [Position; 10] = [Position { x: 0, y: 0 }; 10];

    let mut first_knot_moves = HashSet::new();
    let mut last_knot_moves = HashSet::new();

    first_knot_moves.insert(knots[1].clone());
    last_knot_moves.insert(knots[9].clone());

    for line in content.lines() {
        let mut parts = line.split_whitespace();
        let head_direction = parts.next().unwrap();
        let num_of_moves: usize = parts.next().unwrap().parse().unwrap();

        for _ in 0..num_of_moves {
            // move the head
            knots[0].move_in_direction(head_direction);

            // itterate through all the knots and move them
            for i in 0..9 {
                let tail_move = get_tail_move_direction(&knots[i], &knots[i + 1]);

                match tail_move {
                    Some(direction) => knots[i + 1].move_in_direction(&direction),
                    _ => {}
                }
            }

            // store the new positions of the first and last knot into sets
            first_knot_moves.insert(knots[1].clone());
            last_knot_moves.insert(knots[9].clone());
        }
    }

    let part_one = first_knot_moves.len();
    let part_two = last_knot_moves.len();

    println!("The tail visited {part_one} different positions");
    println!("The last knot visited {part_two} different positions");
}
