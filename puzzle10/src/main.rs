use std::fs;

struct CRT {
    display: [[char; 40]; 6],
}

impl CRT {
    fn new() -> CRT {
        CRT {
            display: [['.'; 40]; 6],
        }
    }
    fn draw(&mut self, reg: &i32, cycle: &i32) {
        let index: usize = *cycle as usize - 1;
        let x: usize = index % 40;

        if (reg - x as i32).abs() > 1 {
            // did not match the "sprite"
            return;
        }

        let row: usize = index / 40;
        self.display[row][x] = '#';
    }
    fn show(&self) {
        println!();
        for row in self.display {
            for c in row {
                // print space instead of . for better readibility
                if c == '.' {
                    print!(" ");
                } else {
                    print!("#");
                }
            }
            println!("");
        }
        println!();
    }
}

fn get_input_content() -> String {
    let file_path = "./puzzle10/input.txt";
    let contents = fs::read_to_string(file_path).expect("Failed to read the file");
    contents
}

fn handle_cycle(counter: &i32, register: &i32, crt: &mut CRT) -> Option<i32> {
    crt.draw(register, counter);
    let loggable_cycles = vec![20, 60, 100, 140, 180, 220];
    if !loggable_cycles.contains(counter) {
        return None;
    }

    let signal = counter * register;
    Some(signal)
}

fn main() {
    let content = get_input_content();

    let mut register: i32 = 1;
    let mut cycle_counter: i32 = 1;
    let mut sum = 0;
    let mut crt = CRT::new();

    for line in content.lines() {
        let mut parts = line.split_whitespace();
        let operation = parts.next().unwrap();

        if let Some(signal) = handle_cycle(&cycle_counter, &register, &mut crt) {
            sum += signal;
        }

        cycle_counter += 1;

        if operation == "addx" {
            if let Some(signal) = handle_cycle(&cycle_counter, &register, &mut crt) {
                sum += signal;
            }

            register += parts
                .next()
                .unwrap()
                .parse::<i32>()
                .expect("Failed to parse");

            cycle_counter += 1;
        };
    }

    println!("Sum of loggable signals is {sum}");
    crt.show();
}
