use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use clap::builder::TypedValueParser;

#[derive(Parser)]
struct Args {
    path: String,
}

fn get_op_and_args(line: &str) -> (&str, i16) {
    let s: Vec<&str> = line.split(' ').collect();
    match s.len() {
        2 => {
            let arg:i16 = s[1].parse().unwrap();
            return ("addx", arg);
        },
        1 => ("noop", 0),
        _ => ("ERR", 0),
    }
}

fn main() {
    println!("Hello, world!");
    let args = Args::parse();
    let path = args.path;
    let mut lines = read_lines(path).unwrap();
    let mut cycle = 0;
    let mut x = 1i16;
    let checkpoints:Vec<i16> = vec![20, 60, 100, 140, 180, 220];
    let mut buffer = String::new();
    // let mut signal_strengths= Vec::new();
    for line in lines.map_while(Result::ok) {
        let (op, arg) = get_op_and_args(&line);
        let sprite:Vec<i16> = [x-1, x, x+1].to_vec();
        match op {
            "addx" => {
                if sprite.contains(&cycle) {
                    buffer.push('#');
                } else {
                    buffer.push('.');
                }

                // if checkpoints.contains(&cycle) {
                //     signal_strengths.push(cycle * x);
                // }
                cycle += 1;
                if (cycle == 40) {
                    cycle = 0;
                    // x = 1;
                }
                if sprite.contains(&cycle) {
                    buffer.push('#');
                } else {
                    buffer.push('.');
                }
                cycle += 1;
                if (cycle == 40) {
                    cycle = 0;
                    // x = 1;
                }
                // if checkpoints.contains(&cycle) {
                //     signal_strengths.push(cycle * x);
                // }
                x += arg;
            }
            "noop" => {

                if sprite.contains(&cycle) {
                    buffer.push('#');
                } else {
                    buffer.push('.');
                }
                cycle += 1;
                if (cycle == 40) {
                    cycle = 0;
                    // x = 1;
                }
                // if checkpoints.contains(&cycle) {
                //     signal_strengths.push(cycle * x);
                // }
            }
            _ => continue,
        }
        // println!("cycle: {}, sprite : {:?}, x: {}, line: {:?}", cycle, sprite, x, buffer);
        // if (cycle == 40) {
        //     cycle = 0;
        //     // x = 1;
        // }
        // println!("State after executing {} {}", op, arg);
        // println!("x={} cycle={}", x, cycle);
    }
    // for line in buffer.chunks(40)
    //         .map(|c| c.iter().collect::<String>())
    //         .collect::<Vec<String>>(){
    //     println!("{}", line);
    // }
    // println!("{}", buffer);
    let mut final_vector_value:Vec<String> = Vec::new();
    while(buffer.len() > 40) {
        let cutoff = buffer.split_off(40);
        final_vector_value.push(buffer);
        buffer = cutoff;
    }
    for line in final_vector_value {
        println!("{}", line);
    }
    println!("{}", buffer);
    // println!("Signal strengths={:?}", signal_strengths);
    // println!("Total signal strengths={:?}", signal_strengths.iter().sum::<i16>());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
