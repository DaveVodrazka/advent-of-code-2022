use std::fs;

fn main() {
    let file_path = "./puzzle1/input.txt";
    let contents = fs::read_to_string(file_path).expect("Failed to read the file");
    let lines = contents.split("\n");

    const AMOUNT: usize = 3;
    let mut maximums: [u32; AMOUNT] = [0; AMOUNT];
    let mut current: u32 = 0;

    for line in lines {
        if line == "" {
            let lowest_max = maximums[0];
            if current > lowest_max {
                println!("Updating maximum from {lowest_max} to {current}");

                // by sorting, lowest maximum is always at 0 position
                maximums[0] = current;
                maximums.sort();
            }
            current = 0;
            continue;
        }
        let num: u32 = line.trim().parse().expect("Failed to parse calories!");
        current += num;
    }

    let sum: u32 = maximums.iter().sum();
    println!("Maximums are {:?} and sum is {}", maximums, sum);
}
