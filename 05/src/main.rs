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
    let mut map: HashMap<usize, Vec<String>> = HashMap::new();
    let box_letter_regex = Regex::new(r"(\w)").unwrap();
    let mut lines = read_lines(path).unwrap();
    for line in lines.by_ref().flatten() {
        // Detects ending of the intial boxes configuration by finding " 1" string
        if line.starts_with(" 1") {
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
            let m = box_letter_regex.captures(split).unwrap();
            let vec = match map.get_mut(&pos) {
                Some(v) => v,
                None => {
                    let v: Vec<String> = Vec::new();
                    map.insert(pos, v);
                    map.get_mut(&pos).unwrap()
                }
            };
            vec.push(m[0].to_string());
        }
    }
    let vec = Some(map).unwrap();
    println!("{:?}", vec.get(&0_usize).unwrap());
    let direction_regex = Regex::new(r"(\d)").unwrap();
    for line in lines.flatten() {
        println!("{:?}", line.to_string());
        if line.eq("") {
            continue;
        }
        let ds = direction_regex.captures(line.as_str()).unwrap();
        println!("{:?}", ds);
        break;
    }
    //  start moving box
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
