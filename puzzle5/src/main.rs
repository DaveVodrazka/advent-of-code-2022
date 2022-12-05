extern crate regex;
use regex::Regex;
use std::fs;

struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

type Cargo = [Vec<char>; 9];

fn get_input_content() -> String {
    let file_path = "./puzzle5/input.txt";
    let contents = fs::read_to_string(file_path).expect("Failed to read the file");
    contents
}

fn string_to_num(v: &str) -> usize {
    match String::from(v).parse::<usize>() {
        Ok(v) => return v,
        Err(e) => {
            panic!("{e}");
        }
    }
}

fn parse_move(s: &str) -> Move {
    // example: "move 7 from 4 to 3"
    let re = Regex::new(r"move (\d*) from (\d*) to (\d*)").unwrap();
    let captures = re.captures(s).unwrap();

    assert!(captures.len() == 4, "Must capture 3 groups");

    Move {
        amount: string_to_num(&captures[1]),
        from: string_to_num(&captures[2]) - 1, // col 1 is indexed at 0
        to: string_to_num(&captures[3]) - 1,   // col 1 is indexed at 0
    }
}

fn transfer(cargo: &mut Cargo, m: Move) {
    let l = cargo[m.from].len();
    let temp: Vec<char> = cargo[m.from].drain(l - m.amount..).collect();
    cargo[m.to].extend(temp);
}

// Initial state:
//
// [V]     [B]                     [F]
// [N] [Q] [W]                 [R] [B]
// [F] [D] [S]     [B]         [L] [P]
// [S] [J] [C]     [F] [C]     [D] [G]
// [M] [M] [H] [L] [P] [N]     [P] [V]
// [P] [L] [D] [C] [T] [Q] [R] [S] [J]
// [H] [R] [Q] [S] [V] [R] [V] [Z] [S]
// [J] [S] [N] [R] [M] [T] [G] [C] [D]
//  1   2   3   4   5   6   7   8   9

fn main() {
    let content = get_input_content();
    let mut cargo: Cargo = [
        vec!['J', 'H', 'P', 'M', 'S', 'F', 'N', 'V'],
        vec!['S', 'R', 'L', 'M', 'J', 'D', 'Q'],
        vec!['N', 'Q', 'D', 'H', 'C', 'S', 'W', 'B'],
        vec!['R', 'S', 'C', 'L'],
        vec!['M', 'V', 'T', 'P', 'F', 'B'],
        vec!['T', 'R', 'Q', 'N', 'C'],
        vec!['G', 'V', 'R'],
        vec!['C', 'Z', 'S', 'P', 'D', 'L', 'R'],
        vec!['D', 'S', 'J', 'V', 'G', 'P', 'B', 'F'],
    ];

    for line in content.lines() {
        transfer(&mut cargo, parse_move(line));
    }

    let ans: String = cargo.iter().map(|v| v.last().unwrap()).collect();

    println!("Answer is: {ans}");
}
