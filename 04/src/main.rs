use clap::Parser;
use regex::Regex;
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
        let mut count = 0;
        let mut partial_count = 0;
        for line in lines {
            if let Ok(value) = line {
                let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
                for caps in re.captures_iter(&value) {
                    let num1_str: String = caps.get(1).unwrap().as_str().to_string();
                    let num1: i32 = num1_str.parse().unwrap();

                    let num2_str: String = caps.get(2).unwrap().as_str().to_string();
                    let num2: i32 = num2_str.parse().unwrap();

                    let num3_str: String = caps.get(3).unwrap().as_str().to_string();
                    let num3: i32 = num3_str.parse().unwrap();

                    let num4_str: String = caps.get(4).unwrap().as_str().to_string();
                    let num4: i32 = num4_str.parse().unwrap();

                    let vec1: Vec<_> = (num1..=num2).collect();
                    let vec2: Vec<_> = (num3..=num4).collect();

                    if vec1.iter().all(|n| vec2.contains(n))
                        || vec2.iter().all(|n| vec1.contains(n))
                    {
                        count += 1;
                    }

                    if vec1.iter().any(|n| vec2.contains(n)) {
                        partial_count += 1;
                    }
                }
            }
        }
        println!("Total overlap count: {}", count);
        println!("Partial overlap count: {}", partial_count);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
