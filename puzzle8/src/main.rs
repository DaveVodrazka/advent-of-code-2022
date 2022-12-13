use std::fs;

type Grid = Vec<Vec<usize>>;

fn get_input_content() -> String {
    let file_path = "./puzzle8/test_input.txt";
    let contents = fs::read_to_string(file_path).expect("Failed to read the file");
    contents
}

fn is_visible(grid: &Grid, x: usize, y: usize) -> bool {
    let row = &grid[y];
    let col = &grid
        .iter()
        .map(|s| s.iter().nth(x).unwrap())
        .collect::<Vec<_>>();
    let current_tree = row[x];
    let row_end = row.len() - 1;
    let col_end = col.len() - 1;

    // is left or right edge
    if x == row_end || x == 0 {
        return true;
    }
    // is top or bottom edge
    if y == col_end || y == 0 {
        return true;
    }

    for i in 0..x {
        if row[i] >= current_tree {
            break;
        }
        // did not break - tree is visible
        return true;
    }

    for i in (x + 1)..=row_end {
        if row[i] >= current_tree {
            break;
        }
        // did not break - tree is visible
        return true;
    }

    for i in 0..y {
        if col[i] >= &current_tree {
            break;
        }
        // did not break - tree is visible
        return true;
    }

    for i in (y + 1)..=col_end {
        if col[i] >= &current_tree {
            break;
        }
        // did not break - tree is visible
        return true;
    }

    false
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

    for (y, line) in grid.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            if is_visible(&grid, x, y) {
                count += 1;
            }
        }
    }

    println!("There are {count} visible trees");
}
