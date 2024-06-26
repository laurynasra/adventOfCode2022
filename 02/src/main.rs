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

static GAME_END_SCORES: phf::Map<&str, i32> = phf_map! {
    "DRAW" => 3,
    "WIN" => 6,
    "LOSE" => 0,
};

fn main() {
    let args = Args::parse();
    let path = args.path;

    if let Ok(lines) = read_lines(path) {
        let mut my_score = 0;
        let mut my_score2 = 0;
        for line in lines.into_iter().flatten() {
            let v: Vec<String> = line
                .trim()
                .split(' ')
                .map(|val| val.parse().unwrap())
                .collect();
            println!("pairs: {}, {}", v[0], v[1]);
            let first_hand: String = v[0].to_string();
            let second_hand: String = v[1].to_string();
            // lets handle draw first
            match second_hand.as_str() {
                "X" => {
                    // Lose
                    match first_hand.as_str() {
                        "A" => {
                            // if Rock (A) then we need to show Scissor (Z) in order to lose
                            my_score2 += HAND_SCORES.get("Z").unwrap();
                        }
                        "B" => {
                            // if Paper (B) then we need to show Rock (X) in order to lose
                            my_score2 += HAND_SCORES.get("X").unwrap();
                        }
                        "C" => {
                            // if Scissor (C) then we need to show Paper (Y) in order to lose
                            my_score2 += HAND_SCORES.get("Y").unwrap();
                        }
                        _ => {}
                    }
                    // Lose give 0 points, no need to add anything else
                }
                "Y" => {
                    // Draw
                    my_score2 += GAME_END_SCORES.get("DRAW").unwrap();
                    my_score2 += HAND_SCORES.get(&first_hand.to_string()).unwrap();
                }
                "Z" => {
                    // Win
                    match first_hand.as_str() {
                        "A" => {
                            // need Paper (Y) to beat Rock (A)
                            my_score2 += HAND_SCORES.get("Y").unwrap();
                        }
                        "B" => {
                            // need Scissor (Z) to beat Paper (B)
                            my_score2 += HAND_SCORES.get("Z").unwrap();
                        }
                        "C" => {
                            // need Rock (X) to beat Scissor (C)
                            my_score2 += HAND_SCORES.get("X").unwrap();
                        }
                        _ => {}
                    }
                    my_score2 += GAME_END_SCORES.get("WIN").unwrap();
                }
                _ => {}
            }

            if HAND_SCORES.get(&first_hand.to_string()).unwrap()
                == HAND_SCORES.get(&second_hand).unwrap()
            {
                my_score += 3; // draw gives me 3 points
                               // add my hand's score
                my_score += HAND_SCORES.get(&(second_hand.to_string())).unwrap();
                continue;
            }

            match first_hand.as_str() {
                "A" => {
                    if second_hand.as_str().eq("Y") {
                        my_score += 6;
                    }
                }
                "B" => {
                    if second_hand.as_str().eq("Z") {
                        my_score += 6;
                    }
                }
                "C" => {
                    if second_hand.as_str().eq("X") {
                        my_score += 6;
                    }
                }
                _ => {
                    println!("something else");
                }
            }
            my_score += HAND_SCORES.get(&second_hand.to_string()).unwrap();
        }
        println!("My score: {}", my_score);
        println!("My 2nd score: {}", my_score2);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
