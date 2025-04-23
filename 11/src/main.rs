use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use clap::Parser;

#[derive(Parser)]
struct Args {
    path: String,
}

fn main() {
    let args = Args::parse();
    let path = args.path;
    let lines = read_lines(path).unwrap();
    for line in lines.map_while(Result::ok) {
        println!("{}", line);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}