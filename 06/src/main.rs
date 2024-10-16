use clap::Parser;
use std::char;
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::collections::HashSet;
use itertools::Itertools;

#[derive(Parser, Debug)]
struct Args {
    path: String,
}

fn main() {
    let args = Args::parse();
    let path = args.path;
    let mut marker: HashSet<char> = HashSet::new();

    let line = read_lines(path)
        .unwrap()
        .flatten()
        .at_most_one()
        .unwrap().unwrap();

    println!("{:?}", line);

    for n in 13..line.chars().count() {
        let marker_str = &line[n - 13..=n];
        for c in marker_str.chars() {
            marker.insert(c);
        }
        if marker.len() == 14 {
            println!("First full marker at {:?}", n + 1);
            break;
        }
        marker.clear();
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

