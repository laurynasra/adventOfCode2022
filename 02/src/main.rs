use clap::Parser;
use phf::phf_map;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Parser, Debug)]
struct Args {
    path: String,
}

static HAND_SCORES: phf::Map<&str, i32> = phf_map! {
    "X" => 1, // Rock
    "A" => 1,
    "Y" => 2, // Paper
    "B" => 2,
    "Z" => 3, // Scissors
    "C" => 3,
};

fn main() {
    let args = Args::parse();
    let path = args.path;
    if let Ok(lines) = read_lines(path) {
        let mut my_score = 0;
        for line in lines {
            if let Ok(value) = line {
                let v: Vec<String> = value
                    .trim()
                    .split(" ")
                    .map(|val| val.parse().unwrap())
                    .collect();
                println!("pairs: {}, {}", v[0], v[1]);
                let first_hand: String = v[0].to_string();
                let second_hand: String = v[1].to_string();
                // lets handle draw first
                if HAND_SCORES.get(&first_hand.to_string()).unwrap()
                    == HAND_SCORES.get(&second_hand).unwrap()
                {
                    my_score += 3; // draw gives me 3 points
                                   // add my hand's score
                    my_score += HAND_SCORES.get(&(second_hand.to_string())).unwrap();
                }
            }
        }
        println!("My score: {}", my_score);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
