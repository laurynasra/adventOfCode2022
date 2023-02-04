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
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(value) = line {
                let v: Vec<String> = value
                    .trim()
                    .split(" ")
                    .map(|val| val.parse().unwrap())
                    .collect();
                println!("pairs: {}, {}", v[0], v[1]);
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
