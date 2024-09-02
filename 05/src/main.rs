use clap::Parser;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Parser, Debug)]
struct Args {
    path: String,
}

fn main() {
    let args = Args::parse();
    let path = args.path;
    let mut map: HashMap<i32, Vec<char>> = HashMap::new();
    let re = Regex::new(r"(\w)").unwrap();
    if let Ok(lines) = read_lines(path) {
        for line in lines.into_iter().flatten() {
            // Detects ending of the intial boxes configuration
            if line.starts_with(' ') && line.chars().nth(1).unwrap() == '1' {
                break;
            }
            let line_split = line
                .chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|c| c.iter().collect::<String>())
                .collect::<Vec<String>>();

            for (pos, split) in line_split.iter().enumerate() {
                // Skip the empty spot in the boxes stash
                if split == "    " {
                    continue;
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
