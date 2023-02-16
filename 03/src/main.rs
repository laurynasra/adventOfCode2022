use clap::Parser;
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
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();
    let mut sum = 0;
    let mut sum2 = 0;
    let mut groups: Vec<String> = vec![];
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(value) = line {
                groups.push(value.clone());
                if groups.len() == 3 {
                    let mut uniq_group: Vec<char> = groups[0].chars().collect::<Vec<_>>();
                    uniq_group.sort_unstable();
                    uniq_group.dedup();
                    for c2 in uniq_group {
                        if groups[1].contains(c2) && groups[2].contains(c2) {
                            let score = alphabet.iter().position(|&l| l == c2).unwrap() + 1;
                            sum2 += score;
                        }
                    }
                    groups.clear();
                }
                let line_lenght = value.len();
                let half_size = line_lenght / 2;
                let split = value.split_at(half_size);
                let mut uniq_split: Vec<char> = split.0.chars().collect::<Vec<_>>();
                uniq_split.sort_unstable();
                uniq_split.dedup();
                for c in uniq_split {
                    if split.1.contains(c) {
                        let score = alphabet.iter().position(|&l| l == c).unwrap() + 1;
                        sum += score;
                        continue;
                    }
                }
            }
        }
        println!("Score: {}, Score 2: {}", sum, sum2);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
