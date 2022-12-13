use std::fs;

type Grid = Vec<Vec<usize>>;

fn get_input_content() -> String {
    let file_path = "./puzzle8/test_input.txt";
    let contents = fs::read_to_string(file_path).expect("Failed to read the file");
    contents
}

fn is_visible(grid: &Grid, x: usize, y: usize) -> (bool, usize) {
    let row = &grid[y];
    let col: Vec<&usize> = grid.iter().map(|s| s.iter().nth(x).unwrap()).collect();
    let current_height = row[x];
    let mut scenic_score_arr: [usize; 4] = [0; 4];
    let mut definitelly_visible = false;
    let mut visible = true;

    // is left or right edge
    if x == (row.len() - 1) || x == 0 {
        definitelly_visible = true;
    }
    // is top or bottom edge
    if y == (col.len() - 1) || y == 0 {
        definitelly_visible = true;
    }

    for i in 0..x {
        scenic_score_arr[0] += 1;
        if row[i] >= current_height {
            visible = false;
            break;
        }
    }

    if visible {
        definitelly_visible = true;
    }

    visible = true;

    for i in x + 1..row.len() {
        scenic_score_arr[1] += 1;
        if row[i] >= current_height {
            visible = false;
            break;
        }
    }

    if visible {
        definitelly_visible = true;
    }

    visible = true;

    for i in 0..y {
        scenic_score_arr[2] += 1;
        if col[i] >= &current_height {
            visible = false;
            break;
        }
    }

    if visible {
        definitelly_visible = true;
    }

    visible = true;

    for i in y + 1..col.len() {
        scenic_score_arr[3] += 1;
        if col[i] >= &current_height {
            visible = false;
            break;
        }
    }

    if visible {
        definitelly_visible = true;
    }

    let scenic_score = scenic_score_arr.iter().product();
    println!("scenic arr {scenic_score_arr:?}, {scenic_score}");

    (definitelly_visible, scenic_score)
}

fn main() {
    let content = get_input_content();

    // declare grid
    let mut grid: Grid = vec![];

    // fill the grid from input
    for line in content.lines() {
        let nums: Vec<usize> = line
            .chars()
            .into_iter()
            .map(|c| match c.to_digit(10) {
                Some(n) => usize::try_from(n).unwrap(),
                None => panic!("Failed to parse"),
            })
            .collect();
        grid.push(nums);
    }

    let mut count: u32 = 0;
    let mut scenic_max: usize = 0;

    for (y, line) in grid.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            let (visible, scenic_score) = is_visible(&grid, x, y);
            if visible {
                count += 1;
            }
            if scenic_score > scenic_max {
                scenic_max = scenic_score
            }
        }
    }

    println!("There are {count} visible trees");
    println!("Maximum scenic score is {scenic_max}");
}
