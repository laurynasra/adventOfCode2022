use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args = env::args();
    let path = args[0];
    if let Ok(lines) = read_lines("./data.txt") {
        let mut sums: Vec<u32> = vec![];
        let mut sum: u32 = 0;
        for line in lines {
            if let Ok(value) = line {
                println!("{}", value);
                if value == "" {
                    println!("sum: {}", sum);
                    sums.push(sum);
                    sum = 0;
                    continue;
                }
                let parsed_value = value.parse::<u32>().unwrap();
                sum = sum + parsed_value;
            }
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
