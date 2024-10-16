use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};
use clap::Parser; 

#[derive(Parser, Debug)]
struct Args {
    path: String,
}

fn main() {
    let args = Args::parse();
    let path = args.path;

    let lines = read_lines(path);
    for line in lines.unwrap().by_ref() {
        println!("{:?}", line);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

