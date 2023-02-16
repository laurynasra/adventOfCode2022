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
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(value) = line {
                let line_lenght = value.len();
                let half_size = line_lenght / 2;
                let split = value.split_at(half_size);
                println!("{} has {} {} parts", value, split.0, split.1);
                for c in split.0.chars() {
                    println!("Processsing {} of {}", c, split.0);
                    if split.1.contains(c) {
                        let score = alphabet.iter().position(|&l| l == c).unwrap() + 1;
                        //println!("Duplicate {} in {}", c, value);
                        println!(
                            "value of duplicate {} is {}, sum: {}",
                            c,
                            alphabet.iter().position(|&l| l == c).unwrap() + 1,
                            sum
                        );
                        sum += score;
                        continue;
                    }
                }
            }
        }
        println!("Score: {}", sum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
