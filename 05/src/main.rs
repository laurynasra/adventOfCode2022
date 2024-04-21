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
    let mut map: HashMap<i32, Vec<char>> = HashMap::new();
    let re = Regex::new(r"(\w)").unwrap();
    if let Ok(lines) = read_lines(path) {
        for line in lines.into_iter().flatten() {
            //if let Ok(value) = line {
            if line.chars().nth(0).unwrap() == ' ' && line.chars().nth(1).unwrap() == '1' {
                break;
            }
            println!("{}", line);
            let line_split = line
                .chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|c| c.iter().collect::<String>())
                .collect::<Vec<String>>();

            for (pos, split) in line_split.iter().enumerate() {
                if split == "    " {
                    continue;
                }

                let Some(_) = re.captures(split) else {
                    println!("no match!");
                    return;
                };

                println!("'{}' at {}", split.to_string(), pos);
                if !map.contains_key(&(pos as i32)) {
                    let vec: Vec<char> = Vec::new();
                    map.insert(pos as i32, vec);
                }

                let _stash = map.get(&1).unwrap();
                //stash.push('1'.to_string().chars().nth(0).unwrap());
                // if let Ok(stash) = map.get(&(pos as i32)) {
                // stash.push(split.to_string().chars().nth(0).unwrap());
                // }
            }

            // let re = Regex::new(r"(...)\s?").unwrap();
            // for caps in re.captures_iter(&value) {
            //     let index = caps
            //         .iter()
            //         .enumerate()
            //         .find(|t| t.1.is_some()) // find the first `Some`
            //         .map(|t| t.0) // extract the index
            //         .unwrap_or(0);
            //     let cap = caps.get(1).unwrap().as_str().to_string();
            //     // skip empty capture
            //     let box_capture = box_re.captures(&cap);
            //     let letter = if let Some(x) = box_capture {
            //         x
            //     } else {
            //         continue;
            //     };
            //     println!("{} {}", letter.get(1).unwrap().as_str().to_string(), index);
            // }
            //}
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
