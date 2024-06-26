use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Parser, Debug)]
struct Params {
    path: String,
}

fn main() {
    let args = Params::parse();
    let path = args.path;
    if let Ok(lines) = read_lines(path) {
        let mut sums: Vec<u32> = vec![];
        let mut sum: u32 = 0;
        for line in lines.into_iter().flatten() {
            if line.is_empty() {
                sums.push(sum);
                sum = 0;
                continue;
            }
            let parsed_value = line.parse::<u32>().unwrap();
            sum += parsed_value;
        }
        sums.sort();
        sums.reverse();
        let max_value = *sums.iter().max().unwrap();
        let top3_vec: Vec<u32> = vec![sums[0], sums[1], sums[2]];
        let top_3_sum: u32 = top3_vec.iter().sum();
        println!("max: {}", max_value);
        println!("sum top 3: {}", top_3_sum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
