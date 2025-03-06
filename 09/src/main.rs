use clap::Parser;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Parser, Debug)]
struct Args {
    path: String,
}
#[derive(Debug, Clone)]
struct Coords {
    x: i16,
    y: i16,
}

#[derive(Debug, Clone)]
struct Rope {
    knots: Vec<Coords>,
}

impl Rope {
    fn move_head(&mut self, direction: &str) {
        let mut head = self.knots.first_mut().unwrap();
        move_head(&mut head, direction);
    }

    fn move_tails(&mut self) {
        for pos in 0..=9 {
            if pos == 9 {
                //skip last knot
                continue;
            }

            //println!("moving {} and {}", pos, pos + 1);

            let mid_head = self.knots.get(pos).unwrap().clone();
            let mut mid_tail = self.knots.get_mut(pos + 1).unwrap();
            move_tail(&mut mid_tail, &mid_head);
        }
    }
}

impl Coords {
    fn is_touching(&mut self, other: &Coords) -> bool {
        let x_diff = self.x.abs_diff(other.x);
        let y_diff = self.y.abs_diff(other.y);

        if x_diff < 2 && y_diff < 2 {
            return true;
        }

        return false;
    }

    fn print(&self) -> String {
        let formatted = format!("{}x{}", self.x, self.y);
        return formatted;
    }
}

fn main() {
    let args = Args::parse();
    let path = args.path;
    let mut lines = read_lines(path).unwrap();
    let directions_parser_r = Regex::new(r"([A-Z]) (\d+)").unwrap();
    let mut head = Coords { x: 0, y: 0 };
    let mut tail = Coords { x: 0, y: 0 };
    let mut vec: Vec<String> = Vec::new();
    let mut vec2: Vec<String> = Vec::new();
    let mut rope = Rope {
        knots: vec![Coords { x: 0, y: 0 }; 10],
    };
    for line in lines.map_while(Result::ok) {
        println!("{}", line);
        let c = directions_parser_r.captures(line.as_str()).unwrap();
        let direction: String = c[1].to_string();
        let steps: u8 = c[2].parse().expect("Failed to prase steps");
        for step in 0..steps {
            rope.move_head(direction.as_str());
            rope.move_tails();
            //println!("Rope after move {:?}", rope);
            let rope_tail = rope.knots.last().unwrap();
            let xy2 = format!("{}x{}", rope_tail.x, rope_tail.y);
            println!("rope tail pos {:?}, storing {}", rope_tail, xy2);
            vec2.push(xy2);
            move_head(&mut head, direction.as_str());
            /*println!(
                "Head and tail after move: {} {}",
                head.print(),
                tail.print()
            );*/
            let x = tail.x.to_string();
            let y = tail.y.to_string();
            let xy = format!("{}x{}", tail.x, tail.y);
            //println!("xy: {}", xy);
            vec.push(xy);
        }
    }
    let unique_vec: Vec<String> = vec
        .clone()
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    let unique_vec2: Vec<String> = vec2
        .clone()
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    //println!("final: {:?}", unique_vec);
    print!("length non uniq: {}", vec.len());
    println!("length: {}", unique_vec.len());
    println!("part2 length: {}", unique_vec2.len());
}

fn move_tail(tail: &mut Coords, head: &Coords) {
    if tail.is_touching(head) {
        return;
    }
    let x_diff = tail.x - head.x;
    let y_diff = tail.y - head.y;
    if x_diff == 0 && y_diff == 0 {
        return;
    }
    // non diagonal
    if x_diff == 0 || y_diff == 0 {
        if x_diff == 0 {
            if tail.y < head.y {
                tail.y += 1;
            } else {
                tail.y -= 1;
            }
        }
        if y_diff == 0 {
            if tail.x < head.x {
                tail.x += 1;
            } else {
                tail.x -= 1;
            }
        }
        return;
    }
    //diagonal
    let mut move_x: i16 = 0;
    let mut move_y: i16 = 0;

    if tail.x < head.x && tail.y < head.y {
        move_x = 1;
        move_y = 1;
    } else if tail.x > head.x && tail.y < head.y {
        move_x = -1;
        move_y = 1;
    } else if tail.x > head.x && tail.y > head.y {
        move_x = -1;
        move_y = -1;
    } else if tail.x < head.x && tail.y > head.y {
        move_x = 1;
        move_y = -1;
    }

    tail.x += move_x;
    tail.y += move_y;
}

fn move_head(head: &mut Coords, direction: &str) {
    match direction {
        "U" => head.y += 1,
        "R" => head.x += 1,
        "D" => head.y -= 1,
        "L" => head.x -= 1,
        _ => println!("Unrecognised direction."),
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
