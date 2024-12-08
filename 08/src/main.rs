use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    path: String,
}

fn main() {
    let args = Args::parse();
    let path = args.path;
    let mut lines = read_lines(path).unwrap();
    let mut i = 0;
    let mut tree_matrix: Vec<Vec<u16>> = Vec::new();
    for line in lines.by_ref().flatten() {
        let heights_in_line = line.chars()
            .map(|c| (c as u16) - ('0' as u16))
            .collect();

        // println!("adding vector {:?}", heights_in_line);
        tree_matrix.insert(i, heights_in_line);
        i += 1;
    }
    let mut total_visible_count = 0;
    for x in 0..tree_matrix.len() {
        for y in 0..tree_matrix[x].len() {
            if is_visible(x, y, tree_matrix.clone()) == true {
                total_visible_count += 1;
            }
        }
    }

    println!("total visible_count: {}", total_visible_count);

    // Part 2
}

fn is_visible(x: usize, y: usize, forest: Vec<Vec<u16>>) -> bool {
    is_visible_top(x, y, forest.clone()) ||
    is_visible_bottom(x, y, forest.clone()) ||
    is_visible_left(x, y, forest.clone()) ||
    is_visible_right(x, y, forest.clone())
}

fn is_visible_top(x: usize, y: usize, forest: Vec<Vec<u16>>) -> bool {
    let mut is_visible = true;
    let target_height = forest[x][y];
    for i in (0..x) {
        // println!("Comparing value at {}x{} ({}) vs {}", i, y, forest[i][y], target_height);
        if (forest[i][y] >= target_height) {
            is_visible = false;
        }
    }
    // println!("from top is visible: {:?}", is_visible);
    is_visible
}

fn is_visible_bottom(x: usize, y: usize, forest: Vec<Vec<u16>>) -> bool {
    let mut is_visible = true;
    let target_height = forest[x][y];
    for i in x+1..forest.len() {
        if (forest[i][y] >= target_height) {
            is_visible = false;
        }
    }
    // println!("from bottom is visible: {:?}", is_visible);
    is_visible
}

fn is_visible_left(x: usize, y: usize, forest: Vec<Vec<u16>>) -> bool {
    let mut is_visible = true;
    let target_height = forest[x][y];
    for i in (0..y).rev() {
        if (forest[x][i] >= target_height) {
            is_visible = false;
        }
    }
    is_visible
}

fn is_visible_right(x: usize, y: usize, forest: Vec<Vec<u16>>) -> bool {
    let mut is_visible = true;
    let target_height = forest[x][y];
    for i in y+1..forest[x].len() {
        if (forest[x][i] >= target_height) {
            is_visible = false;
        }
    }
    is_visible
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
