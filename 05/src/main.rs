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
            let adjusted_pos: usize = pos + 1;
            let vec = match map.get_mut(&adjusted_pos) {
                Some(v) => v,
                None => {
                    let v: Vec<String> = Vec::new();
                    map.insert(adjusted_pos, v);
                    map.get_mut(&adjusted_pos).unwrap()
                }
            };
            vec.insert(0, m[0].to_string());
        }
    }

    let direction_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in lines.skip(1).flatten() {
        if line.eq("") {
            continue;
        }
        for caps in direction_regex.captures_iter(line.as_str()) {
            let mut count: i32 = caps.get(1).unwrap().as_str().to_string().parse().unwrap();
            let from: usize = caps.get(2).unwrap().as_str().parse().unwrap();
            let to: usize = caps.get(3).unwrap().as_str().parse().unwrap();
            /* part 1
            while count > 0 {
                let popped = map.get_mut(&from).unwrap().pop();
                map.get_mut(&to).unwrap().push(popped.unwrap().to_string());
                count = count - 1;
            }
            */
            // part 2
            let mut new_stack: Vec<String> = Vec::new();
            while count > 0 {
                let popped = map.get_mut(&from).unwrap().pop().unwrap();
                new_stack.push(popped);
                count -= 1;
            }
            new_stack.reverse();
            map.get_mut(&to).unwrap().append(&mut new_stack);
        }
    }
    for i in 1..=map.values().count() {
        print!("{:?}", map.get(&i).unwrap().last().unwrap());
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
