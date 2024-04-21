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
    let box_re = Regex::new(r"\[(.)\]").unwrap();
    if let Ok(lines) = read_lines(path) {
        for line in lines.into_iter().flatten() {
            println!("{}", line);
            let re = Regex::new(r"(...)\s?").unwrap();
            for caps in re.captures_iter(&line) {
                let index = caps
                    .iter()
                    .enumerate()
                    .find(|t| t.1.is_some()) // find the first `Some`
                    .map(|t| t.0) // extract the index
                    .unwrap_or(0);
                let cap = caps.get(1).unwrap().as_str().to_string();
                // skip empty capture
                let box_capture = box_re.captures(&cap);
                let letter = if let Some(x) = box_capture {
                    x
                } else {
                    continue;
                };
                println!("{} {}", letter.get(1).unwrap().as_str(), index);
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
